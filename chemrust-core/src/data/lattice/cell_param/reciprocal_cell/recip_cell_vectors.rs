use nalgebra::Matrix3;

use crate::data::lattice::UnitCellParameters;

use super::ReciprocalCellParams;
#[derive(Debug, Clone, Copy)]
pub struct ReciprocalCellVectors {
    pub(crate) matrix: Matrix3<f64>,
}

impl<T: UnitCellParameters> From<T> for ReciprocalCellVectors {
    fn from(value: T) -> Self {
        Self {
            matrix: value
                .lattice_bases()
                .try_inverse()
                .expect("Lattice vector matrix should be invertible.")
                .transpose(),
        }
    }
}

impl ReciprocalCellParams for ReciprocalCellVectors {
    fn lattice_bases(&self) -> Matrix3<f64> {
        self.matrix
    }

    fn length_a(&self) -> f64 {
        self.matrix.column(0).norm()
    }

    fn length_b(&self) -> f64 {
        self.matrix.column(1).norm()
    }

    fn length_c(&self) -> f64 {
        self.matrix.column(2).norm()
    }

    fn angle_alpha(&self) -> f64 {
        self.matrix.column(1).angle(&self.matrix.column(2))
    }

    fn angle_beta(&self) -> f64 {
        self.matrix.column(0).angle(&self.matrix.column(2))
    }

    fn angle_gamma(&self) -> f64 {
        self.matrix.column(0).angle(&self.matrix.column(1))
    }
}
