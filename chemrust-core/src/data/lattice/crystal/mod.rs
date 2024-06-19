use crate::data::atom::CoreAtomData;

use crate::data::lattice::cell_param::UnitCellParameters;

/// The struct to represent a crystal model structure should implement this trait.
pub trait CrystalModel {
    type LatticeData: UnitCellParameters;
    type AtomData: CoreAtomData;
    fn get_cell_parameters(&self) -> &Self::LatticeData;
    fn get_atom_data(&self) -> &Self::AtomData;
    fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData;
    fn get_atom_data_mut(&mut self) -> &mut Self::AtomData;
}

// /// Basic example of a crystal model
// #[derive(Debug, Clone)]
// pub struct LatticeCell {
//     lattice_param: LatticeVectors,
//     atoms: Atoms,
// }

// impl LatticeCell {
//     pub fn new(lattice_param: LatticeVectors, atoms: Atoms) -> Self {
//         Self {
//             lattice_param,
//             atoms,
//         }
//     }

//     pub fn set_atoms(&mut self, atoms: Atoms) {
//         self.atoms = atoms;
//     }

//     pub fn atoms_mut(&mut self) -> &mut Atoms {
//         &mut self.atoms
//     }

//     pub fn lattice_param(&self) -> LatticeVectors {
//         self.lattice_param
//     }

//     pub fn set_lattice_param(&mut self, lattice_param: LatticeVectors) {
//         self.lattice_param = lattice_param;
//     }
// }

// impl CrystalModel for LatticeCell {
//     type AtomData = Atoms;
//     type LatticeData = LatticeVectors;

//     fn get_cell_parameters(&self) -> &Self::LatticeData {
//         &self.lattice_param
//     }

//     fn get_atom_data(&self) -> &Self::AtomData {
//         &self.atoms
//     }

//     fn get_cell_parameters_mut(&mut self) -> &mut Self::LatticeData {
//         &mut self.lattice_param
//     }

//     fn get_atom_data_mut(&mut self) -> &mut Self::AtomData {
//         &mut self.atoms
//     }
// }
