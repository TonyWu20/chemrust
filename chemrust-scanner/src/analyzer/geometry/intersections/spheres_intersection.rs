use nalgebra::{Point3, Unit};

use crate::analyzer::geometry::{Circle, Sphere};

use super::Intersect;

#[derive(Debug)]
pub enum SphereIntersectResult {
    Zero,
    SinglePoint(Point3<f64>),
    Circle(Circle),
    Whole(Sphere),
}

impl Intersect<Sphere> for Sphere {
    type Result = SphereIntersectResult;

    fn intersects(&self, rhs: &Sphere) -> Self::Result {
        let vector_rhs_to_self = self.center - rhs.center; // Vector from rhs to self
        let d = vector_rhs_to_self.norm();
        let radius_sum = self.radius + rhs.radius;
        let radius_diff = self.radius - rhs.radius;
        match d {
            x if radius_diff < x && x < radius_sum => {
                SphereIntersectResult::Circle(two_spheres_between(self, rhs))
            }
            x if x == radius_sum => {
                let n = Unit::new_normalize(vector_rhs_to_self);
                let p = rhs.point_at_surface(&n);
                SphereIntersectResult::SinglePoint(p)
            }
            x if x == 0.0 && radius_diff == 0.0 => SphereIntersectResult::Whole(*self),
            x if x == radius_diff => {
                // Self radius larger than rhs
                let direction = if radius_diff > 0.0 { -1.0 } else { 1.0 };
                let n = Unit::new_normalize(vector_rhs_to_self.scale(direction));
                let p = rhs.point_at_surface(&n);
                SphereIntersectResult::SinglePoint(p)
            }
            _ => SphereIntersectResult::Zero,
        }
    }
}

fn two_spheres_between(s1: &Sphere, s2: &Sphere) -> Circle {
    // First transfer to a 2d, circle-circle problem, as the projections of the spheres.
    let d1d2 = s2.center - s1.center;
    let d = d1d2.norm();
    let det = d.powi(2) - s2.radius.powi(2) - s1.radius.powi(2);
    let x = det / (2.0 * d);
    // radius of the circle
    let a = (4.0 * d.powi(2) * s1.radius.powi(2) - det.powi(2)).sqrt() / (2.0 * d);
    // Go back to 3d
    let n = Unit::new_normalize(d1d2);
    let pt = s1.center + n.scale(x);
    Circle::new(pt, a, n)
}
