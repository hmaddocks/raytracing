use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct HitRecord<'a> {
    pub position: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Option<&'a Material>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

impl HitRecord<'_> {
    /// Sets the HitRecord's normal vector
    ///
    /// The parameter `outward_normal` is assumed to have unit length
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord<'_> {
    fn default() -> Self {
        Self {
            position: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: None,
        }
    }
}
