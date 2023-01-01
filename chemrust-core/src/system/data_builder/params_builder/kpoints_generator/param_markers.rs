use crate::data::{
    format::DataFormat,
    param::{KPoint, KPointGrid, KPointMPSpacing},
};

/// For `IsMetal`
pub trait IsMetal {}
pub struct Yes;
pub struct No;

impl IsMetal for Yes {}
impl IsMetal for No {}

/// Sampling quality
pub trait SamplingQuality {}
pub struct Coarse;
pub struct Medium;
pub struct Fine;

impl SamplingQuality for Coarse {}
impl SamplingQuality for Medium {}
impl SamplingQuality for Fine {}

/// Different types of Kpoint parameter
pub trait KPointParam {}
pub trait Spacing: KPointParam {}
pub trait Grid: KPointParam {}
pub trait List: KPointParam {}

impl<T: DataFormat> KPointParam for KPointMPSpacing<T> {}
impl<T: DataFormat> KPointParam for KPointGrid<T> {}
impl<T: DataFormat> KPointParam for Vec<KPoint<T>> {}
impl<T: DataFormat> Spacing for KPointMPSpacing<T> {}
impl<T: DataFormat> Grid for KPointGrid<T> {}
impl<T: DataFormat> List for Vec<KPoint<T>> {}
