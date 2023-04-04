use std::marker::PhantomData;
mod error;
pub use error::InconsistentLength;

use crate::data::{
    format::DataFormat, AtomCollection, AtomId, AtomicNumber, CartesianCoord, ElementSymbol,
    FractionalCoord,
};

use crate::builder_state::{BuilderState, Pending, Ready};

pub struct AtomCollectionBuilder<T, S>
where
    T: DataFormat,
    S: BuilderState,
{
    size: usize,
    element_symbols: Option<Vec<ElementSymbol<T>>>,
    atomic_number: Option<Vec<AtomicNumber<T>>>,
    xyz: Option<Vec<CartesianCoord<T>>>,
    fractional_xyz: Option<Vec<FractionalCoord<T>>>,
    atom_ids: Option<Vec<AtomId<T>>>,
    state: PhantomData<S>,
}

impl<T, S> AtomCollectionBuilder<T, S>
where
    T: DataFormat,
    S: BuilderState,
{
    pub fn new(size: usize) -> AtomCollectionBuilder<T, Pending> {
        AtomCollectionBuilder {
            size,
            element_symbols: None,
            atomic_number: None,
            xyz: None,
            fractional_xyz: None,
            atom_ids: None,
            state: PhantomData,
        }
    }
}

impl<T> AtomCollectionBuilder<T, Pending>
where
    T: DataFormat,
{
    fn size_check(&self, input_size: usize) -> Result<(), InconsistentLength> {
        if self.size != input_size {
            Err(InconsistentLength {
                curr: input_size,
                expect: self.size,
            })
        } else {
            Ok(())
        }
    }
    pub fn with_symbols(self, symbols: &[ElementSymbol<T>]) -> Result<Self, InconsistentLength> {
        self.size_check(symbols.len())?;
        Ok(Self {
            element_symbols: Some(symbols.to_vec()),
            ..self
        })
    }
    pub fn with_atomic_number(
        self,
        atomic_nums: &[AtomicNumber<T>],
    ) -> Result<Self, InconsistentLength> {
        self.size_check(atomic_nums.len())?;
        Ok(Self {
            atomic_number: Some(atomic_nums.to_vec()),
            ..self
        })
    }
    pub fn with_xyz(self, xyz_coords: &[CartesianCoord<T>]) -> Result<Self, InconsistentLength> {
        self.size_check(xyz_coords.len())?;
        Ok(Self {
            xyz: Some(xyz_coords.to_vec()),
            ..self
        })
    }
    pub fn with_frac_xyz(
        self,
        frac_coords: &[FractionalCoord<T>],
    ) -> Result<Self, InconsistentLength> {
        self.size_check(frac_coords.len())?;
        Ok(Self {
            fractional_xyz: Some(frac_coords.to_vec()),
            ..self
        })
    }
    pub fn with_atom_ids(self, atom_ids: &[AtomId<T>]) -> Result<Self, InconsistentLength> {
        self.size_check(atom_ids.len())?;
        Ok(Self {
            atom_ids: Some(atom_ids.to_vec()),
            ..self
        })
    }
    pub fn finish(self) -> AtomCollectionBuilder<T, Ready> {
        let Self {
            size,
            element_symbols,
            atomic_number,
            xyz,
            fractional_xyz,
            atom_ids,
            state: _,
        } = self;
        AtomCollectionBuilder {
            size,
            element_symbols,
            atomic_number,
            xyz,
            fractional_xyz,
            atom_ids,
            state: PhantomData,
        }
    }
}

impl<T> AtomCollectionBuilder<T, Ready>
where
    T: DataFormat,
{
    pub fn build(self) -> AtomCollection<T> {
        let Self {
            size,
            element_symbols,
            atomic_number,
            xyz,
            fractional_xyz,
            atom_ids,
            state: _,
        } = self;
        AtomCollection {
            element_symbols: element_symbols.unwrap(),
            atomic_number: atomic_number.unwrap(),
            xyz: xyz.unwrap(),
            fractional_xyz,
            atom_ids: atom_ids.unwrap(),
            size,
        }
    }
}
