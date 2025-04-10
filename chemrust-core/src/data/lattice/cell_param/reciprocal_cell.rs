mod recip_cell_constant;
mod recip_cell_vectors;

use nalgebra::Matrix3;
pub use recip_cell_constant::ReciprocalCellConstant;
pub use recip_cell_vectors::ReciprocalCellVectors;

/// Traits that a struct to represent
/// reciprocal cell parameters.
pub trait ReciprocalCellParams {
    /// Return the reciprocal lattice vectors
    /// in `nalgebra::Matrix3<f64>`
    fn lattice_bases(&self) -> Matrix3<f64>;
    fn length_a(&self) -> f64;
    fn length_b(&self) -> f64;
    fn length_c(&self) -> f64;
    fn angle_alpha(&self) -> f64;
    fn angle_beta(&self) -> f64;
    fn angle_gamma(&self) -> f64;
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
