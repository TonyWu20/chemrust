use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use super::units::LengthUnit;

#[derive(Debug, Clone, Copy)]
pub struct IonicPosition {
    symbol: ElementSymbol,
    coord: [f64; 3],
    spin: Option<i32>,
    unit: LengthUnit,
}

impl Display for IonicPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl IonicPosition {
    pub fn new(
        symbol: ElementSymbol,
        coord: [f64; 3],
        spin: Option<i32>,
        unit: LengthUnit,
    ) -> Self {
        Self {
            symbol,
            coord,
            spin,
            unit,
        }
    }
}
