use nalgebra::{Matrix3, Point3};
/// The fractional coordinates are favored when dealing with atoms in lattice.
/// The support of cartesian coordinates is equally important, but not prioritized.
/// # Notes:
/// `f32` should be enough for accuracy in our 3D geometry calculations.
/// But in order to keep as accurate as the original input from users,
/// use `f64`.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CoordData {
    Fractional(Point3<f64>),
    Cartesian(Point3<f64>),
}

impl CoordData {
    pub fn build_fractional_coord(xyz: &Point3<f64>) -> Self {
        Self::Fractional(*xyz)
    }
    pub fn build_cartesian_coord(xyz: &Point3<f64>) -> Self {
        Self::Cartesian(*xyz)
    }
    /// Accepts `CoordData::Fractional(p)` and returns `CoordData::Cartesian(p)`
    /// If the passed in `frac_coord` is `CoordData::Cartesian(p)`, return itself directly.
    pub fn frac_to_coord(frac_coord: &CoordData, cell_vectors: &Matrix3<f64>) -> CoordData {
        match frac_coord {
            CoordData::Fractional(point) => CoordData::Cartesian(cell_vectors * point),
            CoordData::Cartesian(cart_p) => CoordData::Cartesian(*cart_p),
        }
    }
    pub fn is_fractional(&self) -> bool {
        match *self {
            Self::Fractional(_) => true,
            Self::Cartesian(_) => false,
        }
    }
    pub fn xyz(&self) -> Point3<f64> {
        match *self {
            Self::Fractional(p) => p,
            Self::Cartesian(p) => p,
        }
    }
}
