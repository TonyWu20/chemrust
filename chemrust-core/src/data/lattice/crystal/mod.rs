use crate::data::atom::CoreAtomData;

use crate::data::lattice::cell_param::UnitCellParameters;

/// The struct to represent a crystal model structure should implement this trait.
pub trait CrystalModel {
    fn get_cell_parameters(&self) -> &impl UnitCellParameters;
    fn get_atom_data(&self) -> &impl CoreAtomData;
    fn get_cell_parameters_mut(&mut self) -> &mut impl UnitCellParameters;
    fn get_atom_data_mut(&mut self) -> &mut impl CoreAtomData;
}
