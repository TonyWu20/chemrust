use crate::data::lattice::UnitCellParameters;

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
