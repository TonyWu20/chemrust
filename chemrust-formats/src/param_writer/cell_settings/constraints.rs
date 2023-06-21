use std::fmt::Display;

use crate::Cell;

use super::CellSettingExport;

#[derive(Debug, Clone, Copy)]
pub struct FixAllCell(bool);

#[derive(Debug, Clone, Copy)]
pub struct FixCOM(bool);

impl Default for FixAllCell {
    fn default() -> Self {
        Self(true)
    }
}

impl Default for FixCOM {
    fn default() -> Self {
        Self(false)
    }
}

pub struct IonicConstraints {
    lines: Option<Vec<IonicConstraintLine>>,
}

impl Default for IonicConstraints {
    fn default() -> Self {
        Self { lines: None }
    }
}

pub struct IonicConstraintLine {
    id: usize,
    symbol: String,
    id_in_species: usize,
    i: f64,
    j: f64,
    k: f64,
}

impl Display for IonicConstraintLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>6}{:>8}{:>8}{:16.10}{:16.10}{:16.10}",
            self.id, self.symbol, self.id_in_species, self.i, self.j, self.k
        )
    }
}

impl Display for IonicConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = match &self.lines {
            Some(lines) => lines
                .iter()
                .map(|line| format!("{}\n", line))
                .collect::<Vec<String>>()
                .concat(),
            None => "".into(),
        };
        write!(f, "{}", lines)
    }
}

impl CellSettingExport for IonicConstraints {
    fn write_to_cell(&self) -> String {
        Cell::write_block(("IONIC_CONSTRAINTS".into(), format!("{}", self)))
    }
}

impl Display for FixAllCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FIX_ALL_CELL : {}", self.0)
    }
}

impl Display for FixCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FIX_COM : {}", self.0)
    }
}

impl CellSettingExport for FixAllCell {
    fn write_to_cell(&self) -> String {
        format!("{self}")
    }
}

impl CellSettingExport for FixCOM {
    fn write_to_cell(&self) -> String {
        format!("{self}")
    }
}
