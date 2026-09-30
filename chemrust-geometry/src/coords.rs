use std::ops::Deref;

use nalgebra::Point3;

/// Type-safe fractional coordinate newtype wrapping nalgebra's `Point3<f64>`.
///
/// Distinguishes fractional from Cartesian coordinates at the type level,
/// while enabling direct matrix-point multiplication (`m33 * coord.0`).
///
/// The read-only `Deref` to `Point3` is deliberate: it exposes `Point3`'s
/// `.x`/`.y`/`.z` fields and its `Index`/`IndexMut` accessors, so call
/// sites can use semantic accessors or `[0]`, `[1]`, `[2]`. Writes go
/// through the `pub` tuple field (`.0`) or the `wrap()` method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FracCoord(pub Point3<f64>);

impl FracCoord {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Point3::new(x, y, z))
    }

    pub fn from_array(a: [f64; 3]) -> Self {
        Self(Point3::new(a[0], a[1], a[2]))
    }

    pub fn into_array(self) -> [f64; 3] {
        [self.0.x, self.0.y, self.0.z]
    }

    /// Wrap to [0, 1) in-place.
    pub fn wrap(&mut self) {
        self.0.x = self.0.x - self.0.x.floor();
        self.0.y = self.0.y - self.0.y.floor();
        self.0.z = self.0.z - self.0.z.floor();
    }
}

impl Deref for FracCoord {
    type Target = Point3<f64>;

    fn deref(&self) -> &Point3<f64> {
        &self.0
    }
}

impl From<[f64; 3]> for FracCoord {
    fn from(a: [f64; 3]) -> Self {
        Self::from_array(a)
    }
}

impl From<FracCoord> for [f64; 3] {
    fn from(fc: FracCoord) -> Self {
        fc.into_array()
    }
}
