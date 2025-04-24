use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point3::Point3;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod point3;
mod ray;
mod sphere;
mod vec3;

fn random_double() -> f64 {
    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(0.0..1.0) // Generate a random f64 in the range [0.0, 1.0)
}
fn main() {
    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for i in -8..8 {
        for j in -8..8 {
            let choose_mat = random_double();
            let center = Point3::new(
                i as f64 + 0.9 * random_double(),
                0.2,
                j as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let material = Lambertian::new(Color::new(
                        random_double(),
                        random_double(),
                        random_double(),
                    ));
                    world.add(Box::new(Sphere::new(center, 0.2, material.clone())));
                } else if choose_mat < 0.95 {
                    let material = Metal::new(
                        Color::new(random_double(), random_double(), random_double()),
                        0.5,
                    );
                    world.add(Box::new(Sphere::new(center, 0.2, material.clone())));
                } else {
                    let material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, material.clone())));
                }
            }
        }
    }

    let material_1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));

    // Camera
    let camera = camera::CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(800)
        .samples_per_pixel(200)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(2.0)
        .focus_dist(10.0)
        .build();

    camera.render(&world);
    eprintln!("\nDone.");
}
