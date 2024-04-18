use std::fmt::Display;

use nalgebra::Matrix3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LatticeVectors(Matrix3<f32>);

impl LatticeVectors {
    pub fn new(data: Matrix3<f32>) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &Matrix3<f32> {
        &self.0
    }
    pub fn mat_cart_to_frac(&self) -> Option<Matrix3<f32>> {
        self.0.try_inverse()
    }
}

impl Display for LatticeVectors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}
