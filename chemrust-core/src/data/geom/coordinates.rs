use nalgebra::Point3;
/// The fractional coordinates are favored when dealing with atoms in lattice.
/// The support of cartesian coordinates is equally important, but not prioritized.
/// # Notes:
/// `f32` should be enough for accuracy in our 3D geometry calculations.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CoordData {
    Fractional(Point3<f32>),
    Cartesian(Point3<f32>),
}

impl CoordData {
    pub fn build_fractional_coord(xyz: &Point3<f32>) -> Self {
        Self::Fractional(*xyz)
    }
    pub fn build_cartesian_coord(xyz: &Point3<f32>) -> Self {
        Self::Cartesian(*xyz)
    }
}
