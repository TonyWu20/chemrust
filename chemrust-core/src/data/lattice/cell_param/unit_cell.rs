use std::fmt::Display;

use nalgebra::Matrix3;
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub(crate) a: f64,
    pub(crate) b: f64,
    pub(crate) c: f64,
    pub(crate) alpha: f64,
    pub(crate) beta: f64,
    pub(crate) gamma: f64,
}

impl Cell {
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
    pub fn from_matrix(mat: &Matrix3<f64>) -> Self {
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
    pub fn cell_volume(&self) -> f64 {
        let Cell {
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
    /// Matrix (3x3) representation of the cell lattice parameters (lattice vectors)
    pub fn matrix_repr(&self) -> Matrix3<f64> {
        let Cell {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let volume = self.cell_volume();
        //     [a         0                              0]
        // A = [bcosy     bsiny                          0]
        //     [ccosB c(cosa - cosbcosy)/siny   v/(absiny)]
        Matrix3::new(
            *a,
            0.0,
            0.0,
            b * gamma.cos(),
            b * gamma.sin(),
            0.0,
            c * beta.cos(),
            c * (alpha.cos() - beta.cos() * gamma.cos()) / gamma.sin(),
            volume / (a * b * gamma.sin()),
        )
    }
}

impl From<Matrix3<f64>> for Cell {
    fn from(value: Matrix3<f64>) -> Self {
        Self::from_matrix(&value)
    }
}

impl From<&Matrix3<f64>> for Cell {
    fn from(value: &Matrix3<f64>) -> Self {
        Self::from_matrix(value)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a_length: {:>20.18}; b_length: {:>20.18}; c_length: {:>20.18}; alpha: {} beta: {} gamma: {}", self.a, self.b, self.c, self.alpha.to_degrees(), self.beta.to_degrees(), self.gamma.to_degrees())
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Matrix3;

    use super::Cell;

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
        let cell = Cell::from(lattice_cart);
        println!("{}", cell);
        println!("{:#>20.18}", cell.matrix_repr());
    }
}
