use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> (Color, Ray);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> (Color, Ray) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scatter = Ray::new(hit_record.p, scatter_direction);
        (self.albedo, scatter)
    }
}
