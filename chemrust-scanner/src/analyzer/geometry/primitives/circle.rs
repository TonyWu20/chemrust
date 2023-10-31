use nalgebra::{distance_squared, Point3, UnitVector3, Vector3};

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
        // use `matches!` macro
        matches!(
            distance.partial_cmp(&self.radius),
            Some(std::cmp::Ordering::Equal)
        )
    }
    pub fn circle_plane(&self) -> Plane {
        Plane::from_point_normal(self.center, self.normal)
    }
    pub fn point_to_circle_distances(&self, point: &Point3<f64>) -> (f64, f64) {
        // Vector from center to point
        let center_to_point: Vector3<f64> = point - self.center;
        // Because norm is a unit vector, dot product of `cp` and `norm` is the length of
        // the projection.
        let dot_cp_norm: f64 = center_to_point.dot(&self.normal);
        // Get the coordinate of the projection point
        let projection_point: Point3<f64> = point - self.normal.scale(dot_cp_norm);
        let center_to_projection_direction: UnitVector3<f64> =
            UnitVector3::new_normalize(projection_point - self.center);
        let closest_point: Point3<f64> =
            self.center + center_to_projection_direction.scale(self.radius);
        let closest_dist = (point - closest_point).norm();
        let farthest_point: Point3<f64> =
            self.center + center_to_projection_direction.scale(-1.0 * self.radius);
        let farthest_dist = (point - farthest_point).norm();
        (closest_dist, farthest_dist)
    }
}

impl GeometryObject for Circle {}
