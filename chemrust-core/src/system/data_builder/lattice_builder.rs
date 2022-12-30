use std::marker::PhantomData;

use crate::data::{
    format::DataFormat,
    lattice::{LatticeModel, LatticeVectors},
    param::ModelParameters,
    AtomCollection,
};

use super::{BuilderState, Pending, Ready};

#[derive(Debug)]
pub struct LatticeModelBuilder<T: DataFormat, S: BuilderState> {
    lattice_vectors: Option<LatticeVectors<T>>,
    atoms: Option<AtomCollection<T>>,
    settings: Option<ModelParameters<T>>,
    state: PhantomData<S>,
}

impl<T, S> LatticeModelBuilder<T, S>
where
    T: DataFormat,
    S: BuilderState,
{
    pub fn new() -> LatticeModelBuilder<T, Pending> {
        LatticeModelBuilder {
            lattice_vectors: None,
            atoms: None,
            settings: None,
            state: PhantomData,
        }
    }
}

impl<T: DataFormat> LatticeModelBuilder<T, Pending> {
    pub fn with_vectors(self, lattice_vectors: Option<LatticeVectors<T>>) -> Self {
        Self {
            lattice_vectors,
            ..self
        }
    }
    pub fn with_settings(self, settings: Option<ModelParameters<T>>) -> Self {
        Self { settings, ..self }
    }
    pub fn with_atoms(self, atoms: AtomCollection<T>) -> LatticeModelBuilder<T, Ready> {
        LatticeModelBuilder {
            lattice_vectors: self.lattice_vectors,
            atoms: Some(atoms),
            settings: self.settings,
            state: PhantomData,
        }
    }
}

impl<T: DataFormat> LatticeModelBuilder<T, Ready> {
    pub fn build(self) -> LatticeModel<T> {
        LatticeModel {
            lattice_vectors: self.lattice_vectors,
            atoms: self.atoms.unwrap(),
            settings: ModelParameters::default(),
        }
    }
}
