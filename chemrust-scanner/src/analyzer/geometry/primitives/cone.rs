use nalgebra::{Point3, UnitVector3};

use super::GeometryObject;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cone {
    /// Tip of the cone
    pub(crate) tip: Point3<f64>,
    /// Base plane normal
    pub(crate) normal: UnitVector3<f64>,
    /// Base radius
    pub(crate) r: f64,
    /// Slant height `c`
    pub(crate) c: f64,
    /// Height
    pub(crate) h: f64,
}

impl Cone {
    /// Creates a new [`Cone`].
    pub fn new(tip: Point3<f64>, normal: UnitVector3<f64>, r: f64, c: f64, h: f64) -> Self {
        Self {
            normal,
            r,
            c,
            h,
            tip,
        }
    }
    pub fn point_in_cone(&self, point: &Point3<f64>) -> bool {
        let point_to_tip = point - self.tip;
        let cone_dist = point_to_tip.dot(&self.normal);
        if cone_dist < 0.0 || cone_dist - self.h() > 1e-6 {
            return false;
        }
        let cone_radius = (cone_dist / self.h()) * self.r();
        let orth_distance = (point_to_tip - self.normal().scale(cone_dist)).norm();
        orth_distance - cone_radius < -1e-6
    }

    pub fn tip(&self) -> Point3<f64> {
        self.tip
    }

    pub fn normal(&self) -> UnitVector3<f64> {
        self.normal
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn h(&self) -> f64 {
        self.h
    }
}

impl GeometryObject for Cone {}
