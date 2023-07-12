use nalgebra::{Matrix2, Point3, Unit, Vector2, Vector3};

use crate::analyzer::geometry::{Line, Plane};

use super::Intersect;

impl Intersect<Plane> for Plane {
    type Result = Option<Line>;

    fn intersects(&self, rhs: &Plane) -> Self::Result {
        let n3 = self.n.cross(&rhs.n);
        if (n3.x - 0.0).abs() < 1e-6 && (n3.y - 0.0).abs() < 1e-6 && (n3.z - 0.0).abs() < 1e-6 {
            None
        } else {
            // Perform the reduction methods
            // let z = 1, we have:
            // n1.x + n1.y = d1 - n1.z
            // n2.x + n2.y = d2 - n2.z
            // Organize them as the matrix AX=B form:
            // [n1.x, n1.y] [x] = [d1 - n1.z]
            // [n2.x, n2.y] [y]   [d2 - n2.z]
            // Solve [x,y]
            let m_a = Matrix2::new(self.n.x, self.n.y, rhs.n.x, rhs.n.y);
            if m_a.is_invertible() {
                let m_b = Vector2::new(self.d - self.n.z, rhs.d - rhs.n.z);
                let solution = m_a
                    .lu()
                    .solve(&m_b)
                    .expect(&format!("Linear resolution failed when z is set to 1, m_a: {}, m_b: {}, plane_normal: {}, {}, {}, n3: {}", m_a, m_b, self.n.x, self.n.y, self.n.z, n3));
                let d = Unit::new_normalize(n3);
                Some(Line::new(d, Point3::new(solution.x, solution.y, 1.0)))
            } else {
                // let x = 1
                let m_a = Matrix2::new(self.n.y, self.n.z, rhs.n.y, rhs.n.z);
                let m_b = Vector2::new(self.d - self.n.x, rhs.d - rhs.n.x);
                let solution = m_a.lu().solve(&m_b).unwrap();
                let d = Unit::new_normalize(n3);
                Some(Line::new(d, Point3::new(1.0, solution.x, solution.y)))
            }
        }
    }
}
