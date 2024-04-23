use super::unit_cell::{CellConstants, UnitCellParameters};

#[derive(Debug, Clone, Copy)]
pub struct ReciprocalCell {
    pub(crate) recip_a: f64,
    pub(crate) recip_b: f64,
    pub(crate) recip_c: f64,
    pub(crate) recip_alpha: f64,
    pub(crate) recip_beta: f64,
    pub(crate) recip_gamma: f64,
}

impl From<CellConstants> for ReciprocalCell {
    fn from(value: CellConstants) -> Self {
        let CellConstants {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = value;
        let volume = value.cell_volume();
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

impl Into<CellConstants> for ReciprocalCell {
    fn into(self) -> CellConstants {
        todo!()
    }
}
