use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;

fn random_double() -> f64 {
    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(0.0..1.0) // Generate a random f64 in the range [0.0, 1.0)
}

pub struct Camera {
    image_height: u32,
    image_width: u32,
    pixel_samples_scale: f64,
    samples_per_pixel: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let view_port_u = Vec3::new(viewport_width, 0.0, 0.0);
        let view_port_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = view_port_u / image_width as f64;
        let pixel_delta_v = view_port_v / image_height as f64;
        let viewport_upper_left_vec = center.as_vec3()
            - Vec3::new(0.0, 0.0, focal_length)
            - (view_port_u / 2.0)
            - (view_port_v / 2.0);
        let viewport_upper_left: Point3 = viewport_upper_left_vec.into();
        let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

        Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            samples_per_pixel,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
        if let Some(hit_record) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            (Color::new(1.0, 1.0, 1.0)
                + Color::new(
                    hit_record.normal.x(),
                    hit_record.normal.y(),
                    hit_record.normal.z(),
                ))
                * 0.5
        } else {
            let unit_direction = r.direction().unit();
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!(
                "\rScanlines remaining: {}             ",
                self.image_height - j
            );
            for i in 0..self.image_width {
                // let pixel_center: Point3 = self.pixel00_loc
                //     + self.pixel_delta_u * i as f64
                //     + self.pixel_delta_v * j as f64;
                // let ray_direction = pixel_center - self.center;
                // let r = Ray::new(self.center, ray_direction);

                // let pixel_color = self.ray_color(&r, &world);

                //  color pixel_color(0,0,0);
                //                 for (int sample = 0; sample < samples_per_pixel; sample++) {
                //                     ray r = get_ray(i, j);
                //                     pixel_color += ray_color(r, world);
                //                 }
                //                 write_color(std::cout, pixel_samples_scale * pixel_color);

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }

                pixel_color *= self.pixel_samples_scale;
                println!("{}", pixel_color.write_color());
            }
        }
    }
}
