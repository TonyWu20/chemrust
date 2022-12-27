use std::marker::PhantomData;

use crate::data::{format::DataFormat, lattice::LatticeVectors, AtomCollection};

use super::BuilderState;

#[derive(Debug)]
pub struct LatticeModelBuilder<T: DataFormat, S: BuilderState> {
    lattice_vectors: Option<LatticeVectors<T>>,
    atoms: Option<AtomCollection<T>>,
    state: PhantomData<S>,
}
