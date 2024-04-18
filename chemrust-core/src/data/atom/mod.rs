use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use self::builder::AtomBuilder;

use super::geom::coordinates::CoordData;

mod builder;

/// Essential data to record an atom.
/// Other complementary informations can be generated based on `symbol`
/// or wrapped in a newtype.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Atom {
    /// The element symbol of the atom
    symbol: ElementSymbol,
    /// Coordinate of the atom:
    coord: CoordData,
    index: usize,
    label: Option<String>,
}

/// Only methods for `getter`
impl Atom {
    pub fn new_builder() -> AtomBuilder {
        AtomBuilder::default()
    }

    pub fn symbol(&self) -> &ElementSymbol {
        &self.symbol
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn coord(&self) -> CoordData {
        self.coord
    }
    pub fn label(&self) -> &Option<String> {
        &self.label
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Atom {}:
  Element Symbol: {:?}
  XYZ: {:#?}"#,
            self.index + 1,
            self.symbol,
            self.coord
        )
    }
}
