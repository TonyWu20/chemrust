use std::{fmt::Debug, marker::PhantomData};

use crate::na::Point3;

use crate::system::data_view::{attributes::AttrBuild, AttrView};

use super::format::DataFormat;

#[derive(Debug)]
pub struct AtomIdMarker;
#[derive(Debug)]
pub struct ElementSymbolMarker;
#[derive(Debug)]
pub struct AtomicNumberMarker;
#[derive(Debug)]
pub struct CartesianCoordMarker;
#[derive(Debug)]
pub struct FractionalCoordMarker;

pub struct AtomAttr<T, N>(T, usize, PhantomData<N>)
where
    N: Debug;

pub type AtomId = AtomAttr<u32, AtomIdMarker>;
pub type ElementSymbol = AtomAttr<String, ElementSymbolMarker>;
pub type AtomicNumber = AtomAttr<u8, AtomicNumberMarker>;
pub type CartesianCoord = AtomAttr<Point3<f64>, CartesianCoordMarker>;
pub type FractionalCoord = AtomAttr<Point3<f64>, FractionalCoordMarker>;

pub struct AtomCollections<T: DataFormat> {
    element_symbols: Vec<ElementSymbol>,
    atomic_number: Vec<AtomicNumber>,
    xyz: Vec<CartesianCoord>,
    fractional_xyz: Option<Vec<FractionalCoord>>,
    atom_ids: Vec<AtomId>,
    format_info: T,
}

impl<T: DataFormat> AtomCollections<T> {
    pub fn element_symbols(&self) -> &[ElementSymbol] {
        self.element_symbols.as_ref()
    }

    pub fn atomic_number(&self) -> &[AtomicNumber] {
        self.atomic_number.as_ref()
    }

    pub fn xyz(&self) -> &[CartesianCoord] {
        self.xyz.as_ref()
    }

    pub fn fractional_xyz(&self) -> Option<&Vec<FractionalCoord>> {
        self.fractional_xyz.as_ref()
    }

    pub fn atom_ids(&self) -> &[AtomId] {
        self.atom_ids.as_ref()
    }

    pub fn format_info(&self) -> &T {
        &self.format_info
    }
}

impl<T, N> AttrView for AtomAttr<T, N>
where
    N: Debug,
{
    type Output = T;

    fn content(&self) -> &Self::Output {
        &self.0
    }

    fn content_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }

    fn index(&self) -> &usize {
        &self.1
    }

    fn set_index(&mut self, new_id: usize) {
        self.1 = new_id;
    }
}

impl<T, N> AttrBuild for AtomAttr<T, N>
where
    N: Debug,
{
    type Input = T;

    type Output = Self;

    fn new(input: Self::Input, index: usize) -> Self::Output {
        Self(input, index, PhantomData)
    }
}

// Unit tests for AtomAttr<T,N>
#[cfg(test)]
mod test {

    use nalgebra::Point3;

    use crate::system::data_view::{attributes::AttrBuild, AttrView};

    use super::{AtomId, AtomicNumber, CartesianCoord, ElementSymbol};

    #[test]
    fn test_attributes() {
        let atom_id = AtomId::new(1_u32, 0);
        let element_symbol = ElementSymbol::new("H".into(), 0);
        let atomic_number = AtomicNumber::new(0, 0);
        let cart_coord = CartesianCoord::new(Point3::new(0_f64, 0_f64, 0_f64), 0);
        assert_eq!(&1_u32, atom_id.content());
        assert_eq!("H", element_symbol.content());
        assert_eq!(&0_u8, atomic_number.content());
        assert_eq!(&Point3::new(0_f64, 0_f64, 0_f64), cart_coord.content());
    }
}
