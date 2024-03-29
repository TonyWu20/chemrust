use std::{collections::HashSet, fmt::Display};

use nalgebra::Matrix3;

use super::Atom;

mod reciprocal_lattice;

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
pub struct BasicLatticeModel {
    pub(crate) lattice_vectors: Option<LatticeVectors>,
    pub(crate) atoms: Vec<Atom>,
}

#[derive(Debug, Clone, Copy)]
pub struct FractionalCoordRange(f64, f64);

impl Display for FractionalCoordRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl FractionalCoordRange {
    /// Creates a `FractionalCoordRange` with the range restricted to `[0.0, 1.0]`
    /// # Panics
    /// Panics if `lower > higher`
    pub fn new(lower: f64, higher: f64) -> Self {
        assert!(lower <= higher);
        Self(lower.clamp(0.0, 1.0), higher.clamp(0.0, 1.0))
    }
    fn is_in_range(&self, value: f64) -> bool {
        if self.0 <= value && self.1 >= value {
            true
        } else {
            false
        }
    }
    pub fn min(&self) -> f64 {
        self.0
    }
    pub fn max(&self) -> f64 {
        self.1
    }
}

impl BasicLatticeModel {
    pub fn new(lattice_vectors: &Option<LatticeVectors>, atoms: &[Atom]) -> Self {
        BasicLatticeModel {
            lattice_vectors: lattice_vectors.clone(),
            atoms: atoms.to_vec(),
        }
    }
    pub fn number_of_atoms(&self) -> usize {
        self.atoms.len()
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

    pub fn append_atom(&mut self, atoms: &mut Vec<Atom>) {
        self.atoms.append(atoms)
    }

    pub fn lattice_vectors_mut(&mut self) -> &mut Option<LatticeVectors> {
        &mut self.lattice_vectors
    }

    pub fn set_lattice_vectors(&mut self, lattice_vectors: Option<LatticeVectors>) {
        self.lattice_vectors = lattice_vectors;
    }

    pub fn set_atoms(&mut self, atoms: Vec<Atom>) {
        self.atoms = atoms;
    }
    pub fn element_list(&self) -> Vec<String> {
        let all_atom_elements = self
            .atoms
            .iter()
            .map(|atom| (atom.atomic_number(), atom.symbol()))
            .collect::<Vec<(u8, &str)>>()
            .drain(..)
            .collect::<HashSet<(u8, &str)>>();
        let mut element_list: Vec<(u8, &str)> = all_atom_elements.into_iter().collect();
        element_list.sort_by(|a, b| a.0.cmp(&b.0));
        element_list
            .iter()
            .map(|(_, symbol)| symbol.to_string())
            .collect()
    }
    /// Select atoms by the given ranges of x, y, z in fractional coordinates
    pub fn xyz_range_filter(
        &self,
        x_range: FractionalCoordRange,
        y_range: FractionalCoordRange,
        z_range: FractionalCoordRange,
    ) -> Vec<Atom> {
        self.atoms
            .iter()
            .filter(|&atom| {
                let frac_coord = self.lattice_vectors.as_ref().unwrap().mat_cart_to_frac()
                    * atom.cartesian_coord();
                if x_range.is_in_range(frac_coord.x)
                    && y_range.is_in_range(frac_coord.y)
                    && z_range.is_in_range(frac_coord.z)
                {
                    true
                } else {
                    false
                }
            })
            .cloned()
            .collect::<Vec<Atom>>()
    }
}

impl Display for BasicLatticeModel {
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
