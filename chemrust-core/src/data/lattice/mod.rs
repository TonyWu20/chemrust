use std::fmt::Display;

use nalgebra::Matrix3;

use super::Atom;

#[derive(Debug, Clone)]
pub struct LatticeVectors(Matrix3<f64>);

impl LatticeVectors {
    pub fn new(data: Matrix3<f64>) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &Matrix3<f64> {
        &self.0
    }
    pub fn mat_cart_to_frac(&self) -> Matrix3<f64> {
        self.0.try_inverse().unwrap()
    }
}

impl Display for LatticeVectors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct LatticeModel {
    pub(crate) lattice_vectors: Option<LatticeVectors>,
    pub(crate) atoms: Vec<Atom>,
}

impl LatticeModel {
    pub fn new(lattice_vectors: &Option<LatticeVectors>, atoms: &[Atom]) -> Self {
        LatticeModel {
            lattice_vectors: lattice_vectors.clone(),
            atoms: atoms.to_vec(),
        }
    }
    // pub fn builder() -> LatticeModelBuilder<T, Pending> {
    //     LatticeModelBuilder::<T, Pending>::new()
    // }
    pub fn lattice_vectors(&self) -> Option<&LatticeVectors> {
        self.lattice_vectors.as_ref()
    }

    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    pub fn atoms_mut(&mut self) -> &mut Vec<Atom> {
        &mut self.atoms
    }

    pub fn lattice_vectors_mut(&mut self) -> &mut Option<LatticeVectors> {
        &mut self.lattice_vectors
    }
}

impl Display for LatticeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lattice_vector_disp = if let Some(v) = &self.lattice_vectors {
            format!("{}", v)
        } else {
            "N/A".to_string()
        };
        let atoms_text_vec: Vec<String> = self.atoms.iter().map(|atom| format!("{atom}")).collect();
        let atoms_text = atoms_text_vec.join("\n");
        write!(
            f,
            "Lattice Vectors: {}\n{}",
            lattice_vector_disp, atoms_text
        )
    }
}
