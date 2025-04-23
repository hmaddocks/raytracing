use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    #[inline]
    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[inline]
    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin().x(), 1.0);
        assert_eq!(ray.origin().y(), 2.0);
        assert_eq!(ray.origin().z(), 3.0);
        assert_eq!(ray.direction().x(), 4.0);
        assert_eq!(ray.direction().y(), 5.0);
        assert_eq!(ray.direction().z(), 6.0);
    }

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        // Test at t = 0
        let point_at_zero = ray.at(0.0);
        assert_eq!(point_at_zero.x(), 1.0);
        assert_eq!(point_at_zero.y(), 2.0);
        assert_eq!(point_at_zero.z(), 3.0);

        // Test at t = 1
        let point_at_one = ray.at(1.0);
        assert_eq!(point_at_one.x(), 5.0); // 1 + 4*1
        assert_eq!(point_at_one.y(), 7.0); // 2 + 5*1
        assert_eq!(point_at_one.z(), 9.0); // 3 + 6*1

        // Test at t = 2
        let point_at_two = ray.at(2.0);
        assert_eq!(point_at_two.x(), 9.0); // 1 + 4*2
        assert_eq!(point_at_two.y(), 12.0); // 2 + 5*2
        assert_eq!(point_at_two.z(), 15.0); // 3 + 6*2
    }
}
