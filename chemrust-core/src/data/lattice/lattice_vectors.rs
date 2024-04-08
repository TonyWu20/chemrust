use std::fmt::Display;

use nalgebra::Matrix3;

#[derive(Debug, Clone)]
pub struct LatticeVectors(Matrix3<f64>);

impl LatticeVectors {
    pub fn new(data: Matrix3<f64>) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &Matrix3<f64> {
        &self.0
    }
    pub fn mat_cart_to_frac(&self) -> Matrix3<f64> {
        self.0.try_inverse().unwrap()
    }
}

impl Display for LatticeVectors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}
