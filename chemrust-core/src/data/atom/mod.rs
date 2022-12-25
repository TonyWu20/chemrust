use std::{fmt::Debug, marker::PhantomData};

use crate::na::Point3;

use crate::system::data_view::{attributes::AttrBuild, AttrView};

use super::format::DataFormat;

#[derive(Debug, Clone)]
pub struct AtomIdMarker;
#[derive(Debug, Clone)]
pub struct ElementSymbolMarker;
#[derive(Debug, Clone)]
pub struct AtomicNumberMarker;
#[derive(Debug, Clone)]
pub struct CartesianCoordMarker;
#[derive(Debug, Clone)]
pub struct FractionalCoordMarker;

#[derive(Debug, Clone)]
pub struct AtomAttr<T, N, F>(T, usize, PhantomData<N>, PhantomData<F>)
where
    N: Debug,
    F: DataFormat;

pub type AtomId<T> = AtomAttr<u32, AtomIdMarker, T>;
pub type ElementSymbol<T> = AtomAttr<String, ElementSymbolMarker, T>;
pub type AtomicNumber<T> = AtomAttr<u8, AtomicNumberMarker, T>;
pub type CartesianCoord<T> = AtomAttr<Point3<f64>, CartesianCoordMarker, T>;
pub type FractionalCoord<T> = AtomAttr<Point3<f64>, FractionalCoordMarker, T>;

#[derive(Debug, Clone)]
pub struct AtomCollections<T: DataFormat> {
    element_symbols: Vec<ElementSymbol<T>>,
    atomic_number: Vec<AtomicNumber<T>>,
    xyz: Vec<CartesianCoord<T>>,
    fractional_xyz: Option<Vec<FractionalCoord<T>>>,
    atom_ids: Vec<AtomId<T>>,
    format_info: T,
}

impl<T: DataFormat> AtomCollections<T> {
    pub fn element_symbols(&self) -> &[ElementSymbol<T>] {
        self.element_symbols.as_ref()
    }

    pub fn atomic_number(&self) -> &[AtomicNumber<T>] {
        self.atomic_number.as_ref()
    }

    pub fn xyz(&self) -> &[CartesianCoord<T>] {
        self.xyz.as_ref()
    }

    pub fn fractional_xyz(&self) -> Option<&Vec<FractionalCoord<T>>> {
        self.fractional_xyz.as_ref()
    }

    pub fn atom_ids(&self) -> &[AtomId<T>] {
        self.atom_ids.as_ref()
    }

    pub fn format_info(&self) -> &T {
        &self.format_info
    }
}

impl<T, N, F> AttrView for AtomAttr<T, N, F>
where
    N: Debug,
    F: DataFormat,
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

impl<T, N, F> AttrBuild for AtomAttr<T, N, F>
where
    N: Debug,
    F: DataFormat,
{
    type Input = T;

    type Output = Self;

    fn new(input: Self::Input, index: usize) -> Self::Output {
        Self(input, index, PhantomData, PhantomData)
    }
}

// Unit tests for AtomAttr<T,N>
#[cfg(test)]
mod test {

    use nalgebra::Point3;

    use crate::{
        data::format::msi::Msi,
        system::data_view::{attributes::AttrBuild, AttrView},
    };

    use super::{AtomId, AtomicNumber, CartesianCoord, ElementSymbol};

    #[test]
    fn test_attributes() {
        let atom_id: AtomId<Msi> = AtomId::new(1_u32, 0);
        let element_symbol: ElementSymbol<Msi> = ElementSymbol::new("H".into(), 0);
        let atomic_number: AtomicNumber<Msi> = AtomicNumber::new(0, 0);
        let cart_coord: CartesianCoord<Msi> =
            CartesianCoord::new(Point3::new(0_f64, 0_f64, 0_f64), 0);
        assert_eq!(&1_u32, atom_id.content());
        assert_eq!("H", element_symbol.content());
        assert_eq!(&0_u8, atomic_number.content());
        assert_eq!(&Point3::new(0_f64, 0_f64, 0_f64), cart_coord.content());
    }
}
