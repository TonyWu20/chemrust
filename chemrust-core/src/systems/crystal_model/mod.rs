use nalgebra::{Matrix3, Rotation3};

use crate::data::{
    atom::CoreAtomData, geom::coordinates::CoordData, lattice::cell_param::UnitCellParameters,
};

/// Convert fractional coordinates to cartesian by reference to the unit cell
/// parameters.
/// # Return
/// - If the input data is mix of fractional coordinate and cartesian coordinate
/// It should be better to refuse doing anything and return `None` to indicate.
pub fn frac_to_cart_coords<T: UnitCellParameters, U: CoreAtomData>(
    lattice_parameters: T,
    atoms_data: U,
) -> Option<Vec<CoordData>> {
    let cell_tensor = lattice_parameters.cell_tensor();
    let all_is_frac = atoms_data
        .coords()
        .iter()
        .all(|coord| coord.is_fractional());
    if !all_is_frac {
        None
    } else {
        Some(
            atoms_data
                .coords()
                .iter()
                .map(|coord| CoordData::Fractional(cell_tensor * coord.xyz()))
                .collect(),
        )
    }
}

pub fn rotated_lattice_tensor<T: UnitCellParameters>(
    lattice_parameters: &T,
    rotation: Rotation3<f64>,
) -> Matrix3<f64> {
    rotation.matrix() * lattice_parameters.cell_tensor()
}

#[cfg(test)]
mod test {
    use std::f64::consts::FRAC_PI_4;

    use nalgebra::{Matrix3, Point3, Rotation3, Vector3};

    use crate::data::{
        geom::coordinates::CoordData,
        lattice::cell_param::{LatticeVectors, UnitCellParameters},
    };

    #[test]
    fn test_rotation_of_frac_coord() {
        let p = CoordData::Fractional(Point3::new(
            0.0756034347004260,
            0.0756034355668187,
            0.5000000004346841,
        ));
        let lattice_vector = LatticeVectors::new(Matrix3::from_columns(&[
            Vector3::new(
                18.931530020488704480,
                -0.000000000000003553,
                0.000000000000000000,
            ),
            Vector3::new(
                -9.465765010246645517,
                16.395185930251127360,
                0.000000000000000000,
            ),
            Vector3::new(
                0.000000000000000000,
                0.000000000000000000,
                9.999213039981000861,
            ),
        ]));
        let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), FRAC_PI_4);
        let cart_p = lattice_vector.cell_tensor() * p.xyz();
        let rot_cart_p = rotation.matrix() * cart_p;
        let back_to_frac_p = lattice_vector.cell_tensor().try_inverse().unwrap() * rot_cart_p;
        let rot = lattice_vector.cell_tensor().try_inverse().unwrap()
            * rotation.matrix()
            * lattice_vector.cell_tensor();
        let rot_frac_p = rot * p.xyz();
        println!(
            "Direct rotate frac_p: {:.3}, cart rotate back to frac_p: {:.3}",
            rot_frac_p, back_to_frac_p
        );
    }
}
