#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    #[inline]
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    // #[inline]
    // pub fn size(&self) -> f64 {
    //     self.max - self.min
    // }

    // #[inline]
    // pub fn contains(&self, value: f64) -> bool {
    //     self.min <= value && value <= self.max
    // }

    #[inline]
    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    // #[inline]
    // pub fn empty() -> Self {
    //     Interval {
    //         min: f64::INFINITY,
    //         max: f64::NEG_INFINITY,
    //     }
    // }

    // #[inline]
    // pub fn universe() -> Self {
    //     Interval {
    //         min: f64::NEG_INFINITY,
    //         max: f64::INFINITY,
    //     }
    // }

    #[inline]
    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    #[inline]
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval { min: 0.0, max: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interval = Interval::new(1.0, 5.0);
        assert_eq!(interval.min, 1.0);
        assert_eq!(interval.max, 5.0);
    }

    #[test]
    fn test_surrounds_inside() {
        let interval = Interval::new(1.0, 5.0);
        assert!(interval.surrounds(3.0));
    }

    #[test]
    fn test_surrounds_outside() {
        let interval = Interval::new(1.0, 5.0);
        assert!(!interval.surrounds(0.0));
        assert!(!interval.surrounds(5.0));
        assert!(!interval.surrounds(1.0));
        assert!(!interval.surrounds(6.0));
    }

    #[test]
    fn test_default() {
        let interval = Interval::default();
        assert_eq!(interval.min, 0.0);
        assert_eq!(interval.max, 0.0);
    }

    // #[test]
    // fn test_size() {
    //     let interval = Interval::new(2.0, 5.5);
    //     assert_eq!(interval.size(), 3.5);
    // }

    // #[test]
    // fn test_contains() {
    //     let interval = Interval::new(1.0, 4.0);
    //     assert!(interval.contains(1.0));
    //     assert!(interval.contains(4.0));
    //     assert!(interval.contains(2.5));
    //     assert!(!interval.contains(0.99));
    //     assert!(!interval.contains(4.01));
    // }

    // #[test]
    // fn test_empty() {
    //     let interval = Interval::empty();
    //     assert_eq!(interval.min, f64::INFINITY);
    //     assert_eq!(interval.max, f64::NEG_INFINITY);
    // }

    // #[test]
    // fn test_universe() {
    //     let interval = Interval::universe();
    //     assert_eq!(interval.min, f64::NEG_INFINITY);
    //     assert_eq!(interval.max, f64::INFINITY);
    // }
}
