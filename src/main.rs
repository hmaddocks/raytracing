use crate::bvh::Bvh;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point3::Point3;
use crate::sphere::{SphereBuilder, SphereType};
use crate::texture::{CheckerTexture, TextureEnum};
use crate::utilities::random_double;
use crate::vec3::Vec3;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod point3;
mod ray;
mod sphere;
mod texture;
mod utilities;
mod vec3;

fn bouncing_spheres() -> () {
    // World
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(0.0, -1000.0, 0.0))
            .radius(1000.0)
            .material(Lambertian::new(Box::new(TextureEnum::CheckerTexture(
                CheckerTexture::new(
                    3.0,
                    Box::new(TextureEnum::SolidColor(Color::new(1.0, 1.0, 1.0).into())),
                    Box::new(TextureEnum::SolidColor(Color::new(0.0, 0.0, 0.0).into())),
                ),
            ))))
            .build()
            .expect("Failed to build ground sphere"),
    ));

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
                    let center2 = center + Vec3::new(0.0, random_double() * 0.5, 0.0);
                    if let Some(SphereType::Moving(moving_sphere)) = SphereBuilder::new()
                        .center(center)
                        .center_end(center2)
                        .radius(0.2)
                        .material(Lambertian::new(Box::new(TextureEnum::SolidColor(
                            Color::new(random_double(), random_double(), random_double()).into(),
                        ))))
                        .time_range(0.0, 1.0)
                        .build()
                    {
                        objects.push(Box::new(moving_sphere));
                    } else {
                        panic!("Failed to build moving sphere");
                    }
                } else if choose_mat < 0.95 {
                    objects.push(Box::new(
                        SphereBuilder::new()
                            .center(center)
                            .radius(0.2)
                            .material(Metal::new(
                                Color::new(random_double(), random_double(), random_double()),
                                0.5,
                            ))
                            .build()
                            .expect("Failed to build metal sphere"),
                    ));
                } else {
                    objects.push(Box::new(
                        SphereBuilder::new()
                            .center(center)
                            .radius(0.2)
                            .material(Dielectric::new(1.5))
                            .build()
                            .expect("Failed to build dielectric sphere"),
                    ));
                }
            }
        }
    }

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(0.0, 1.0, 0.0))
            .radius(1.0)
            .material(Dielectric::new(1.5))
            .build()
            .expect("Failed to build large dielectric sphere"),
    ));

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(-4.0, 1.0, 0.0))
            .radius(1.0)
            .material(Lambertian::new(Box::new(TextureEnum::SolidColor(
                Color::new(0.4, 0.2, 0.1).into(),
            ))))
            .build()
            .expect("Failed to build brown lambertian sphere"),
    ));

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(4.0, 1.0, 0.0))
            .radius(1.0)
            .material(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))
            .build()
            .expect("Failed to build metal sphere"),
    ));

    // Build BVH from objects
    let world = Bvh::new(objects).expect("Failed to create BVH");

    // Camera
    let camera = camera::CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(1.0)
        .focus_dist(10.0)
        .build();

    camera.render(&world as &dyn Hittable);
}

fn checkered_spheres() -> () {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let checker = CheckerTexture::new(
        3.0,
        Box::new(TextureEnum::SolidColor(Color::new(0.2, 0.3, 0.1).into())),
        Box::new(TextureEnum::SolidColor(Color::new(0.9, 0.9, 0.9).into())),
    );

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(0.0, -10.0, 0.0))
            .radius(10.0)
            .material(Lambertian::new(Box::new(TextureEnum::CheckerTexture(
                checker.clone(),
            ))))
            .build()
            .expect("Failed to build ground sphere"),
    ));

    objects.push(Box::new(
        SphereBuilder::new()
            .center(Point3::new(0.0, 10.0, 0.0))
            .radius(10.0)
            .material(Lambertian::new(Box::new(TextureEnum::CheckerTexture(
                checker.clone(),
            ))))
            .build()
            .expect("Failed to build ground sphere"),
    ));

    let world = Bvh::new(objects).expect("Failed to create BVH");

    let camera = camera::CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(800)
        .samples_per_pixel(100)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .focus_dist(10.0)
        .build();

    camera.render(&world as &dyn Hittable);
}

fn main() {
    // bouncing_spheres();
    checkered_spheres();
}
