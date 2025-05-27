use crate::color::Color;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::random_double;
use crate::ray::Ray;
use crate::utilities::degrees_to_radians;
use crate::vec3::Vec3;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::f64;

// Constants for common values
const BLACK: Color = Color::new(0.0, 0.0, 0.0);
const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const SKY_BLUE: Color = Color::new(0.5, 0.7, 1.0);
const MIN_IMAGE_HEIGHT: u32 = 1;
const RAY_T_MIN: f64 = 0.001;

/// Camera for rendering a scene.
///
/// Handles ray generation and rendering of the scene to a PPM format.
#[derive(Debug, Clone)]
pub struct Camera {
    image_height: u32,
    image_width: u32,
    pixel_samples_scale: f64,
    samples_per_pixel: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: u32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

/// Builder for creating a customized camera.
///
/// Uses the builder pattern to configure camera parameters.
#[derive(Debug, Clone)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vertical_fov: f64,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default().build()
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth: 10,
            vertical_fov: 90.0,
            look_from: Point3::new(-2.0, 2.0, 1.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 1.0,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    /// Build the camera with the configured parameters.
    pub fn build(self) -> Camera {
        // Calculate image height based on aspect ratio, ensuring it's at least 1
        let image_height =
            ((self.image_width as f64 / self.aspect_ratio) as u32).max(MIN_IMAGE_HEIGHT);

        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        let center = self.look_from;

        // Calculate viewport dimensions
        let theta = degrees_to_radians(self.vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate camera basis vectors
        let w = (self.look_from - self.look_at).unit();
        let u = self.vup.cross(&w).unit();
        let v = w.cross(&u).unit();

        // Calculate viewport vectors
        let view_port_u = viewport_width * u;
        let view_port_v = viewport_height * -v;

        // Calculate pixel delta vectors
        let pixel_delta_u = view_port_u / self.image_width as f64;
        let pixel_delta_v = view_port_v / image_height as f64;

        // Calculate location of upper-left pixel
        let viewport_upper_left =
            center.as_vec3() - self.focus_dist * w - view_port_u / 2.0 - view_port_v / 2.0;
        let pixel00_loc =
            Point3::from(viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v);

        // Calculate defocus disk vectors
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle) / 2.0).tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera {
            image_height,
            image_width: self.image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    /// Generate a ray from the camera through the specified pixel.
    ///
    /// # Arguments
    ///
    /// * `i` - The x-coordinate of the pixel
    /// * `j` - The y-coordinate of the pixel
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Get a random offset within the pixel for anti-aliasing
        let offset = Vec3::sample_square();

        // Calculate the exact position on the viewport
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        // Determine ray origin (either camera center or point on defocus disk)
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            Point3::from(self.defocus_disk_sample())
        };

        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        Ray::new(ray_origin, ray_direction, ray_time)
    }

    /// Sample a point on the defocus disk for depth-of-field effect.
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center.as_vec3() + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    /// Calculate the color for a ray in the scene.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to trace
    /// * `depth` - The maximum recursion depth remaining
    /// * `world` - The scene to render
    fn ray_color(ray: &Ray, depth: u32, world: &dyn crate::hittable::Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth == 0 {
            return BLACK;
        }

        // Check if the ray hits anything in the world
        if let Some(hit_record) = world.hit(ray, Interval::new(RAY_T_MIN, f64::INFINITY)) {
            // If there's a material, calculate scattered ray
            if let Some(material) = &hit_record.material {
                let (attenuation, scatter) = material.scatter(ray, &hit_record);
                return Self::ray_color(&scatter, depth - 1, world) * attenuation;
            }
            return BLACK;
        }

        // Background - a simple gradient
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        WHITE * (1.0 - t) + SKY_BLUE * t
    }

    /// Render the scene to PPM format on stdout.
    ///
    /// # Arguments
    ///
    /// * `world` - The scene to render (any object implementing Hittable)
    pub fn render(&self, world: &dyn crate::hittable::Hittable) {
        // Create a progress bar for tracking scanlines
        let progress_bar = ProgressBar::new(self.image_height as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} scanlines ({eta})")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );

        // Process scanlines in parallel
        let image: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter() // Parallelize over scanlines
            .map(|j| {
                // Process each pixel in the current scanline
                let row: Vec<Color> = (0..self.image_width)
                    .into_par_iter() // Parallelize over pixels in the scanline
                    .map(|i| {
                        // Start with black
                        let mut pixel_color = BLACK;

                        // Sample each pixel multiple times for anti-aliasing
                        for _ in 0..self.samples_per_pixel {
                            let ray = self.get_ray(i, j);
                            pixel_color += Self::ray_color(&ray, self.max_depth, world);
                        }

                        // Scale the color by the number of samples
                        pixel_color * self.pixel_samples_scale
                    })
                    .collect();

                // Increment the progress bar for each completed scanline
                progress_bar.inc(1);
                row
            })
            .collect();

        // Finish the progress bar
        progress_bar.finish_with_message("Rendering complete");

        // Output PPM header
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        // Output all scanlines
        for scanline in image {
            for pixel in scanline {
                println!("{}", pixel.write_color());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bvh::Bvh;
    use crate::material::TestMaterial;
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::sphere::SphereBuilder;
    use crate::utilities::random_double;
    use crate::vec3::Vec3;

    #[test]
    fn test_camera_builder_defaults() {
        let camera = CameraBuilder::default().build();
        assert_eq!(camera.image_width, 100);
        assert_eq!(camera.image_height, 100); // aspect_ratio 1.0
        assert_eq!(camera.samples_per_pixel, 100);
        assert_eq!(camera.max_depth, 10);
    }

    #[test]
    fn test_camera_builder_custom() {
        let camera = CameraBuilder::new()
            .image_width(200)
            .samples_per_pixel(50)
            .max_depth(5)
            .build();
        assert_eq!(camera.image_width, 200);
        assert_eq!(camera.samples_per_pixel, 50);
        assert_eq!(camera.max_depth, 5);
    }

    #[test]
    fn test_random_double_range() {
        for _ in 0..100 {
            let v = random_double();
            assert!(v >= 0.0 && v < 1.0, "random_double out of range: {}", v);
        }
    }

    #[test]
    fn test_sample_square_range() {
        for _ in 0..100 {
            let v = Vec3::sample_square();
            assert!(v.x() >= -0.5 && v.x() < 0.5);
            assert!(v.y() >= -0.5 && v.y() < 0.5);
            assert_eq!(v.z(), 0.0);
        }
    }

    #[test]
    fn test_get_ray() {
        let camera = CameraBuilder::default().build();
        let ray = camera.get_ray(0, 0);
        // The ray's origin should be at the camera center
        assert_eq!(ray.origin(), &camera.center);
        // The direction should be normalized (or close to)
        let dir = ray.direction();
        let len = dir.length();
        assert!(len > 0.0);
    }

    #[test]
    fn test_ray_color_depth_zero() {
        let ray = Ray::new(Point3::default(), Vec3::new(1.0, 0.0, 0.0), 0.0);
        // Create a sphere that the ray will miss
        let sphere = SphereBuilder::new()
            .center(Point3::new(0.0, 0.0, -1.0))
            .radius(0.5)
            .material(TestMaterial::new())
            .build()
            .unwrap();
        let world = Bvh::new(vec![Box::new(sphere)]).unwrap();
        let color = Camera::ray_color(&ray, 0, &world as &dyn crate::hittable::Hittable);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
}
