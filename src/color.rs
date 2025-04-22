use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

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

        // Translate the [0,1] component values to the byte range [0,255].
        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        format!("{} {} {}", rbyte, gbyte, bbyte)
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
