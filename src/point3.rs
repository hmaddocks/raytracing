use crate::vec3::Vec3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3(Vec3);

impl Point3 {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3(Vec3::new(x, y, z))
    }

    #[inline]
    pub const fn x(&self) -> f64 {
        self.0.x()
    }

    #[inline]
    pub const fn y(&self) -> f64 {
        self.0.y()
    }

    #[inline]
    pub const fn z(&self) -> f64 {
        self.0.z()
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    #[inline]
    fn add(self, other: Vec3) -> Point3 {
        Point3::new(
            self.0.x() + other.x(),
            self.0.y() + other.y(),
            self.0.z() + other.z(),
        )
    }
}

// Prevent adding two points together by making this panic with a clear error message
impl Add<Point3> for Point3 {
    type Output = Point3;

    #[inline]
    fn add(self, _other: Point3) -> Point3 {
        panic!("Cannot add two Point3 values together: operation is undefined.");
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    #[inline]
    fn sub(self, other: Vec3) -> Point3 {
        Point3::new(
            self.0.x() - other.x(),
            self.0.y() - other.y(),
            self.0.z() - other.z(),
        )
    }
}

// Allow Point3 - Point3 = Vec3
impl Sub for &Point3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: &Point3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot add two Point3 values together: operation is undefined.")]
    fn test_point3_add_point3_panics() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(4.0, 5.0, 6.0);
        let _ = p1 + p2;
    }

    #[test]
    fn test_point3_creation() {
        let p = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
    }

    #[test]
    fn test_point3_zero() {
        let p = Point3::new(0.0, 0.0, 0.0);
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
        assert_eq!(p.z(), 0.0);
    }

    #[test]
    fn test_point3_negative() {
        let p = Point3::new(-1.0, -2.0, -3.0);
        assert_eq!(p.x(), -1.0);
        assert_eq!(p.y(), -2.0);
        assert_eq!(p.z(), -3.0);
    }
}
