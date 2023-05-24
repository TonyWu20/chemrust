use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use crate::{ModelFormat, StructureFile};

mod cell_param;

#[derive(Debug, Clone, Default)]
/// A unit struct to mark `cell`format.
pub struct Cell;

impl ModelFormat for Cell {}

/// Methods for `CellFormat`
impl Cell {
    pub fn write_block(block: (String, String)) -> String {
        let (block_name, content) = block;
        format!(
            "%BlOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, content, block_name
        )
    }
}

impl StructureFile<Cell> {
    pub fn write_atoms(&self) -> String {
        let cart_to_frac_matrix = self
            .lattice_model
            .lattice_vectors()
            .unwrap()
            .mat_cart_to_frac();
        let atom_frac_coords_text: Vec<String> = self
            .lattice_model
            .atoms()
            .iter()
            .map(|atom| {
                let frac_xyz = cart_to_frac_matrix * atom.cartesian_coord();
                let symbol = atom.symbol();
                let spin = ELEMENT_TABLE.get_by_symbol(symbol).unwrap().spin();
                let spin_str = if spin > 0 {
                    format!(" SPIN={:14.10}", spin)
                } else {
                    "".into()
                };
                format!(
                    "{:>3}{:20.16}{:20.16}{:20.16}{spin_str}",
                    symbol, frac_xyz.x, frac_xyz.y, frac_xyz.z
                )
            })
            .collect();
        atom_frac_coords_text.join("\n")
    }
}
