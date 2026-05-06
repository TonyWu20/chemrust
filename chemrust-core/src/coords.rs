use nalgebra::Point3;

/// Type-safe fractional coordinate newtype.
///
/// Distinguishes fractional from Cartesian coordinates at the type level.
/// Useful in function signatures where mixing up coordinate systems is a risk.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FracCoord(pub [f64; 3]);

impl FracCoord {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn to_point(&self) -> Point3<f64> {
        Point3::new(self.0[0], self.0[1], self.0[2])
    }

    pub fn from_array(a: [f64; 3]) -> Self {
        Self(a)
    }

    pub fn into_array(self) -> [f64; 3] {
        self.0
    }

    /// Wrap to [0, 1) in-place.
    pub fn wrap(&mut self) {
        self.0[0] = self.0[0] - self.0[0].floor();
        self.0[1] = self.0[1] - self.0[1].floor();
        self.0[2] = self.0[2] - self.0[2].floor();
    }
}

impl From<[f64; 3]> for FracCoord {
    fn from(a: [f64; 3]) -> Self {
        Self(a)
    }
}

impl From<FracCoord> for [f64; 3] {
    fn from(fc: FracCoord) -> Self {
        fc.0
    }
}
