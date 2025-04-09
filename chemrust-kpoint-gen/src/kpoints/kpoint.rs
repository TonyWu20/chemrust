use std::fmt::Display;

use super::map_to_neg_one_pos_one;
use super::CUSTOM_EPSILON;
use nalgebra::Vector3;

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
