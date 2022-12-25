use std::{fmt::Debug, marker::PhantomData};

use super::format::DataFormat;

pub trait ParamMarker: Debug {}

#[derive(Debug, Clone)]
pub struct KPointMark;
#[derive(Debug, Clone)]
pub struct KPointGridMark;
#[derive(Debug, Clone)]
pub struct KPointSpacingMark;
#[derive(Debug, Clone)]
pub struct KPointOffsetMark;
#[derive(Debug, Clone)]
pub struct EFieldMark;
#[derive(Debug, Clone)]
pub struct EPressureMark;
#[derive(Debug, Clone)]
pub struct CryDisplayMark;
#[derive(Debug, Clone)]
pub struct PeriodicTypeMark;
#[derive(Debug, Clone)]
pub struct SpaceGroupMark;
#[derive(Debug, Clone)]
pub struct CryTolMark;

#[macro_export]
macro_rules! impl_param_marker {
    ($($type: ty), *) => {
        $(impl ParamMarker for $type{})*
    };
}

impl_param_marker!(
    KPointMark,
    KPointGridMark,
    KPointOffsetMark,
    KPointSpacingMark,
    EFieldMark,
    EPressureMark,
    CryDisplayMark,
    CryTolMark,
    PeriodicTypeMark,
    SpaceGroupMark
);

#[derive(Debug, Clone)]
pub struct ParamAttr<T, M: ParamMarker, F: DataFormat>(T, PhantomData<M>, PhantomData<F>)
where
    M: ParamMarker,
    F: DataFormat;

#[macro_export]
macro_rules! def_param_type {
    ($(($type_name: ident,$type: ty,  $marker: ty)), *) => {
       $(type $type_name<T> = ParamAttr<$type, $marker,T>;)*
    };
}

def_param_type!(
    (KPoint, [f64; 4], KPointMark),
    (KPointGrid, [u8; 3], KPointGridMark),
    (KPointMPSpacing, f64, KPointSpacingMark),
    (KPointOffset, [f64; 3], KPointOffsetMark),
    (EField, [f64; 3], EFieldMark),
    (EPressure, [f64; 6], EPressureMark),
    (CryDisplay, (u32, u32), CryDisplayMark),
    (PeriodicType, u8, PeriodicTypeMark),
    (SpaceGroup, String, SpaceGroupMark),
    (CryTolerance, f64, CryTolMark)
);

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
    format_marker: T,
}
