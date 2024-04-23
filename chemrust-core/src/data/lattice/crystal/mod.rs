use crate::data::atom::CoreAtomData;

use crate::data::lattice::cell_param::UnitCellParameters;

pub struct CrystalModel<T: UnitCellParameters, U: CoreAtomData> {
    cell_param: T,
    atom_sites: U,
}

impl<T, U> CrystalModel<T, U>
where
    T: UnitCellParameters,
    U: CoreAtomData,
{
    pub fn new(cell_param: T, atom_sites: U) -> Self {
        Self {
            cell_param,
            atom_sites,
        }
    }
    pub fn cell_param(&self) -> &T {
        &self.cell_param
    }

    pub fn atom_sites(&self) -> &U {
        &self.atom_sites
    }
}
