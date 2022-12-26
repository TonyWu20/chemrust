use std::{fmt::Debug, marker::PhantomData};

use nalgebra::Point3;

use crate::data::{atom::AtomAttrMarker, format::DataFormat};

use super::{
    AtomIdMarker, AtomicNumberMarker, CartesianCoordMarker, ElementSymbolMarker,
    FractionalCoordMarker,
};

#[derive(Clone)]
pub struct AtomAttr<T, N, F>(T, usize, PhantomData<N>, PhantomData<F>)
where
    T: PartialEq + Clone,
    N: AtomAttrMarker,
    F: DataFormat;

pub type AtomId<T> = AtomAttr<u32, AtomIdMarker, T>;
pub type ElementSymbol<T> = AtomAttr<String, ElementSymbolMarker, T>;
pub type AtomicNumber<T> = AtomAttr<u8, AtomicNumberMarker, T>;
pub type CartesianCoord<T> = AtomAttr<Point3<f64>, CartesianCoordMarker, T>;
pub type FractionalCoord<T> = AtomAttr<Point3<f64>, FractionalCoordMarker, T>;

impl<T, N, F> Debug for AtomAttr<T, N, F>
where
    T: PartialEq + Clone + Debug,
    N: AtomAttrMarker,
    F: DataFormat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AtomAttr")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T, N, F> PartialEq for AtomAttr<T, N, F>
where
    T: PartialEq + Clone,
    N: AtomAttrMarker,
    F: DataFormat,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl<T, N, F> PartialOrd for AtomAttr<T, N, F>
where
    T: PartialEq + Clone,
    N: AtomAttrMarker,
    F: DataFormat,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T, N, F> AtomAttr<T, N, F>
where
    T: PartialEq + Clone,
    N: AtomAttrMarker,
    F: DataFormat,
{
    pub fn new(input: T, index: usize) -> Self {
        Self(input, index, PhantomData, PhantomData)
    }
    pub fn content(&self) -> &T {
        &self.0
    }

    pub fn content_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn index(&self) -> &usize {
        &self.1
    }

    pub fn set_index(&mut self, new_id: usize) {
        self.1 = new_id;
    }
}
