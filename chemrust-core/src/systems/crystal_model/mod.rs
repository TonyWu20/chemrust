use nalgebra::{Matrix3, Rotation3};

use crate::data::{atom::CoreAtomData, geom::coordinates::CoordData, lattice::UnitCellParameters};

/// Convert fractional coordinates to cartesian by reference to the unit cell
/// parameters.
/// # Return
/// - If the input data is mix of fractional coordinate and cartesian coordinate
/// It should be better to refuse doing anything and return `None` to indicate.
pub fn frac_to_cart_coords<T: UnitCellParameters, U: CoreAtomData>(
    lattice_parameters: T,
    atoms_data: U,
) -> Option<Vec<CoordData>> {
    let cell_tensor = lattice_parameters.lattice_bases();
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
                .map(|coord| CoordData::Fractional(cell_tensor * coord.raw_data()))
                .collect(),
        )
    }
}

pub fn rotated_lattice_tensor<T: UnitCellParameters>(
    lattice_parameters: &T,
    rotation: Rotation3<f64>,
) -> Matrix3<f64> {
    rotation.matrix() * lattice_parameters.lattice_bases()
}

#[cfg(test)]
mod test {
    use std::f64::consts::FRAC_PI_4;

    use nalgebra::{Matrix3, Point3, Rotation3, Vector3};

    use crate::data::{
        geom::coordinates::CoordData,
        lattice::{LatticeVectors, UnitCellParameters},
    };

    #[test]
    fn test_rotation_of_frac_coord() {
        let p = CoordData::Fractional(Point3::new(
            0.0756034347004260,
            0.0756034355668187,
            0.5000000004346841,
        ));
        #[allow(clippy::excessive_precision)]
        let lattice_vector = LatticeVectors::new(Matrix3::from_columns(&[
            Vector3::new(
                18.931530020488704480_f64,
                -0.000000000000003553_f64,
                0.000000000000000000_f64,
            ),
            Vector3::new(
                -9.465765010246645517_f64,
                16.395185930251127360_f64,
                0.000000000000000000_f64,
            ),
            Vector3::new(
                0.000000000000000000_f64,
                0.000000000000000000_f64,
                9.999213039981000861_f64,
            ),
        ]));
        let rotation = Rotation3::from_axis_angle(&Vector3::z_axis(), FRAC_PI_4);
        let cart_p = lattice_vector.lattice_bases() * p.raw_data();
        let rot_cart_p = rotation.matrix() * cart_p;
        let back_to_frac_p = lattice_vector.lattice_bases().try_inverse().unwrap() * rot_cart_p;
        let rot = lattice_vector.lattice_bases().try_inverse().unwrap()
            * rotation.matrix()
            * lattice_vector.lattice_bases();
        let rot_frac_p = rot * p.raw_data();
        println!(
            "Direct rotate frac_p: {:.3}, cart rotate back to frac_p: {:.3}",
            rot_frac_p, back_to_frac_p
        );
    }
}
