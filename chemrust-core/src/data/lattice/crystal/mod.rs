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

pub trait SymmetryInfo {
    /// 1-230
    fn get_space_group_it_num(&self) -> u8;
}
