use crystallographic_group::SeitzMatrix;
use nalgebra::Matrix3;

pub trait SymmetryOperation {
    fn rotation(&self) -> Matrix3<f64>;
}

impl SymmetryOperation for SeitzMatrix {
    fn rotation(&self) -> Matrix3<f64> {
        self.rotation_part().map(|v| v as f64)
    }
}
