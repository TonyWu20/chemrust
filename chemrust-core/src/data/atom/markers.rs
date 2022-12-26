use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct AtomIdMarker;
#[derive(Debug, Clone, Copy)]
pub struct ElementSymbolMarker;
#[derive(Debug, Clone, Copy)]
pub struct AtomicNumberMarker;
#[derive(Debug, Clone, Copy)]
pub struct CartesianCoordMarker;
#[derive(Debug, Clone, Copy)]
pub struct FractionalCoordMarker;

pub trait AtomAttrMarker: Debug + Clone {}
macro_rules! impl_marker {
    ($($x: ty), *) => {
        $(impl AtomAttrMarker for $x{})*
    };
}
impl_marker!(
    AtomIdMarker,
    ElementSymbolMarker,
    AtomicNumberMarker,
    CartesianCoordMarker,
    FractionalCoordMarker
);
