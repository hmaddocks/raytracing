use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, PartialEq)]
pub struct Color(Vec3);

impl Color {
    #[inline]
    pub const fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn write_color(&self) -> String {
        // Apply a linear to gamma transform for gamma 2
        let r = Color::linear_to_gamma(self.0.x());
        let g = Color::linear_to_gamma(self.0.y());
        let b = Color::linear_to_gamma(self.0.z());

        // Translate the [0,1] component values to the byte range [0,255].
        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        format!("{} {} {}", rbyte, gbyte, bbyte)
    }

    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            self.0.x() + other.0.x(),
            self.0.y() + other.0.y(),
            self.0.z() + other.0.z(),
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.0[0] += other.0.x();
        self.0[1] += other.0.y();
        self.0[2] += other.0.z();
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color::new(self.0.x() * other, self.0.y() * other, self.0.z() * other)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        self.0[0] *= other;
        self.0[1] *= other;
        self.0[2] *= other;
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0.x(), self.0.y(), self.0.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_color_new() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert!((c.0.x() - 0.1).abs() < EPSILON);
        assert!((c.0.y() - 0.2).abs() < EPSILON);
        assert!((c.0.z() - 0.3).abs() < EPSILON);
    }

    #[test]
    fn test_color_equality() {
        let c1 = Color::new(0.1, 0.2, 0.3);
        let c2 = Color::new(0.1, 0.2, 0.3);
        let c3 = Color::new(0.3, 0.2, 0.1);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_write_color() {
        // Test normal values in range [0,1]
        let c1 = Color::new(0.0, 0.5, 1.0);
        assert_eq!(c1.write_color(), "0 128 255");

        // Test clamping for values > 1.0
        let c2 = Color::new(1.5, 0.5, 2.0);
        assert_eq!(c2.write_color(), "255 128 255");

        // Test clamping for values < 0.0
        let c3 = Color::new(-0.5, 0.5, -1.0);
        assert_eq!(c3.write_color(), "0 128 0");
    }

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.1, 0.2, 0.3);
        let c2 = Color::new(0.2, 0.3, 0.4);
        let result = c1 + c2;

        // Using approx_eq for floating point comparison
        assert!((result.0.x() - 0.3).abs() < EPSILON);
        assert!((result.0.y() - 0.5).abs() < EPSILON);
        assert!((result.0.z() - 0.7).abs() < EPSILON);
    }

    #[test]
    fn test_color_add_assign() {
        let mut c1 = Color::new(0.1, 0.2, 0.3);
        let c2 = Color::new(0.2, 0.3, 0.4);
        c1 += c2;

        // Using approx_eq for floating point comparison
        assert!((c1.0.x() - 0.3).abs() < EPSILON);
        assert!((c1.0.y() - 0.5).abs() < EPSILON);
        assert!((c1.0.z() - 0.7).abs() < EPSILON);
    }

    #[test]
    fn test_color_mul_scalar() {
        let c = Color::new(0.1, 0.2, 0.3);
        let result = c * 2.0;

        let expected = Color::new(0.2, 0.4, 0.6);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_color_mul_assign_scalar() {
        let mut c = Color::new(0.1, 0.2, 0.3);
        c *= 2.0;

        let expected = Color::new(0.2, 0.4, 0.6);
        assert_eq!(c, expected);
    }

    #[test]
    fn test_color_display() {
        let c = Color::new(0.1, 0.2, 0.3);
        let display_string = format!("{}", c);

        assert_eq!(display_string, "0.1 0.2 0.3");
    }

    #[test]
    fn test_color_debug() {
        let c = Color::new(0.1, 0.2, 0.3);
        let debug_string = format!("{:?}", c);

        // The debug format includes the struct name and wraps values
        assert!(debug_string.contains("Color"));
        assert!(debug_string.contains("0.1"));
        assert!(debug_string.contains("0.2"));
        assert!(debug_string.contains("0.3"));
    }
}
