use std::fmt::Display;

use nalgebra::Matrix3;
#[derive(Debug, Clone, Copy)]
pub struct CellConstants {
    pub(crate) a: f64,
    pub(crate) b: f64,
    pub(crate) c: f64,
    pub(crate) alpha: f64,
    pub(crate) beta: f64,
    pub(crate) gamma: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct LatticeVectors {
    pub(crate) tensor: Matrix3<f64>,
}

pub trait UnitCellParameters {
    fn cell_volume(&self) -> f64;
    fn cell_tensor(&self) -> Matrix3<f64>;
    fn metric_tensor(&self) -> Matrix3<f64> {
        let mat = self.cell_tensor();
        let mat_transpose = mat.transpose();
        mat_transpose * mat
    }
}

impl CellConstants {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

impl UnitCellParameters for CellConstants {
    fn cell_volume(&self) -> f64 {
        let CellConstants {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let cos_a = alpha.cos();
        let cos_b = beta.cos();
        let cos_y = gamma.cos();
        a * b
            * c
            * (1.0 - cos_a * cos_a - cos_b * cos_b - cos_y * cos_y + 2.0 * cos_a * cos_b * cos_y)
                .sqrt()
    }

    fn cell_tensor(&self) -> Matrix3<f64> {
        let CellConstants {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let volume = self.cell_volume();
        //     [a         bcosy                     ccosB]
        // A = [0         bsiny   c(cosa - cosbcosy)/siny]
        //     [0             0                v/(absiny)]
        // The columns are `a`, `b` and `c` vectors;
        Matrix3::new(
            *a,
            b * gamma.cos(),
            c * beta.cos(),
            0.0,
            b * gamma.sin(),
            c * (alpha.cos() - beta.cos() * gamma.cos()) / gamma.sin(),
            0.0,
            0.0,
            volume / (a * b * gamma.sin()),
        )
    }
}

impl From<Matrix3<f64>> for CellConstants {
    fn from(mat: Matrix3<f64>) -> Self {
        let (v_a, v_b, v_c) = (mat.row(0), mat.row(1), mat.row(2));
        let (a, b, c) = (v_a.norm(), v_b.norm(), v_c.norm());
        let (alpha, beta, gamma) = (v_b.angle(&v_c), v_a.angle(&v_c), v_a.angle(&v_b));
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

impl From<&Matrix3<f64>> for CellConstants {
    fn from(value: &Matrix3<f64>) -> Self {
        Self::from(*value)
    }
}

impl Display for CellConstants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a_length: {:>20.18}; b_length: {:>20.18}; c_length: {:>20.18}; alpha: {} beta: {} gamma: {}", self.a, self.b, self.c, self.alpha.to_degrees(), self.beta.to_degrees(), self.gamma.to_degrees())
    }
}

impl LatticeVectors {
    pub fn new(tensor: Matrix3<f64>) -> Self {
        Self { tensor }
    }

    pub fn tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
}

impl UnitCellParameters for LatticeVectors {
    fn cell_volume(&self) -> f64 {
        self.tensor().determinant()
    }

    fn cell_tensor(&self) -> Matrix3<f64> {
        self.tensor()
    }
}

impl From<CellConstants> for LatticeVectors {
    fn from(constants: CellConstants) -> Self {
        Self::new(constants.cell_tensor())
    }
}

#[cfg(test)]
mod test {
    use nalgebra::{Matrix3, Point3, Rotation3, Vector3};

    use crate::{
        data::lattice::cell_param::unit_cell::UnitCellParameters,
        systems::crystal_model::rotated_lattice_tensor,
    };

    use super::CellConstants;

    #[test]
    fn cell_repr() {
        let lattice_cart = Matrix3::new(
            // a
            18.931530020488704480,
            -0.000000000000003553,
            0.000000000000000000,
            // b
            -9.465765010246645517,
            16.395185930251127360,
            0.000000000000000000,
            // c
            0.000000000000000000,
            0.000000000000000000,
            9.999213039981000861,
        );
        let cell = CellConstants::from(lattice_cart);
        println!("{}", cell);
        println!("{:#>20.18}", cell.cell_tensor());
        let p = Point3::new(0.07560343470042601, 0.0756034355668187, 0.5000000004346841);
        let o: Point3<f64> = Point3::origin();
        let b = cell.cell_tensor().column(1).xyz();
        let j = Vector3::y_axis();
        let rot = Rotation3::rotation_between(&b, &j).unwrap();
        let mat = rotated_lattice_tensor(&cell, rot);
        let cart_p = mat * p;
        println!("{:#.5}", mat);
        println!("{:?}", j);
        println!("{:#}", cart_p);
        let frac_p =
            cell.cell_tensor().try_inverse().unwrap() * rot.matrix() * cell.cell_tensor() * p;
        println!("{:#}", frac_p);
        println!("{:#}", cell.cell_tensor() * frac_p);
        let po_cart = (cart_p - o).norm_squared();
        let metric_tensor = cell.metric_tensor();
        let po = frac_p - o;
        let po_norm_squared = po.transpose() * metric_tensor * po;
        println!("{:#}", metric_tensor);
        println!(
            "V^2: {}, det(G) : {}",
            cell.cell_volume().powi(2),
            metric_tensor.determinant()
        );
        println!(
            "cart_length: {}, frac_length by metric tensor: {}",
            po_cart, po_norm_squared.x
        );
    }
}
