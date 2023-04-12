use crate::ModelFormat;

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
