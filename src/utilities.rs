use rand::Rng;

use crate::vec3::Vec3;

/// Generate a random f64 in the range [0.0, 1.0)
#[inline]
pub fn random_double() -> f64 {
    random_double_range(0.0, 1.0)
}

/// Generate a random f64 in the range [min, max)
#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

/// Generate a random point in the unit square [-0.5, 0.5)
#[inline]
pub fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}

/// Generate a random point in the unit disk
#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Convert degrees to radians
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
