use rand::Rng;

/// Generate a random f64 in the range [0.0, 1.0)
#[inline]
pub fn random_double() -> f64 {
    random_double_range(0.0, 1.0)
}

/// Generate a random f64 in the range [min, max)
#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}

/// Convert degrees to radians
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
