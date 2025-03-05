use nalgebra::Matrix3;

use super::unit_cell::UnitCellParameters;

#[derive(Debug, Clone, Copy)]
/// The angles are expressed in radians.
pub struct ReciprocalCellConstant {
    pub(crate) recip_a: f64,
    pub(crate) recip_b: f64,
    pub(crate) recip_c: f64,
    pub(crate) recip_alpha: f64,
    pub(crate) recip_beta: f64,
    pub(crate) recip_gamma: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ReciprocalCellVectors {
    pub(crate) matrix: Matrix3<f64>,
}

impl<T: UnitCellParameters> From<T> for ReciprocalCellConstant {
    fn from(value: T) -> Self {
        let volume = value.cell_volume();
        let alpha = value.angle_alpha();
        let beta = value.angle_beta();
        let gamma = value.angle_gamma();
        let a = value.length_a();
        let b = value.length_b();
        let c = value.length_c();
        let cos_recip_a = (beta.cos() * gamma.cos() - alpha.cos()) / (beta.sin() * gamma.sin());
        let cos_recip_b = (gamma.cos() * alpha.cos() - beta.cos()) / (gamma.sin() * alpha.sin());
        let cos_recip_y = (alpha.cos() * beta.cos() - gamma.cos()) / (alpha.sin() * beta.sin());
        Self {
            recip_a: b * c * alpha.sin() / volume,
            recip_b: c * a * beta.sin() / volume,
            recip_c: a * b * gamma.sin() / volume,
            recip_alpha: cos_recip_a.acos(),
            recip_beta: cos_recip_b.acos(),
            recip_gamma: cos_recip_y.acos(),
        }
    }
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

#[cfg(test)]
mod test {
    use core::f64;

    use nalgebra::Matrix3;

    use crate::data::lattice::{
        cell_param::reciprocal_cell::ReciprocalCellVectors, CellConstants, LatticeVectors,
        ReciprocalCellConstant, UnitCellParameters,
    };

    #[test]
    fn test_recip_lat() {
        let lattice_vectors = LatticeVectors::new(Matrix3::<f64>::new(
            9.999_213_039_981,
            0.0,
            0.0,
            0.0,
            9.999_213_039_981,
            0.0,
            0.0,
            0.0,
            16.395_185_930_251_127,
        ));
        println!("{:?}", lattice_vectors);
        let cell_consts = CellConstants::from(lattice_vectors.tensor);
        println!("{:?}", cell_consts);
        println!(
            "cell_volume from lattice_vectors: {}",
            lattice_vectors.cell_volume()
        );
        println!(
            "cell_volume from cell_consts: {}",
            cell_consts.cell_volume()
        );
        let recip_vectors = ReciprocalCellVectors::from(lattice_vectors);
        println!("{:?}", recip_vectors);
        let recip_consts = ReciprocalCellConstant::from(lattice_vectors);
        println!("{:?}", recip_consts);
    }
}
