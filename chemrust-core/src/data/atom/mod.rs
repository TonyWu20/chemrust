use std::fmt::Display;

use nalgebra::Point3;

use crate::builder_state::Pending;

use self::builder::AtomBuilder;

mod builder;
mod collection;

pub use collection::AtomCollections;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Atom {
    symbol: String,
    atomic_number: u8,
    cartesian_coord: Point3<f64>,
    index: usize,
}

impl Atom {
    pub fn new_builder() -> AtomBuilder<Pending> {
        AtomBuilder::default()
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub fn cartesian_coord(&self) -> Point3<f64> {
        self.cartesian_coord
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Atom {}:
  Atomic Number: {}
  Element Symbol: {}
  XYZ: {:#}"#,
            self.index + 1,
            self.atomic_number,
            self.symbol,
            self.cartesian_coord
        )
    }
}
