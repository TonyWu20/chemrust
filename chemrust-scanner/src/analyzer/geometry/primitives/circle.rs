use nalgebra::{distance_squared, Point3, UnitVector3};

use super::{GeometryObject, Plane};

/// A 3d-dimensional form of a circle, defined by a center, a radius,
/// and the normal vector indicating direction of the plane of the circle.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Circle {
    pub center: Point3<f64>,
    pub radius: f64,
    pub normal: UnitVector3<f64>,
}

impl Circle {
    pub fn new(center: Point3<f64>, radius: f64, normal: UnitVector3<f64>) -> Self {
        Self {
            center,
            radius,
            normal,
        }
    }
    pub fn is_on_circle(&self, point: &Point3<f64>) -> bool {
        let distance = distance_squared(&self.center, point);
        match distance.partial_cmp(&self.radius) {
            Some(std::cmp::Ordering::Equal) => true,
            _ => false,
        }
    }
    pub fn circle_plane(&self) -> Plane {
        Plane::from_point_normal(self.center, self.normal)
    }
}

impl GeometryObject for Circle {}
