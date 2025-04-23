use crate::hittable_list::HittableList;
use crate::point3::Point3;
use crate::sphere::Sphere;

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
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.0, -10.0), // FIXME: The book has this at -z: 1.0
        100.0,
    )));

    // Camera
    let camera = camera::CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(10)
        .build();

    camera.render(&world);
    eprintln!("\nDone.");
}
