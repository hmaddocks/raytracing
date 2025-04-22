use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::point3::Point3;
use crate::sphere::Sphere;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let samples_per_pixel = 100;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);

    camera.render(&world);
    eprintln!("\nDone.");
}
