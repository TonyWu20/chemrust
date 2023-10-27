use nalgebra::{Point, Vector3};

use super::GeometryObject;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cone {
    /// Base plane normal
    pub(crate) normal: Vector3<f64>,
    /// Base radius
    pub(crate) r: f64,
    /// Slant height `c`
    pub(crate) c: f64,
    /// Height
    pub(crate) h: f64,
}

impl Cone {
    pub fn new(normal: Vector3<f64>, r: f64, c: f64, h: f64) -> Self {
        Self { normal, r, c, h }
    }
    pub fn point_in_cone(&self, point: &Point<f64>) -> bool {
        todo!()
    }
}

impl GeometryObject for Cone {}
