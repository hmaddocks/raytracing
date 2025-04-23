use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
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

    pub fn build(self) -> Camera {
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        let center = Point3::default();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        let view_port_u = Vec3::new(viewport_width, 0.0, 0.0);
        let view_port_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = &view_port_u / self.image_width as f64;
        let pixel_delta_v = &view_port_v / image_height as f64;
        let viewport_upper_left_vec = center.as_vec3()
            - Vec3::new(0.0, 0.0, focal_length)
            - (&view_port_u / 2.0)
            - (&view_port_v / 2.0);
        let viewport_upper_left: Point3 = viewport_upper_left_vec.into();
        let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

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
        }
    }
}

fn random_double() -> f64 {
    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(0.0..1.0) // Generate a random f64 in the range [0.0, 1.0)
}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}

impl Camera {
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = &self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            let direction = hit_record.normal + Vec3::random_unit();
            self.ray_color(&Ray::new(hit_record.p, direction), depth - 1, world) * 0.5
        } else {
            let unit_direction = ray.direction().unit();
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }

    pub fn render(&self, world: &HittableList) {
        // Process scanlines in parallel
        let image: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter() // Parallelize over scanlines
            .map(|j| {
                // Process each pixel in the current scanline
                let row: Vec<Color> = (0..self.image_width)
                    .into_par_iter() // Parallelize over pixels in the scanline
                    .map(|i| {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        // Sample each pixel multiple times for anti-aliasing
                        for _ in 0..self.samples_per_pixel {
                            let ray = self.get_ray(i, j);
                            pixel_color += self.ray_color(&ray, self.max_depth, world);
                        }

                        pixel_color *= self.pixel_samples_scale;
                        pixel_color
                    })
                    .collect();

                row
            })
            .collect();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        // Output all scanlines
        for scanline in image {
            for pixel in scanline {
                println!("{}", &pixel.write_color());
            }
        }

        eprintln!("\nDone.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hittable_list::HittableList;
    use crate::point3::Point3;
    use crate::ray::Ray;
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
            let v = sample_square();
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
        let camera = CameraBuilder::default().build();
        let ray = Ray::new(Point3::default(), Vec3::new(1.0, 0.0, 0.0));
        let world = HittableList::new();
        let color = camera.ray_color(&ray, 0, &world);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
}
