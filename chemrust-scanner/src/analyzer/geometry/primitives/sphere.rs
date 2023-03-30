use nalgebra::{Point3, UnitVector3};

use super::GeometryObject;

/// A 3-dimensional form of a sphere, defined by a center and a radius.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn point_at_surface(&self, direction: &UnitVector3<f64>) -> Point3<f64> {
        self.center + direction.scale(self.radius)
    }
}

impl GeometryObject for Sphere {}
