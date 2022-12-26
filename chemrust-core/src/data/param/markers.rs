use std::fmt::Debug;
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
