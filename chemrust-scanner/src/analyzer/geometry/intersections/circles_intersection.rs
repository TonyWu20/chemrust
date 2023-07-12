use std::cmp::Ordering::{Equal, Greater, Less};

use nalgebra::{Point3, Unit, Vector3};

use crate::analyzer::geometry::{Circle, Line};

use super::Intersect;

/// Determine two 3-dimensional circles' intersection.
/// Generally it is the following steps:
/// - Check if the two circles are parallel or co-planar. If parallel but not co-planar, `Zero`; If co-planar, it is a 2d problem.
/// For not co-planar cases:
/// - Find intersections between the circles' planes. If no, return `Zero`
/// - Check if the circle intersects with the plane intersection line, if either one does not reach the line, return `Zero`
/// - If the intersect points from two circles do not align, they do not intersect either.
/// - There can only be: 1. Two circles touching at one common point; 2. Two circles intersect the line at two common points.
#[derive(Debug)]
pub enum CheckStage {
    Coplanar,
    Noncoplanar,
}

#[derive(Debug)]
pub enum CircleIntersectResult {
    Zero,
    Single(Point3<f64>),
    Double((Point3<f64>, Point3<f64>)),
    Whole(Circle),
}

pub struct CircleIntersectChecker {
    c1: Circle,
    c2: Circle,
    state: CheckStage,
}

impl CircleIntersectChecker {
    pub fn new(c1: &Circle, c2: &Circle) -> Self {
        let n3 = c1.normal.cross(&c2.normal);
        // Ensure the 0.0 is proceeded and found correctly by using epsilon
        if (n3.x - 0.0).abs() < 1e-6 && (n3.y - 0.0).abs() < 1e-6 && (n3.z - 0.0).abs() < 1e-6 {
            CircleIntersectChecker {
                c1: *c1,
                c2: *c2,
                state: CheckStage::Coplanar,
            }
        } else {
            CircleIntersectChecker {
                c1: *c1,
                c2: *c2,
                state: CheckStage::Noncoplanar,
            }
        }
    }
    pub fn check(&self) -> CircleIntersectResult {
        match self.state {
            CheckStage::Coplanar => self.coplanar_check(),
            CheckStage::Noncoplanar => self.noncoplanar_check(),
        }
    }
    /// Seems here have problems
    fn coplanar_check(&self) -> CircleIntersectResult {
        let d1d2 = self.c2.center - self.c1.center;
        // If the line between two circle is not orthogonal to the circle normal, not coplanar
        // !!!! The dot product must use absolute value to compare with the epsilon.
        if d1d2.dot(&self.c1.normal).abs() > 1e-6 {
            CircleIntersectResult::Zero
        } else {
            let radius_sum = self.c1.radius + self.c2.radius;
            let radius_difference = self.c1.radius - self.c2.radius;
            let d = d1d2.norm();
            if d > radius_sum
                || d < 1e-6 && radius_difference > 1e-6
                || (d - radius_difference) < 1e-6
            {
                // Two circles out of distance or concentric or one circle is inside but not touching the edge
                CircleIntersectResult::Zero
            } else if (d - radius_sum).abs() < 1e-6 {
                let p = self.c1.center + d1d2.scale(self.c1.radius);
                CircleIntersectResult::Single(p)
            } else if (d - radius_difference.abs()) < 1e-6 {
                if radius_difference.abs() < 1e-6 {
                    // c1 overlaps with c2 with the same radius
                    CircleIntersectResult::Whole(self.c1)
                } else if radius_difference > 1e-6 {
                    // c1 contains c2
                    let n = Unit::new_normalize(d1d2);
                    let distance = n.scale(self.c2.radius);
                    let p = self.c2.center + distance;
                    CircleIntersectResult::Single(p)
                } else {
                    // c2 contains c1
                    let n = Unit::new_normalize(d1d2).scale(-1.0);
                    let distance = n.scale(self.c1.radius);
                    let p = self.c1.center + distance;
                    CircleIntersectResult::Single(p)
                }
            } else {
                // c1 intersects with c2 when radius_difference d < radius_sum
                CircleIntersectResult::Double(two_circles_between(&self.c1, &self.c2))
            }
        }
    }
    fn noncoplanar_check(&self) -> CircleIntersectResult {
        let plane_1 = self.c1.circle_plane();
        let plane_2 = self.c2.circle_plane();
        // Because the two planes have been guaranteed by checking dot product beforehand
        let plane_intersection_line = plane_1.intersects(&plane_2).unwrap();
        let c1_line_intersection =
            circle_intersection_line_intersect(&self.c1, &plane_intersection_line);
        let c2_line_intersection =
            circle_intersection_line_intersect(&self.c2, &plane_intersection_line);
        // Either circle does not touch the intersection line
        match (c1_line_intersection, c2_line_intersection) {
            (CircleIntersectResult::Single(p1), CircleIntersectResult::Single(p2)) => {
                if let Some(p) = check_identity_points(&p1, &p2) {
                    CircleIntersectResult::Single(p)
                } else {
                    CircleIntersectResult::Zero
                }
            }
            (CircleIntersectResult::Double(points_1), CircleIntersectResult::Double(points_2)) => {
                let (p11, p12) = points_1;
                let (p21, p22) = points_2;
                if check_identity_points(&p11, &p21).is_some()
                    && check_identity_points(&p12, &p22).is_some()
                {
                    CircleIntersectResult::Double(points_1)
                } else {
                    CircleIntersectResult::Zero
                }
            }
            (_, _) => CircleIntersectResult::Zero,
        }
    }
}
/// c1 intersects with c2 when radius_difference d < radius_sum
fn two_circles_between(c1: &Circle, c2: &Circle) -> (Point3<f64>, Point3<f64>) {
    // First transfer to a 2d problem: https://mathworld.wolfram.com/Circle-CircleIntersection.html
    let d1d2 = c2.center - c1.center;
    let d = d1d2.norm();
    let x_frac = d.powi(2) - c2.radius.powi(2) + c1.radius.powi(2);
    let x = x_frac / (2.0 * d);
    let a = (4.0 * d.powi(2) * c1.radius.powi(2) - x_frac.powi(2)).sqrt() / d;
    // Go back to 3d
    let n = Unit::new_normalize(d1d2);
    let pt = c1.center + n.scale(x);
    let chord_dir = Unit::new_normalize(c1.normal.cross(&d1d2));
    let p1 = pt + chord_dir.scale(a / 2.0);
    let p2 = pt + chord_dir.scale(a / -2.0);
    (p1, p2)
}
fn circle_center_to_intersection_line_distance(c: &Circle, line: &Line) -> f64 {
    let origin_to_center = c.center - line.origin;
    let co_line_angle = origin_to_center.angle(&line.d);
    origin_to_center.norm() * co_line_angle.sin()
}
fn circle_intersection_line_intersect(c: &Circle, line: &Line) -> CircleIntersectResult {
    let distance_to_line = circle_center_to_intersection_line_distance(c, line);
    match distance_to_line.partial_cmp(&c.radius) {
        None => CircleIntersectResult::Zero,
        Some(ord) => match ord {
            Greater => CircleIntersectResult::Zero,
            Equal => {
                let origin_to_center = c.center - line.origin;
                let co_line_angle = origin_to_center.angle(&line.d);
                let od_distance = origin_to_center.norm() * co_line_angle.cos();
                let point = line.get_point_at_line(od_distance);
                CircleIntersectResult::Single(point)
            }
            Less => {
                let d = distance_to_line;
                let delta = (c.radius.powi(2) - d.powi(2)).sqrt();
                let origin_to_center = c.center - line.origin;
                let co_line_angle = origin_to_center.angle(&line.d);
                let od_distance = origin_to_center.norm() * co_line_angle.cos();
                let op1_distance = od_distance + delta;
                let op2_distance = od_distance - delta;
                let points = (
                    line.get_point_at_line(op1_distance),
                    line.get_point_at_line(op2_distance),
                );
                CircleIntersectResult::Double(points)
            }
        },
    }
}
fn check_identity_points(p1: &Point3<f64>, p2: &Point3<f64>) -> Option<Point3<f64>> {
    // Ensure floating point cmp
    if (p1.x - p2.x).abs() < 1e-6 && (p1.y - p2.y).abs() < 1e-6 && (p1.z - p2.z).abs() < 1e-6 {
        Some(*p1)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use nalgebra::{Point3, Vector3};

    use crate::analyzer::geometry::Circle;

    use super::two_circles_between;

    #[test]
    fn two_circles() {
        let c1 = Circle::new(Point3::origin(), 2.0, Vector3::z_axis());
        let c2 = Circle::new(Point3::new(0.0, 3.0, 0.0), 2.0, Vector3::z_axis());
        let points = two_circles_between(&c1, &c2);
        dbg!(points);
    }
}
