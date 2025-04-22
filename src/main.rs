use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;

mod color;
mod hittable;
mod hittable_list;
mod point3;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(hit_record) = world.hit(r, 0.001, f64::INFINITY) {
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

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;

    // Calculate image height
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.0, -10.0), // FIXME: The book has this at -z: 1.0
        100.0,
    )));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let view_port_u = Point3::new(viewport_width, 0.0, 0.0);
    let view_port_v = Point3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = view_port_u / image_width as f64;
    let pixel_delta_v = view_port_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left_vec = camera_center.as_vec3()
        - Point3::new(0.0, 0.0, focal_length).as_vec3()
        - (view_port_u / 2.0).as_vec3()
        - (view_port_v / 2.0).as_vec3();
    let viewport_upper_left: Point3 = viewport_upper_left_vec.into();

    // Calculate pixel 0,0 location
    let pixel00_loc =
        viewport_upper_left + 0.5 * pixel_delta_u.as_vec3() + 0.5 * pixel_delta_v.as_vec3();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}             ", image_height - j);
        for i in 0..image_width {
            let pixel_center: Point3 =
                pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);

            println!("{}", pixel_color.write_color());
        }
    }
    eprintln!("\nDone.");
}
