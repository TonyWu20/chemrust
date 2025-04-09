use std::ops::Neg;

const CUSTOM_EPSILON: f64 = 1e-6;

mod irreducible_kpt;
mod kpoint;

pub use irreducible_kpt::IrreducibleKpt;
pub use kpoint::KPoint;

/// Map positive f64 to [0.0, 1.0)
fn map_to_zero_one(x: f64) -> f64 {
    let val = x.rem_euclid(1.0);
    if (val - 1.0).abs() > CUSTOM_EPSILON {
        val
    } else {
        0.0
    }
}

/// Map f64 to (-1.0, 1.0)
fn map_to_neg_one_pos_one(x: f64) -> f64 {
    if x < 0.0 {
        map_to_zero_one(x.abs()).neg()
    } else {
        map_to_zero_one(x)
    }
}

#[cfg(test)]
mod test {

    use std::f64::consts::FRAC_PI_2;

    use nalgebra::{Rotation3, Vector3};

    use crate::{functions::apply_rotation_to_kpt, kpoints::map_to_zero_one};

    use super::KPoint;

    #[test]
    fn map_value() {
        let val: f64 = -1.9;
        dbg!(map_to_zero_one(val.abs()));
    }

    #[test]
    fn new_kpt() {
        let new_coord = Vector3::<f64>::new(1.2, 0.6, 2.5);
        let kpt = KPoint::new(new_coord);
        dbg!(kpt)
            .coord()
            .iter()
            .for_each(|x| debug_assert!((0.0..1.0).contains(x)));
    }
    #[test]
    fn apply_rot() {
        let kpt = KPoint::new(Vector3::new(0.5, 0.2, 0.5));
        let rotation_mat = Rotation3::from_axis_angle(&Vector3::z_axis(), FRAC_PI_2);
        let new_kpt = apply_rotation_to_kpt(rotation_mat.matrix(), &kpt);
        dbg!(new_kpt);
        dbg!(rotation_mat.matrix() * kpt.coord());
    }
}
