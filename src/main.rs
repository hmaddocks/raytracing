use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point3::Point3;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

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

fn main() {
    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // Add spheres with materials
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // Camera
    let camera = camera::CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(50)
        .vertical_fov(60.0)
        .look_from(Point3::new(-2.0, 2.0, 1.0))
        .look_at(Point3::new(0.0, 0.0, -1.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(2.0)
        .focus_dist(3.4)
        .build();

    camera.render(&world);
    eprintln!("\nDone.");
}
