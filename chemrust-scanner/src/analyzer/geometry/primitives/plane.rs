use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use super::{Circle, GeometryObject};

/// A 3-dimensional plane formed from the equation: A*x + B*y + C*z - D = 0.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Plane {
    pub n: UnitVector3<f64>,
    pub d: f64,
}

impl Plane {
    pub fn new(n: UnitVector3<f64>, d: f64) -> Self {
        Self { n, d }
    }
    pub fn from_point_normal(point: Point3<f64>, normal: UnitVector3<f64>) -> Self {
        Self {
            n: normal,
            d: normal.dot(&point.coords),
        }
    }
    pub fn from_points(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<Self> {
        let v0 = b - a;
        let v1 = c - a;
        let n = v0.cross(&v1);
        if n == Vector3::zeros() {
            None
        } else {
            let n = Unit::new_normalize(n);
            let d = -a.coords.dot(&n);
            Some(Plane::new(n, d))
        }
    }
}

impl From<Circle> for Plane {
    fn from(value: Circle) -> Self {
        Plane::from_point_normal(value.center, value.normal)
    }
}

impl GeometryObject for Plane {}
