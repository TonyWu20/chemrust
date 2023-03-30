mod circles_intersection;
mod planes_intersection;
mod spheres_intersection;

pub use circles_intersection::{CircleIntersectChecker, CircleIntersectResult};

use super::GeometryObject;

pub trait Intersect<T: GeometryObject> {
    type Result;
    fn intersects(&self, rhs: &T) -> Self::Result;
}

#[cfg(test)]
mod test {
    use nalgebra::{Point3, Unit, Vector3};

    use crate::analyzer::geometry::{
        intersections::circles_intersection::CircleIntersectChecker, primitives::Plane, Circle,
        Intersect,
    };

    #[test]
    fn plane_intersections() {
        let p1: Plane = Plane::new(Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0)), 5.0);
        let p2: Plane = Plane::new(Unit::new_normalize(Vector3::new(1.0, 2.0, -3.0)), 9.0);
        let intersection = p1.intersects(&p2);
        assert!(intersection.is_some());
        println!(
            "p0: {}, d: {}",
            intersection.unwrap().origin,
            intersection.unwrap().d.normalize()
        );
        println!("Verify: {}", intersection.unwrap().origin.coords.dot(&p1.n));
    }
    #[test]
    fn circle_intersections() {
        let z_axis_normal = Vector3::z_axis();
        let c1 = Circle::new(Point3::new(0.0, 0.0, 0.0), 5.0, z_axis_normal);
        let c2 = Circle::new(Point3::new(0.0, 2.0, 0.0), 3.0, z_axis_normal);
        let intersects = CircleIntersectChecker::new(&c1, &c2).check();
        println!("{:?}", intersects);
        let c3 = Circle::new(
            Point3::new(0.0, 3.0, 0.0),
            4.0,
            Unit::new_normalize(Vector3::new(0.0, 1.0, 1.0)),
        );
        let intersects = CircleIntersectChecker::new(&c1, &c3).check();
        println!("{:?}", intersects);
        let c4 = Circle::new(
            Point3::new(3.0, 4.0, 5.0),
            12.0,
            Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0)),
        );
        let intersects_c3c4 = CircleIntersectChecker::new(&c3, &c4).check();
        println!("{:?}", intersects_c3c4);
    }
}
