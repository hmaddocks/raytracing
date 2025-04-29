use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x: Interval::new(0.0, 0.0),
            y: Interval::new(0.0, 0.0),
            z: Interval::new(0.0, 0.0),
        }
    }
}

impl Aabb {
    #[inline]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn other_new(a: Point3, b: Point3) -> Self {
        Self {
            x: if a.x() < b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            y: if a.y() < b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            z: if a.z() < b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        }
    }

    #[inline]
    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index"),
        }
    }
}

impl Hittable for Aabb {
    #[inline]
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        let mut t_min = ray_t.min();
        let mut t_max = ray_t.max();

        for axis in 0..3 {
            let axis_interval = self.axis_interval(axis);
            let inv_d = 1.0 / ray_direction[axis];

            let origin_component = match axis {
                0 => ray_origin.x(),
                1 => ray_origin.y(),
                2 => ray_origin.z(),
                _ => panic!("Invalid axis index"),
            };

            let mut t0 = (axis_interval.min() - origin_component) * inv_d;
            let mut t1 = (axis_interval.max() - origin_component) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            // Update interval
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None;
            }
        }

        // If we've made it here, there is a hit
        Some(HitRecord {
            t: t_min,
            position: ray.at(t_min),
            normal: Vec3::default(), // Arbitrary normal, not used for AABB
            front_face: true,        // Arbitrary, not used for AABB
            material: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hittable::Hittable;
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn test_default() {
        let aabb = Aabb::default();
        assert_eq!(aabb.x, Interval::new(0.0, 0.0));
        assert_eq!(aabb.y, Interval::new(0.0, 0.0));
        assert_eq!(aabb.z, Interval::new(0.0, 0.0));
    }

    #[test]
    fn test_new() {
        let x = Interval::new(1.0, 2.0);
        let y = Interval::new(3.0, 4.0);
        let z = Interval::new(5.0, 6.0);
        let aabb = Aabb::new(x, y, z);

        assert_eq!(aabb.x, x);
        assert_eq!(aabb.y, y);
        assert_eq!(aabb.z, z);
    }

    #[test]
    fn test_other_new() {
        // Test when a < b for all coordinates
        let a = Point3::new(1.0, 2.0, 3.0);
        let b = Point3::new(4.0, 5.0, 6.0);
        let aabb = Aabb::other_new(a, b);

        assert_eq!(aabb.x, Interval::new(1.0, 4.0));
        assert_eq!(aabb.y, Interval::new(2.0, 5.0));
        assert_eq!(aabb.z, Interval::new(3.0, 6.0));

        // Test when a > b for all coordinates
        let aabb = Aabb::other_new(b, a);
        assert_eq!(aabb.x, Interval::new(1.0, 4.0));
        assert_eq!(aabb.y, Interval::new(2.0, 5.0));
        assert_eq!(aabb.z, Interval::new(3.0, 6.0));

        // Test when a and b are mixed
        let c = Point3::new(0.0, 5.0, 2.0);
        let d = Point3::new(3.0, 1.0, 7.0);
        let aabb = Aabb::other_new(c, d);

        assert_eq!(aabb.x, Interval::new(0.0, 3.0));
        assert_eq!(aabb.y, Interval::new(1.0, 5.0));
        assert_eq!(aabb.z, Interval::new(2.0, 7.0));
    }

    #[test]
    fn test_axis_interval() {
        let aabb = Aabb::new(
            Interval::new(1.0, 2.0),
            Interval::new(3.0, 4.0),
            Interval::new(5.0, 6.0),
        );

        assert_eq!(aabb.axis_interval(0), Interval::new(1.0, 2.0));
        assert_eq!(aabb.axis_interval(1), Interval::new(3.0, 4.0));
        assert_eq!(aabb.axis_interval(2), Interval::new(5.0, 6.0));
    }

    #[test]
    #[should_panic(expected = "Invalid axis index")]
    fn test_axis_interval_invalid() {
        let aabb = Aabb::default();
        aabb.axis_interval(3); // Should panic
    }

    #[test]
    fn test_hit_inside_box() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray starting inside the box
        let ray = Ray::new(Point3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0));
        let hit = aabb.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());
    }

    #[test]
    fn test_hit_from_outside() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray starting outside the box and hitting it
        let ray = Ray::new(Point3::new(-1.0, 0.5, 0.5), Vec3::new(1.0, 0.0, 0.0));
        let hit = aabb.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());
    }

    #[test]
    fn test_miss() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray completely missing the box
        let ray = Ray::new(Point3::new(-1.0, -1.0, -1.0), Vec3::new(-1.0, -1.0, -1.0));
        let hit = aabb.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_none());
    }

    #[test]
    fn test_hit_with_t_interval() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray that would hit the box, but t interval excludes the hit
        let ray = Ray::new(Point3::new(-1.0, 0.5, 0.5), Vec3::new(1.0, 0.0, 0.0));

        // Hit should be at t=1.0, so this interval should include it
        let hit1 = aabb.hit(&ray, Interval::new(0.5, 2.0));
        assert!(hit1.is_some());

        // This interval excludes the hit
        let hit2 = aabb.hit(&ray, Interval::new(2.0, 3.0));
        assert!(hit2.is_none());
    }

    #[test]
    fn test_hit_negative_direction() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray with negative direction components
        let ray = Ray::new(Point3::new(2.0, 2.0, 2.0), Vec3::new(-1.0, -1.0, -1.0));
        let hit = aabb.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());
    }

    #[test]
    fn test_hit_parallel_to_axis() {
        let aabb = Aabb::new(
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
            Interval::new(0.0, 1.0),
        );
        // Ray parallel to x-axis
        let ray1 = Ray::new(Point3::new(-1.0, 0.5, 0.5), Vec3::new(1.0, 0.0, 0.0));
        assert!(
            aabb.hit(&ray1, Interval::new(0.001, f64::INFINITY))
                .is_some()
        );

        // Ray parallel to y-axis
        let ray2 = Ray::new(Point3::new(0.5, -1.0, 0.5), Vec3::new(0.0, 1.0, 0.0));
        assert!(
            aabb.hit(&ray2, Interval::new(0.001, f64::INFINITY))
                .is_some()
        );

        // Ray parallel to z-axis
        let ray3 = Ray::new(Point3::new(0.5, 0.5, -1.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(
            aabb.hit(&ray3, Interval::new(0.001, f64::INFINITY))
                .is_some()
        );
    }
}
