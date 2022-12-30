use std::marker::PhantomData;

use nalgebra::Matrix3;

use crate::system::data_builder::{LatticeModelBuilder, Pending};

use super::{format::DataFormat, param::ModelParameters, AtomCollection};

#[derive(Debug, Clone)]
pub struct LatticeVectors<T: DataFormat> {
    data: Matrix3<f64>,
    format: PhantomData<T>,
}

impl<T> LatticeVectors<T>
where
    T: DataFormat,
{
    pub fn new(data: Matrix3<f64>) -> Self {
        Self {
            data,
            format: PhantomData,
        }
    }

    pub fn data(&self) -> &Matrix3<f64> {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub struct LatticeModel<T: DataFormat> {
    pub(crate) lattice_vectors: Option<LatticeVectors<T>>,
    pub(crate) atoms: AtomCollection<T>,
    pub(crate) settings: ModelParameters<T>,
}

impl<T: DataFormat> LatticeModel<T> {
    pub fn builder() -> LatticeModelBuilder<T, Pending> {
        LatticeModelBuilder::<T, Pending>::new()
    }
    pub fn lattice_vectors(&self) -> Option<&LatticeVectors<T>> {
        self.lattice_vectors.as_ref()
    }

    pub fn atoms(&self) -> &AtomCollection<T> {
        &self.atoms
    }

    pub fn settings(&self) -> &ModelParameters<T> {
        &self.settings
    }

    pub fn atoms_mut(&mut self) -> &mut AtomCollection<T> {
        &mut self.atoms
    }

    pub fn lattice_vectors_mut(&mut self) -> &mut Option<LatticeVectors<T>> {
        &mut self.lattice_vectors
    }
}
