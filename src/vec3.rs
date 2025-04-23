use rand::Rng;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(min..max) // Generate a random f64 in the range [min, max)
}

/// 3D vector for geometric calculations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// Create a new Vec3.
    #[inline]
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    /// X component.
    #[inline]
    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    /// Y component.
    #[inline]
    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    /// Z component.
    #[inline]
    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    /// Length (magnitude) of the vector.
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the unit vector, or zero if the vector is zero.
    #[inline]
    pub fn unit(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            Vec3::default()
        } else {
            self / len
        }
    }

    /// Squared length.
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Dot product.
    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    /// Cross product.
    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    /// Returns a random vector in the range [min, max).
    #[inline]
    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }

    /// Returns a random vector in the unit sphere.
    #[inline]
    pub fn random_unit() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let length_squared = p.length_squared();
            if 1e-160 < length_squared && length_squared <= 1.0 {
                return p / length_squared.sqrt();
            }
        }
    }

    /// Returns a random vector on the hemisphere.
    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Returns true if the vector is near zero.
    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * normal
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vec3_default() {
        let v = Vec3::default();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 7.0);
        assert_eq!(result.z(), 9.0);
    }

    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x(), 3.0);
        assert_eq!(result.y(), 3.0);
        assert_eq!(result.z(), 3.0);
    }

    #[test]
    fn test_vec3_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = -&v;
        assert_eq!(result.x(), -1.0);
        assert_eq!(result.y(), -2.0);
        assert_eq!(result.z(), -3.0);
    }

    #[test]
    fn test_vec3_scalar_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = &v * 2.0;
        assert_eq!(result.x(), 2.0);
        assert_eq!(result.y(), 4.0);
        assert_eq!(result.z(), 6.0);
    }

    #[test]
    fn test_vec3_scalar_div() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = &v / 2.0;
        assert_eq!(result.x(), 1.0);
        assert_eq!(result.y(), 2.0);
        assert_eq!(result.z(), 3.0);
    }

    #[test]
    fn test_vec3_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = &v1 * &v2;
        assert_eq!(result.x(), 4.0);
        assert_eq!(result.y(), 10.0);
        assert_eq!(result.z(), 18.0);
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_vec3_cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert_eq!(result.x(), 0.0);
        assert_eq!(result.y(), 0.0);
        assert_eq!(result.z(), 1.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length_squared(), 25.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vec3_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_vec3_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] = 4.0;
        v[1] = 5.0;
        v[2] = 6.0;
        assert_eq!(v.x(), 4.0);
        assert_eq!(v.y(), 5.0);
        assert_eq!(v.z(), 6.0);
    }

    #[test]
    fn test_vec3_display() {
        let v = Vec3::new(1.1, 2.2, 3.3);
        let s = format!("{}", v);
        assert!(s.contains("1.1"));
        assert!(s.contains("2.2"));
        assert!(s.contains("3.3"));
    }
}
