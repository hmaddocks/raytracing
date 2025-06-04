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

/// A builder for creating `Sphere` instances with a fluent interface.
#[derive(Debug, Default)]
pub struct SphereBuilder {
    center: Point3,
    radius: f64,
    material: Option<Material>,
    // New fields for moving sphere
    center_end: Option<Point3>,
    time_start: Option<f64>,
    time_end: Option<f64>,
}

impl SphereBuilder {
    /// Creates a new empty `SphereBuilder`.
    #[inline]
    pub fn new() -> Self {
        Self {
            center: Point3::default(),
            radius: 1.0,
            material: None,
            center_end: None,
            time_start: None,
            time_end: None,
        }
    }

    /// Sets the center point of the sphere.
    #[inline]
    pub fn center(mut self, center: Point3) -> Self {
        self.center = center;
        self
    }

    /// Sets the radius of the sphere.
    #[inline]
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the material of the sphere.
    #[inline]
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    /// Sets the end center point for a moving sphere.
    #[inline]
    pub fn center_end(mut self, center: Point3) -> Self {
        self.center_end = Some(center);
        self
    }

    /// Sets the time range for a moving sphere.
    #[inline]
    pub fn time_range(mut self, start: f64, end: f64) -> Self {
        self.time_start = Some(start);
        self.time_end = Some(end);
        self
    }

    /// Builds a new sphere instance.
    ///
    /// # Returns
    ///
    /// Returns `Some(SphereType)` if all required fields are set, `None` otherwise.
    /// The returned object will be either a `Sphere` or `MovingSphere` depending on whether
    /// moving properties were set.
    #[inline]
    pub fn build(self) -> Option<SphereType> {
        let material = self.material?;

        // If we have all the moving sphere properties, create a MovingSphere
        if let (Some(center_end), Some(time_start), Some(time_end)) =
            (self.center_end, self.time_start, self.time_end)
        {
            Some(SphereType::Moving(MovingSphere::new(
                (self.center, center_end),
                (time_start, time_end),
                self.radius,
                material,
            )))
        } else {
            // Otherwise create a regular Sphere
            Some(SphereType::Static(Sphere::new(
                self.center,
                self.radius,
                material,
            )))
        }
    }
}

/// An enum that can hold either a regular Sphere or a MovingSphere
#[derive(Debug)]
pub enum SphereType {
    Static(Sphere),
    Moving(MovingSphere),
}

impl Hittable for SphereType {
    #[inline]
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            SphereType::Static(sphere) => sphere.hit(ray, ray_t),
            SphereType::Moving(sphere) => sphere.hit(ray, ray_t),
        }
    }

    #[inline]
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self {
            SphereType::Static(sphere) => sphere.bounding_box(time0, time1),
            SphereType::Moving(sphere) => sphere.bounding_box(time0, time1),
        }
    }
}

impl Sphere {
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
        let texture_coords = get_sphere_uv(outward_normal);

        // Create hit record and set the normal based on ray direction
        let mut hit_record = HitRecord {
            t: root,
            position,
            front_face: true,
            material: Some(&self.material),
            texture_coords,
            normal: outward_normal,
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

#[derive(Debug)]
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
fn get_sphere_uv(point: Vec3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       < -1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-point.y()).acos();
    let phi = (-point.z()).atan2(point.x()) + std::f64::consts::PI;

    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    (u, v)
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

        let texture_coords = get_sphere_uv(outward_normal);
        // Create hit record and set the normal based on ray direction
        let mut hit_record = HitRecord {
            t: root,
            position,
            normal: outward_normal,
            front_face: true,
            material: Some(&self.material),
            texture_coords,
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

    #[test]
    fn test_get_sphere_uv() {
        // Test cases from the function documentation
        let test_cases = vec![
            (Vec3::new(1.0, 0.0, 0.0), (0.5, 0.5)), // <1 0 0> yields <0.50 0.50>
            (Vec3::new(-1.0, 0.0, 0.0), (0.0, 0.5)), // < -1 0 0> yields <0.00 0.50>
            (Vec3::new(0.0, 1.0, 0.0), (0.5, 1.0)), // <0 1 0> yields <0.50 1.00>
            (Vec3::new(0.0, -1.0, 0.0), (0.5, 0.0)), // <0 -1 0> yields <0.50 0.00>
            (Vec3::new(0.0, 0.0, 1.0), (0.25, 0.5)), // <0 0 1> yields <0.25 0.50>
            (Vec3::new(0.0, 0.0, -1.0), (0.75, 0.5)), // <0 0 -1> yields <0.75 0.50>
        ];

        for (point, expected) in test_cases {
            let (u, v) = get_sphere_uv(point);
            assert!(
                (u - expected.0).abs() < 1e-6,
                "U coordinate mismatch for point {:?}: expected {}, got {}",
                point,
                expected.0,
                u
            );
            assert!(
                (v - expected.1).abs() < 1e-6,
                "V coordinate mismatch for point {:?}: expected {}, got {}",
                point,
                expected.1,
                v
            );
        }
    }

    #[test]
    fn test_get_sphere_uv_normalized() {
        // Test that the function works with non-unit vectors
        let point = Vec3::new(2.0, 0.0, 0.0);
        let (u, v) = get_sphere_uv(point);
        assert!((u - 0.5).abs() < 1e-6);
        assert!((v - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_get_sphere_uv_range() {
        // Test that UV coordinates are always in [0,1] range
        let test_points = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(-0.5, -0.5, -0.5),
        ];

        for point in test_points {
            let (u, v) = get_sphere_uv(point);
            assert!(
                u >= 0.0 && u <= 1.0,
                "U coordinate out of range [0,1]: {}",
                u
            );
            assert!(
                v >= 0.0 && v <= 1.0,
                "V coordinate out of range [0,1]: {}",
                v
            );
        }
    }
}
