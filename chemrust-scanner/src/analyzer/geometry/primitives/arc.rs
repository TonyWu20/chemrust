use nalgebra::{distance_squared, Point3, UnitVector3};

/// Section from a circle
/// The arc sweeps from the starting position (indicated by the `radius_starting_dir`,
/// perpendicular to the normal) to the ending position determined by `theta`.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Arc {
    center: Point3<f64>,
    radius: f64,
    theta: f64, // 0-2Pi
    normal: UnitVector3<f64>,
    radius_starting_dir: UnitVector3<f64>,
}

impl Arc {
    pub fn new(
        center: Point3<f64>,
        radius: f64,
        theta: f64,
        normal: UnitVector3<f64>,
        radius_starting_dir: UnitVector3<f64>,
    ) -> Self {
        Self {
            center,
            radius,
            theta,
            normal,
            radius_starting_dir,
        }
    }
    pub fn from_two_points(
        center: &Point3<f64>,
        start_point: &Point3<f64>,
        end_point: &Point3<f64>,
    ) -> Option<Self> {
        let sc = start_point - center;
        let ec = end_point - center;
        if (sc.norm() - ec.norm()).abs() > 1e-6 {
            return None;
        }
        let radius = sc.norm();
        let radius_starting_dir = UnitVector3::new_normalize(sc);
        let normal = UnitVector3::new_normalize(sc.cross(&ec));
        let theta = sc.angle(&ec);
        Some(Self {
            center: *center,
            radius,
            theta,
            normal,
            radius_starting_dir,
        })
    }
    pub fn is_on_arc(&self, point: &Point3<f64>) -> bool {
        let distance = distance_squared(&self.center, point);
        let same_distance = (distance - self.radius).abs() < 1e-6;
        let c_to_p = UnitVector3::new_normalize(point - self.center);
        let angle = self.radius_starting_dir.angle(&c_to_p);
        // If cross product vector dot normal > 0.0, same direction, within the arc,
        // else opposite direction, out of the arc
        let cross_product = self.radius_starting_dir.cross(&c_to_p).dot(&self.normal);
        cross_product > 0.0
            && (angle < self.theta || (angle - self.theta).abs() < 1e-6)
            && same_distance
    }
}
