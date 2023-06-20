use std::fmt::Display;

use crate::Cell;

use super::CellSettingExport;

#[derive(Debug, Clone, Copy)]
pub struct ExternalEField {
    ex: f64,
    ey: f64,
    ez: f64,
}

impl Default for ExternalEField {
    fn default() -> Self {
        Self {
            ex: 0.0,
            ey: 0.0,
            ez: 0.0,
        }
    }
}

impl Display for ExternalEField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:16.10}{:16.10}{:16.10}", self.ex, self.ey, self.ez)
    }
}

impl CellSettingExport for ExternalEField {
    fn write_to_cell(&self) -> String {
        Cell::write_block(("EXTERNAL_EFIELD".into(), format!("{self}")))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExternalPressure {
    rxx: f64,
    rxy: f64,
    rxz: f64,
    ryy: f64,
    ryz: f64,
    rzz: f64,
}

impl Default for ExternalPressure {
    fn default() -> Self {
        Self {
            rxx: 0.0,
            rxy: 0.0,
            rxz: 0.0,
            ryy: 0.0,
            ryz: 0.0,
            rzz: 0.0,
        }
    }
}

impl Display for ExternalPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:16.10}{:16.10}{:16.10}\n{:32.10}{:16.10}\n{:48.10}",
            self.rxx, self.rxy, self.rxz, self.ryy, self.ryz, self.rzz
        )
    }
}

impl CellSettingExport for ExternalPressure {
    fn write_to_cell(&self) -> String {
        Cell::write_block(("EXTERNAL_PRESSURE".into(), format!("{self}")))
    }
}

#[cfg(test)]
mod test {
    use crate::param_writer::cell_settings::CellSettingExport;

    use super::ExternalPressure;

    #[test]
    fn extern_field_disp() {
        let pressure = ExternalPressure::default();
        println!("{}", pressure.write_to_cell());
    }
}
