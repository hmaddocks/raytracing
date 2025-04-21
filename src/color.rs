use crate::vec3::Vec3;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(Vec3);

impl Color {
    #[inline]
    pub const fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn write_color(&self) -> String {
        let r = self.0.x();
        let g = self.0.y();
        let b = self.0.z();

        // Clamp the color values to the range [0, 1] and then scale up to
        // the range [0, 255] and round to the nearest integer. 255.999 is
        // used to round up in the case of a value of exactly 1.0, which
        // would otherwise be rounded down to 0.
        let r = (255.999 * r.clamp(0.0, 1.0)) as i32;
        let g = (255.999 * g.clamp(0.0, 1.0)) as i32;
        let b = (255.999 * b.clamp(0.0, 1.0)) as i32;

        format!("{} {} {}", r, g, b)
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

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color::new(self.0.x() * other, self.0.y() * other, self.0.z() * other)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0.x(), self.0.y(), self.0.z())
    }
}
