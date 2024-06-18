use crate::data::atom::{Atoms, CoreAtomData};

use crate::data::lattice::cell_param::UnitCellParameters;

use super::cell_param::LatticeVectors;

/// The struct to represent a crystal model structure should implement this trait.
/// # Examples
/// ```
/// use std::f64::consts::FRAC_PI_4;
/// use castep_periodic_table::element::ElementSymbol;
/// use nalgebra::Point3;

/// use chemrust_core::data::{
///     atom::CoreAtomData,
///     geom::coordinates::CoordData,
///     lattice::{
///         cell_param::{CellConstants, UnitCellParameters},
///         CrystalModel,
///     },
/// };
/// let lattice_param = CellConstants::new(9.99, 9.99, 9.99, FRAC_PI_4, FRAC_PI_4, FRAC_PI_4);
/// struct Atoms {
///     indices: Vec<usize>,
///     symbols: Vec<ElementSymbol>,
///     frac: Vec<CoordData>,
///     label: Vec<Option<String>>,
/// }
/// let atoms = Atoms {
///     indices: vec![0, 1],
///     symbols: vec![ElementSymbol::H, ElementSymbol::Li],
///     frac: vec![
///         CoordData::Fractional(Point3::new(0.0, 0.0, 0.0)),
///         CoordData::Fractional(Point3::new(0.5, 0.5, 0.5)),
///     ],
///     label: vec![None, None],
/// };
/// impl CoreAtomData for Atoms {
///     fn indices(&self) -> &[usize] {
///         &self.indices
///     }
///     fn symbols(&self) -> &[ElementSymbol] {
///         &self.symbols
///     }
///     fn coords(&self) -> &[CoordData] {
///         &self.frac
///     }
///     fn labels(&self) -> &[Option<String>] {
///         &self.label
///     }
/// }
/// struct CellStructure {
///     params: CellConstants,
///     atoms: Atoms,
/// }
///
/// impl CrystalModel for CellStructure {
///     fn get_cell_parameters(&self) -> &impl UnitCellParameters {
///             &self.params
///     }
///
///     fn get_atom_data(&self) -> &impl CoreAtomData {
///             &self.atoms
///     }
/// }
/// let cell_struct = CellStructure {
///     params: lattice_param,
///     atoms,
/// };
/// let _cell_tensor = cell_struct.get_cell_parameters().cell_tensor();
/// let _atoms_coords = cell_struct.get_atom_data().coords();
/// ```
pub trait CrystalModel {
    type LatticeData: UnitCellParameters;
    type AtomData: CoreAtomData;
    fn get_cell_parameters(&self) -> &Self::LatticeData;
    fn get_atom_data(&self) -> &Self::AtomData;
    fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData;
    fn get_atom_data_mut(&mut self) -> &mut Self::AtomData;
}

/// Basic example of a crystal model
#[derive(Debug, Clone)]
pub struct LatticeCell {
    lattice_param: LatticeVectors,
    atoms: Atoms,
}

impl LatticeCell {
    pub fn new(lattice_param: LatticeVectors, atoms: Atoms) -> Self {
        Self {
            lattice_param,
            atoms,
        }
    }

    pub fn set_atoms(&mut self, atoms: Atoms) {
        self.atoms = atoms;
    }

    pub fn atoms_mut(&mut self) -> &mut Atoms {
        &mut self.atoms
    }

    pub fn lattice_param(&self) -> LatticeVectors {
        self.lattice_param
    }

    pub fn set_lattice_param(&mut self, lattice_param: LatticeVectors) {
        self.lattice_param = lattice_param;
    }
}

impl CrystalModel for LatticeCell {
    type AtomData = Atoms;
    type LatticeData = LatticeVectors;

    fn get_cell_parameters(&self) -> &Self::LatticeData {
        &self.lattice_param
    }

    fn get_atom_data(&self) -> &Self::AtomData {
        &self.atoms
    }

    fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData {
        &mut self.lattice_param
    }

    fn get_atom_data_mut(&mut self) -> &mut Self::AtomData {
        &mut self.atoms
    }
}
