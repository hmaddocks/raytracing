use color::Color;
use point3::Point3;
use ray::Ray;
use vec3::Vec3;

mod color;
mod hittable;
mod point3;
mod ray;
mod sphere;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = center - ray.origin();
    let a = ray.direction().length_squared();
    let h = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((h - discriminant.sqrt()) / a)
    }
}

fn ray_color(r: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        let n: Vec3 = (r.at(t) - Point3::new(0.0, 0.0, -1.0)).unit();
        Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    } else {
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let view_port_u = Point3::new(viewport_width, 0.0, 0.0);
    let view_port_v = Point3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = view_port_u / image_width as f64;
    let pixel_delta_v = view_port_v / image_height as f64;

    let viewport_upper_left_vec = camera_center.as_vec3()
        - Point3::new(0.0, 0.0, focal_length).as_vec3()
        - (view_port_u / 2.0).as_vec3()
        - (view_port_v / 2.0).as_vec3();
    let viewport_upper_left: Point3 = viewport_upper_left_vec.into();

    let pixel00_loc =
        viewport_upper_left + 0.5 * pixel_delta_u.as_vec3() + 0.5 * pixel_delta_v.as_vec3();

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

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.write_color());
        }
    }
    eprintln!("\nDone.");
}
