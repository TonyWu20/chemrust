use nalgebra::Point3;
use std::marker::PhantomData;
/// The fractional coordinates are favored when dealing with atoms in lattice.
/// The support of cartesian coordinates is equally important
/// # Notes:
/// `f32` should be enough for accuracy in our 3D geometry calculations.

pub trait CoordinateType: Copy {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Fractional;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Cartesian;

impl CoordinateType for Fractional {}
impl CoordinateType for Cartesian {}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coordinate<T: CoordinateType> {
    xyz: Point3<f32>,
    coord_type: PhantomData<T>,
}

impl<T: CoordinateType> Coordinate<T> {
    pub fn new(xyz: Point3<f32>) -> Self {
        Self {
            xyz,
            coord_type: PhantomData,
        }
    }

    pub fn xyz(&self) -> Point3<f32> {
        self.xyz
    }
}
