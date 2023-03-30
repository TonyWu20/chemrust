use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use super::GeometryObject;

/// A 3-dimensional form of a line: pt = p0 + t*D
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Line {
    pub d: UnitVector3<f64>,
    pub origin: Point3<f64>,
}

impl Line {
    pub fn new(d: UnitVector3<f64>, origin: Point3<f64>) -> Self {
        Self { d, origin }
    }
    pub fn from_two_points(origin: Point3<f64>, dest: Point3<f64>) -> Option<Self> {
        let d = dest - origin;
        if d == Vector3::zeros() {
            None
        } else {
            let d = Unit::new_normalize(d);
            Some(Line::new(d, origin))
        }
    }
    pub fn get_point_at_line(&self, distance: f64) -> Point3<f64> {
        self.origin + self.d.scale(distance)
    }
}

impl GeometryObject for Line {}
