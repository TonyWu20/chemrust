use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct FractionalCoordRange(f32, f32);

impl Display for FractionalCoordRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl FractionalCoordRange {
    /// Creates a `FractionalCoordRange` with the range restricted to `[0.0, 1.0]`
    /// # Panics
    /// Panics if `lower > higher`
    pub fn new(lower: f32, higher: f32) -> Self {
        assert!(lower <= higher);
        Self(lower.clamp(0.0, 1.0), higher.clamp(0.0, 1.0))
    }
    pub fn is_in_range(&self, value: f32) -> bool {
        if self.0 <= value && self.1 >= value {
            true
        } else {
            false
        }
    }
    pub fn min(&self) -> f32 {
        self.0
    }
    pub fn max(&self) -> f32 {
        self.1
    }
}
