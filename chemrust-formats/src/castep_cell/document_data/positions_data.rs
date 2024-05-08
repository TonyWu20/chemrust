use castep_periodic_table::element::ElementSymbol;

use super::units::LengthUnit;

pub struct IonicPosition {
    symbol: ElementSymbol,
    coord: [f64; 3],
    spin: Option<i32>,
    unit: LengthUnit,
}
