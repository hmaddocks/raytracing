//! Sphere implementation for ray tracing.
//!
//! This module provides a `Sphere` struct that implements the `Hittable` trait,
//! allowing rays to intersect with spheres in the scene.

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A sphere defined by its center point, radius, and material.
#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    radius_squared: f64, // Pre-computed for efficiency
    material: Material,
}

impl Sphere {
    /// Creates a new sphere with the given center, radius, and material.
    ///
    /// # Arguments
    ///
    /// * `center` - The center point of the sphere
    /// * `radius` - The radius of the sphere
    /// * `material` - The material of the sphere
    ///
    /// # Returns
    ///
    /// A new `Sphere` instance
    #[inline]
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            radius_squared: radius * radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    #[inline]
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Get the current center based on time (for moving spheres)
        let current_center = self.center;

        // Vector from ray origin to sphere center
        let oc = *ray.origin() - current_center;

        // Coefficients of the quadratic equation for sphere intersection
        // Using the optimized quadratic formula: ax² + 2bx + c = 0
        // where b = half_b in our implementation
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius_squared;

        // Calculate discriminant to determine if ray intersects sphere
        let discriminant = half_b * half_b - a * c;

        // Early return if no intersection (discriminant is negative)
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root in the acceptable range
        // First try the closer intersection
        let mut root = (-half_b - sqrt_discriminant) / a;

        // If closer intersection is not in range, try the farther one
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        // Calculate hit position
        let position = ray.at_time(root);

        // Calculate outward normal at hit point (normalized vector from center to hit point)
        let outward_normal = (position - current_center) / self.radius;

        // Create hit record and set the normal based on ray direction
        let mut hit_record = HitRecord {
            t: root,
            position,
            normal: Vec3::default(),
            front_face: true,
            material: Some(self.material.clone()),
        };

        hit_record.set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }

    #[inline]
    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Interval::new(self.center.x() - self.radius, self.center.x() + self.radius),
            Interval::new(self.center.y() - self.radius, self.center.y() + self.radius),
            Interval::new(self.center.z() - self.radius, self.center.z() + self.radius),
        ))
    }
}

pub struct MovingSphere {
    center: (Point3, Point3),
    time: (f64, f64),
    radius: f64,
    radius_squared: f64, // Pre-computed for efficiency
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center: (Point3, Point3),
        time: (f64, f64),
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            center,
            time,
            radius: radius.max(0.0),
            radius_squared: radius * radius,
            material,
        }
    }

    pub fn center_at(&self, time: f64) -> Point3 {
        self.center.0
            + (self.center.1 - self.center.0) * (time - self.time.0) / (self.time.1 - self.time.0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Get the current center based on time (for moving spheres)
        let current_center = self.center_at(ray.time());

        // Vector from ray origin to sphere center
        let oc = *ray.origin() - current_center;

        // Coefficients of the quadratic equation for sphere intersection
        // Using the optimized quadratic formula: ax² + 2bx + c = 0
        // where b = half_b in our implementation
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius_squared;

        // Calculate discriminant to determine if ray intersects sphere
        let discriminant = half_b * half_b - a * c;

        // Early return if no intersection (discriminant is negative)
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root in the acceptable range
        // First try the closer intersection
        let mut root = (-half_b - sqrt_discriminant) / a;

        // If closer intersection is not in range, try the farther one
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        // Calculate hit position
        let position = ray.at_time(root);

        // Calculate outward normal at hit point (normalized vector from center to hit point)
        let outward_normal = (position - current_center) / self.radius;

        // Create hit record and set the normal based on ray direction
        let mut hit_record = HitRecord {
            t: root,
            position,
            normal: Vec3::default(),
            front_face: true,
            material: Some(self.material.clone()),
        };

        hit_record.set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let bbox0 = Aabb::new(
            Interval::new(
                self.center.0.x() - self.radius,
                self.center.0.x() + self.radius,
            ),
            Interval::new(
                self.center.0.y() - self.radius,
                self.center.0.y() + self.radius,
            ),
            Interval::new(
                self.center.0.z() - self.radius,
                self.center.0.z() + self.radius,
            ),
        );
        let bbox1 = Aabb::new(
            Interval::new(
                self.center.1.x() - self.radius,
                self.center.1.x() + self.radius,
            ),
            Interval::new(
                self.center.1.y() - self.radius,
                self.center.1.y() + self.radius,
            ),
            Interval::new(
                self.center.1.z() - self.radius,
                self.center.1.z() + self.radius,
            ),
        );
        Some(Aabb::surrounding(&bbox0, &bbox1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::TestMaterial;
    use crate::vec3::Vec3;

    #[test]
    fn test_sphere_hit_direct_hit() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray that should hit the sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit should be at t = 4.0 (sphere at origin with radius 1, ray from z=-5 going in +z direction)
        let hit = hit_record.unwrap();
        assert!((hit.t - 4.0).abs() < 1e-6);

        // The hit point should be at (0, 0, -1) - the front of the sphere
        let hit_point = hit.position;
        assert!((hit_point.x() - 0.0).abs() < 1e-6);
        assert!((hit_point.y() - 0.0).abs() < 1e-6);
        assert!((hit_point.z() - (-1.0)).abs() < 1e-6);

        // The normal should point outward from the sphere at the hit point
        let normal = hit.normal;
        assert!((normal.x() - 0.0).abs() < 1e-6);
        assert!((normal.y() - 0.0).abs() < 1e-6);
        assert!((normal.z() - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_sphere_hit_tangent() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray that should hit the sphere tangentially
        let ray = Ray::new(Point3::new(0.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit point should be at y=1 (tangent to the sphere)
        let hit = hit_record.unwrap();
        let hit_point = hit.position;
        assert!((hit_point.y() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_sphere_hit_miss() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray that should miss the sphere
        let ray = Ray::new(Point3::new(0.0, 2.0, -5.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should miss the sphere
        assert!(hit_record.is_none());
    }

    #[test]
    fn test_sphere_hit_from_inside() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray from inside the sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit should be at t = 1.0 (sphere radius is 1, ray from origin going in +z direction)
        let hit = hit_record.unwrap();
        assert!((hit.t - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_sphere_hit_behind_ray() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 5.0), 1.0, TestMaterial::new());

        // Create a ray pointing away from the sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 0.0);

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should miss the sphere since it's pointing away
        assert!(hit_record.is_none());
    }

    #[test]
    fn test_sphere_hit_t_min_max() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray that should hit the sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), 0.0);

        // The ray hits at t=4 (front) and t=6 (back)

        // Check with t_min > front hit point but < back hit point
        let hit_record = sphere.hit(&ray, Interval::new(5.0, f64::INFINITY));

        // The ray should still hit the sphere at the back intersection (t=6)
        assert!(hit_record.is_some());
        let hit = hit_record.unwrap();
        assert!((hit.t - 6.0).abs() < 1e-6);

        // Check with t_max < both hit points
        let hit_record = sphere.hit(&ray, Interval::new(0.001, 3.0));

        // The ray should miss the sphere due to t_max constraint
        assert!(hit_record.is_none());

        // Check with t_min > both hit points
        let hit_record = sphere.hit(&ray, Interval::new(7.0, f64::INFINITY));

        // The ray should miss the sphere due to t_min constraint
        assert!(hit_record.is_none());
    }
}
