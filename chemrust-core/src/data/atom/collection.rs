use std::fmt::Debug;

use crate::system::data_view::AttrCollectionView;

use crate::data::format::DataFormat;

use super::{
    AtomAttr, AtomAttrMarker, AtomId, AtomicNumber, CartesianCoord, ElementSymbol, FractionalCoord,
};
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

impl<T, N, F, U, M> AttrCollectionView<Vec<AtomAttr<U, M, F>>> for Vec<AtomAttr<T, N, F>>
where
    T: PartialEq + Clone,
    N: AtomAttrMarker,
    F: DataFormat,
    U: PartialEq + Clone,
    M: AtomAttrMarker,
{
    type Ref = Vec<AtomAttr<U, M, F>>;
    type Output = T;
    type NewCollection = Self;

    fn view_content_at(&self, index: usize) -> Option<&Self::Output> {
        Some(self.get(index).as_ref()?.content())
    }

    fn rearrange_with(self, other: &Self::Ref) -> Self::NewCollection {
        let mut new_vec: Vec<AtomAttr<T, N, F>> = Vec::with_capacity(self.len());
        for item in other.iter() {
            let reference_index: &usize = item.index();
            let mapped_item = &self[*reference_index];
            new_vec.push(mapped_item.clone())
        }
        new_vec
    }
    fn update_all_index(&mut self) {
        self.iter_mut()
            .enumerate()
            .for_each(|(i, item)| item.set_index(i));
    }
}
