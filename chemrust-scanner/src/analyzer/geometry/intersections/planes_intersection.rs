use nalgebra::{Matrix2, Point3, Unit, Vector2};

use crate::analyzer::geometry::{Line, Plane};

use super::Intersect;

impl Intersect<Plane> for Plane {
    type Result = Option<Line>;

    fn intersects(&self, rhs: &Plane) -> Self::Result {
        let n3 = self.n.cross(&rhs.n);
        // Perform the reduction methods
        // let z = 0, we have:
        // n1.x + n1.y = d1
        // n2.x + n2.y = d2
        // Organize them as the matrix AX=B form:
        // [n1.x, n1.y] [x] = [d1 - n1.z]
        // [n2.x, n2.y] [y]   [d2 - n2.z]
        // Solve [x,y]
        let m_coeff_z0 = Matrix2::new(self.n.x, self.n.y, rhs.n.x, rhs.n.y);
        let m_b = Vector2::new(self.d, rhs.d);
        let solve = m_coeff_z0.lu().solve(&m_b);
        if let Some(solution) = solve {
            let d = Unit::new_normalize(n3);
            Some(Line::new(d, Point3::new(solution.x, solution.y, 0.0)))
        } else {
            let solve = m_coeff_z0.full_piv_lu().solve(&m_b);
            if let Some(solution) = solve {
                let d = Unit::new_normalize(n3);
                Some(Line::new(d, Point3::new(solution.x, solution.y, 0.0)))
            }
            let m_coeff_x0 = Matrix2::new(self.n.y, self.n.z, rhs.n.y, rhs.n.z);
            let m_b = Vector2::new(self.d, rhs.d);
            let solve = m_coeff_x0.lu().solve(&m_b);
            if let Some(solution) = solve {
                let d = Unit::new_normalize(n3);
                Some(Line::new(d, Point3::new(0.0, solution.x, solution.y)))
            } else {
                let m_coeff_y0 = Matrix2::new(self.n.x, self.n.z, rhs.n.x, rhs.n.z);
                let m_b = Vector2::new(self.d, rhs.d);
                let solve = m_coeff_y0.lu().solve(&m_b);
                if let Some(solution) = solve {
                    let d = Unit::new_normalize(n3);
                    Some(Line::new(d, Point3::new(solution.x, 0.0, solution.y)))
                } else {
                    None
                }
            }
        }
    }
}
