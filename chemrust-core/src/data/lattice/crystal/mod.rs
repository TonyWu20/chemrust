use crate::data::{atom::AtomDataArray, geom::coordinates::CoordData};

use super::Cell;

pub struct CrystalModel<T: AtomDataArray> {
    cell_param: Cell,
    atom_sites: T,
}

impl<T: AtomDataArray> CrystalModel<T> {
    pub fn new(cell_param: Cell, atom_sites: T) -> Self {
        Self {
            cell_param,
            atom_sites,
        }
    }
    /// Convert `Vec<CoordData::Fractional>` to `Vec<CoordData::Cartesian>`
    pub fn cartesian_coord(&self) -> Vec<CoordData> {
        let cell_vectors = self.cell_param.matrix_repr();
        self.atom_sites
            .coord()
            .iter()
            .map(|frac_coord| CoordData::frac_to_coord(frac_coord, &cell_vectors))
            .collect()
    }
}
