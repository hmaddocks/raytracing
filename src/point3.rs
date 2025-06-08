use crate::vec3::Vec3;
use std::ops::Deref;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
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

    #[inline]
    pub fn as_vec3(&self) -> Vec3 {
        self.0
    }
}

impl From<Vec3> for Point3 {
    fn from(value: Vec3) -> Self {
        Point3(value)
    }
}

impl Deref for Point3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl Sub for Point3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Point3) -> Vec3 {
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
