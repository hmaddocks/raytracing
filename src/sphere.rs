use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root in the range [ray_tmin, ray_tmax]
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::default(),
            front_face: true,
            material: Some(self.material.clone()),
        };

        let outward_normal = &(&hit_record.p - &self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);
        Some(hit_record)
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
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit should be at t = 4.0 (sphere at origin with radius 1, ray from z=-5 going in +z direction)
        let hit = hit_record.unwrap();
        assert!((hit.t - 4.0).abs() < 1e-6);

        // The hit point should be at (0, 0, -1) - the front of the sphere
        let hit_point = hit.p;
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
        let ray = Ray::new(Point3::new(0.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the sphere
        let hit_record = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit point should be at y=1 (tangent to the sphere)
        let hit = hit_record.unwrap();
        let hit_point = hit.p;
        assert!((hit_point.y() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_sphere_hit_miss() {
        // Create a sphere at the origin with radius 1
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, TestMaterial::new());

        // Create a ray that should miss the sphere
        let ray = Ray::new(Point3::new(0.0, 2.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

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
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

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
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

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
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

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
