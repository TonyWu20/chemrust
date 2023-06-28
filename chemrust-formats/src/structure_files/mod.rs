use crate::ModelFormat;
use chemrust_core::data::lattice::LatticeModel;
use std::{io, marker::PhantomData, path::Path};

mod cell;
mod msi;

pub use cell::Cell;
pub use msi::Msi;

#[derive(Debug, Clone)]
pub struct StructureFile<T: ModelFormat> {
    lattice_model: LatticeModel,
    format: PhantomData<T>,
}

impl<T: ModelFormat> StructureFile<T> {
    pub fn new(lattice_model: LatticeModel) -> Self {
        Self {
            lattice_model,
            format: PhantomData,
        }
    }

    pub fn lattice_model(&self) -> &LatticeModel {
        &self.lattice_model
    }
}

pub trait FileExport {
    fn write_to<P: AsRef<Path>>(&self, path: &P) -> Result<(), io::Error>;
}

impl<T: ModelFormat> From<LatticeModel> for StructureFile<T> {
    fn from(value: LatticeModel) -> Self {
        Self {
            lattice_model: value,
            format: PhantomData,
        }
    }
}
