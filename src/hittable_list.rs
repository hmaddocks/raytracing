use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_t.max;

        // Iterate through all objects and find the closest hit
        for object in &self.objects {
            if let Some(hit) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::TestMaterial;
    use crate::point3::Point3;
    use crate::sphere::Sphere;
    use crate::vec3::Vec3;

    #[test]
    fn test_hittable_list_empty() {
        let list = HittableList::new();
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // An empty list should not hit anything
        let hit_record = list.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit_record.is_none());
    }

    #[test]
    fn test_hittable_list_single_object() {
        let mut list = HittableList::new();

        // Add a sphere at (0,0,5) with radius 1
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 5.0),
            1.0,
            TestMaterial::new(),
        )));

        // Create a ray that should hit the sphere
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the list
        let hit_record = list.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the sphere
        assert!(hit_record.is_some());

        // The hit should be at t = 4.0 (sphere at z=5 with radius 1, ray from origin going in +z direction)
        let hit = hit_record.unwrap();
        assert!((hit.t - 4.0).abs() < 1e-6);

        // The hit point should be at (0, 0, 4) - the front of the sphereb
        let hit_point = hit.position;
        assert!((hit_point.x() - 0.0).abs() < 1e-6);
        assert!((hit_point.y() - 0.0).abs() < 1e-6);
        assert!((hit_point.z() - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_hittable_list_multiple_objects_closest_first() {
        let mut list = HittableList::new();

        // Add two spheres, one closer to the ray origin than the other
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 3.0),
            1.0,
            TestMaterial::new(),
        ))); // Closer sphere
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 8.0),
            1.0,
            TestMaterial::new(),
        ))); // Farther sphere

        // Create a ray that should hit both spheres
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the list
        let hit_record = list.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the list
        assert!(hit_record.is_some());

        // The hit should be at t = 2.0 (closer sphere at z=3 with radius 1)
        let hit = hit_record.unwrap();
        assert!((hit.t - 2.0).abs() < 1e-6);

        // The hit point should be at (0, 0, 2) - the front of the closer sphere
        let hit_point = hit.position;
        assert!((hit_point.x() - 0.0).abs() < 1e-6);
        assert!((hit_point.y() - 0.0).abs() < 1e-6);
        assert!((hit_point.z() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_hittable_list_multiple_objects_closest_last() {
        let mut list = HittableList::new();

        // Add two spheres, but add the farther one first in the list
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 8.0),
            1.0,
            TestMaterial::new(),
        ))); // Farther sphere
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 3.0),
            1.0,
            TestMaterial::new(),
        ))); // Closer sphere

        // Create a ray that should hit both spheres
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the list
        let hit_record = list.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should hit the list
        assert!(hit_record.is_some());

        // The hit should still be at t = 2.0 (closer sphere at z=3 with radius 1)
        // This tests that the list correctly finds the closest hit regardless of order
        let hit = hit_record.unwrap();
        assert!((hit.t - 2.0).abs() < 1e-6);

        // The hit point should be at (0, 0, 2) - the front of the closer sphere
        let hit_point = hit.position;
        assert!((hit_point.x() - 0.0).abs() < 1e-6);
        assert!((hit_point.y() - 0.0).abs() < 1e-6);
        assert!((hit_point.z() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_hittable_list_t_min_max() {
        let mut list = HittableList::new();

        // Add two spheres at different distances
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 3.0),
            1.0,
            TestMaterial::new(),
        ))); // Closer sphere
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 5.0),
            1.0,
            TestMaterial::new(),
        ))); // Farther sphere

        // Create a ray that should hit both spheres
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // Test with t_min that excludes the closer sphere
        let hit_record = list.hit(&ray, Interval::new(3.0, f64::INFINITY));

        // The ray should still hit the farther sphere
        assert!(hit_record.is_some());

        // The hit should be at the front of the farther sphere
        let hit = hit_record.unwrap();

        // For a sphere at z=5 with radius 1, the hit point should be at z=4
        assert!((hit.position.z() - 4.0).abs() < 1e-6);
        assert!((hit.t - 4.0).abs() < 1e-6); // t should equal the z-coordinate in this case

        // Test with t_max that excludes both spheres
        let hit_record = list.hit(&ray, Interval::new(0.001, 1.0));

        // The ray should not hit anything
        assert!(hit_record.is_none());
    }

    #[test]
    fn test_hittable_list_no_hits() {
        let mut list = HittableList::new();

        // Add spheres that the ray will miss
        list.add(Box::new(Sphere::new(
            Point3::new(2.0, 0.0, 5.0),
            1.0,
            TestMaterial::new(),
        ))); // Off to the side
        list.add(Box::new(Sphere::new(
            Point3::new(0.0, 2.0, 8.0),
            1.0,
            TestMaterial::new(),
        ))); // Off to the side

        // Create a ray that should miss both spheres
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        // Check if the ray hits the list
        let hit_record = list.hit(&ray, Interval::new(0.001, f64::INFINITY));

        // The ray should not hit anything
        assert!(hit_record.is_none());
    }
}
