use std::marker::PhantomData;

use nalgebra::Matrix3;

use super::{format::DataFormat, parameters::ModelParameters, AtomCollections};

#[derive(Debug, Clone)]
pub struct LatticeVectors<T> {
    data: Matrix3<f64>,
    format: PhantomData<T>,
}

impl<T> LatticeVectors<T> {
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
    lattice_vectors: Option<LatticeVectors<T>>,
    atoms: AtomCollections<T>,
    settings: ModelParameters<T>,
}

impl<T: DataFormat> LatticeModel<T> {
    pub fn lattice_vectors(&self) -> Option<&LatticeVectors<T>> {
        self.lattice_vectors.as_ref()
    }

    pub fn atoms(&self) -> &AtomCollections<T> {
        &self.atoms
    }

    pub fn settings(&self) -> &ModelParameters<T> {
        &self.settings
    }

    pub fn atoms_mut(&mut self) -> &mut AtomCollections<T> {
        &mut self.atoms
    }

    pub fn lattice_vectors_mut(&mut self) -> &mut Option<LatticeVectors<T>> {
        &mut self.lattice_vectors
    }
}
