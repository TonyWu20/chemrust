use std::{fmt::Display, ops::Neg};

use nalgebra::Vector3;

const CUSTOM_EPSILON: f64 = 1e-6;

#[derive(Debug, Clone, Copy)]
pub struct KPoint(Vector3<f64>);

impl KPoint {
    /// Always map the coordinate to interval [0, 1)
    pub fn new(kpt_coord: Vector3<f64>) -> Self {
        Self(kpt_coord.map(map_to_neg_one_pos_one))
    }
    pub fn coord(&self) -> &Vector3<f64> {
        &self.0
    }
    pub(crate) fn inv(&self) -> KPoint {
        KPoint(-self.0)
    }
}

impl Display for KPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.15} {:.15} {:.15}", self.0.x, self.0.y, self.0.z)
    }
}

impl PartialOrd for KPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0
            .iter()
            .zip(other.0.iter())
            .find_map(|(a, b)| {
                let diff = a - b;
                if diff.abs() > CUSTOM_EPSILON {
                    a.partial_cmp(b)
                } else {
                    None
                }
            })
            .or(Some(std::cmp::Ordering::Equal))
    }
}

impl PartialEq for KPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| {
            // Allow full inversed point to be equal, e.g. (x, y, z) and (-x, -y, -z)
            (a - b).abs() < CUSTOM_EPSILON || (a + b).abs() < CUSTOM_EPSILON
        })
    }
}

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

    use crate::{functions::apply_rotation_to_kpt, kpoint::map_to_zero_one};

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
