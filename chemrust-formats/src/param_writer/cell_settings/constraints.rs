use std::fmt::Display;

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

pub struct IonicConstraintLine {
    symbol: String,
    id_in_species: usize,
    i: f64,
    j: f64,
    k: f64,
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
