use nalgebra::Point3;

use crate::builder_state::Pending;

use self::builder::AtomBuilder;

mod builder;
mod collection;

#[derive(Debug, PartialEq, PartialOrd)]
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

// Unit tests for Atom
#[cfg(test)]
mod test {}
