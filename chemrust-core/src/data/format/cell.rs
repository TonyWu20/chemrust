use castep_periodic_table::element::LookupElement;

use crate::cpt::data::ELEMENT_TABLE;

use crate::data::AtomCollections;
use crate::{
    data::{ElementSymbol, FractionalCoord},
    impl_display,
};
use std::fmt::Display;

use super::{BlockWriter, DataFormat};

#[derive(Debug, Clone, Default)]
pub struct Cell;

impl DataFormat for Cell {}

impl BlockWriter for Cell {
    fn format_block(block_name: &str, block_content: &str) -> String {
        format!(
            "%BLOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, block_content, block_name
        )
    }
}

impl_display!(ElementSymbol<Cell>, "{:>3}");
impl_display!(FractionalCoord<Cell>, "{:20.16}{:20.16}{:20.16}", x, y, z);

impl Display for AtomCollections<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_positions_str: Vec<String> = self
            .element_symbols()
            .iter()
            // In `AtomCollections<Cell>`, `fractional_xyz` is guaranteed to be `Some`.
            .zip(self.fractional_xyz().unwrap().iter())
            .map(|(symbol, frac_xyz)| -> String {
                let spin = ELEMENT_TABLE
                    .get_by_symbol(symbol.content())
                    .unwrap()
                    .spin();
                if spin > 0 {
                    format!("{}{} SPIN={:14.10}", symbol, frac_xyz, spin)
                } else {
                    format!("{}{}", symbol, frac_xyz)
                }
            })
            .collect();
        write!(f, "{}", all_positions_str.join("\n"))
    }
}
