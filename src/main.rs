use color::Color;
use point3::Point3;
use ray::Ray;
use vec3::Vec3;

mod color;
mod point3;
mod ray;
mod vec3;

//     vec3 oc = center - r.origin();
//     auto a = dot(r.direction(), r.direction());
//     auto b = -2.0 * dot(r.direction(), oc);
//     auto c = dot(oc, oc) - radius*radius;
//     auto discriminant = b*b - 4*a*c;
//     return (discriminant >= 0);
// }

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = center - ray.origin();
    let a = ray.direction().dot(ray.direction());
    let b = -2.0 * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let view_port_u = Vec3::new(viewport_width, 0.0, 0.0);
    let view_port_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = view_port_u / image_width as f64;
    let pixel_delta_v = view_port_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - view_port_u / 2.0 - view_port_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}             ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction: Vec3 = &pixel_center - &camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.write_color());
        }
    }
    eprintln!("\nDone.");
}
