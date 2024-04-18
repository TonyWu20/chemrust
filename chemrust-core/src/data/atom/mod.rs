use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use self::builder::AtomBuilder;

use super::geom::coordinates::{Coordinate, CoordinateType};

mod builder;

/// Essential data to record an atom.
/// Other complementary informations can be generated based on `symbol`
/// or wrapped in a newtype.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Atom<T: CoordinateType> {
    /// The element symbol of the atom
    symbol: ElementSymbol,
    /// Coordinate of the atom:
    coord: Coordinate<T>,
    index: usize,
    label: Option<String>,
}

/// Only methods for `getter`
impl<T: CoordinateType> Atom<T> {
    pub fn new_builder() -> AtomBuilder<T> {
        AtomBuilder::default()
    }

    pub fn symbol(&self) -> &ElementSymbol {
        &self.symbol
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn coord(&self) -> Coordinate<T> {
        self.coord
    }
    pub fn label(&self) -> &Option<String> {
        &self.label
    }
}

impl<T: CoordinateType> Display for Atom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Atom {}:
  Element Symbol: {:?}
  XYZ: {:#}"#,
            self.index + 1,
            self.symbol,
            self.coord.xyz()
        )
    }
}
