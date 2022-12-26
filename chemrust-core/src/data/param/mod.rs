use std::{fmt::Debug, marker::PhantomData};

use super::format::DataFormat;

mod attributes;
mod markers;
use attributes::*;

#[derive(Debug, Clone)]
pub struct ModelParameters<T: DataFormat> {
    /// List of k-points. Each k-point has xyz and a weight factor.
    kpoints_list: Vec<KPoint<T>>,
    /// An array to specify the grid of k-point used in this model
    kpoints_grid: KPointGrid<T>,
    /// Spacing of k-point.
    kpoints_mp_spacing: Option<KPointMPSpacing<T>>,
    /// Offset of the k-points from the origin.
    kpoints_mp_offset: KPointOffset<T>,
    /// Option in `IONIC_CONSTRAINTS` in cell format
    fix_all_cell: bool,
    /// Option in `IONIC_CONSTRAINTS` in cell format
    fix_com: bool,
    /// Option in `cell` format
    external_efield: EField<T>,
    /// The order is `Rxx`, `Rxy`, `Rxz`, `Ryy`, `Ryz`, `Rzz`
    external_pressure: EPressure<T>,
    /// A parameter in `msi` format
    cry_display: CryDisplay<T>,
    /// A parameter in `msi` format
    periodic_type: PeriodicType<T>,
    /// A parameter in `msi` format
    space_group: SpaceGroup<T>,
    /// A parameter in `msi` format
    cry_tolerance: CryTolerance<T>,
    format_marker: PhantomData<T>,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_param_attr() {
        todo!()
    }
}
