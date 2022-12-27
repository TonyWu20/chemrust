use std::{fmt::Debug, marker::PhantomData};

use super::format::DataFormat;

mod attributes;
mod markers;
pub use attributes::*;

#[derive(Debug, Clone)]
pub struct ModelParameters<T: DataFormat> {
    /// List of k-points. Each k-point has xyz and a weight factor.
    pub(crate) kpoints_list: Vec<KPoint<T>>,
    /// An array to specify the grid of k-point used in this model
    pub(crate) kpoints_grid: KPointGrid<T>,
    /// Spacing of k-point.
    pub(crate) kpoints_mp_spacing: Option<KPointMPSpacing<T>>,
    /// Offset of the k-points from the origin.
    pub(crate) kpoints_mp_offset: KPointOffset<T>,
    /// Option in `IONIC_CONSTRAINTS` in cell format
    pub(crate) fix_all_cell: bool,
    /// Option in `IONIC_CONSTRAINTS` in cell format
    pub(crate) fix_com: bool,
    /// Option in `cell` format
    pub(crate) external_efield: EField<T>,
    /// The order is `Rxx`, `Rxy`, `Rxz`, `Ryy`, `Ryz`, `Rzz`
    pub(crate) external_pressure: EPressure<T>,
    /// A parameter in `msi` format
    pub(crate) cry_display: CryDisplay<T>,
    /// A parameter in `msi` format
    pub(crate) periodic_type: PeriodicType<T>,
    /// A parameter in `msi` format
    pub(crate) space_group: SpaceGroup<T>,
    /// A parameter in `msi` format
    pub(crate) cry_tolerance: CryTolerance<T>,
    pub(crate) format_marker: PhantomData<T>,
}

#[cfg(test)]
mod test {
    use crate::data::format::cell::Cell;

    use super::{attributes::KPoint, ModelParameters};

    #[test]
    fn test_param_attr() {
        let kpt = KPoint::<Cell>::default();
        assert_eq!(&[0.0, 0.0, 0.0, 1.0], kpt.content());
    }
    #[test]
    fn test_params() {
        println!("{:?}", ModelParameters::<Cell>::default());
    }
}
