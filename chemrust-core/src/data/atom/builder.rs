use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};
use nalgebra::Point3;

use crate::builder_state::{BuilderState, Pending, Ready};

use super::Atom;

pub struct AtomBuilder<U: BuilderState> {
    symbol: Option<String>,
    atomic_number: Option<u8>,
    cartesian_coord: Option<Point3<f64>>,
    index: Option<usize>,
    state: PhantomData<U>,
}
#[derive(Debug)]
pub struct AtomBuilderIncomplete;

impl Display for AtomBuilderIncomplete {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The AtomBuilder has one or more fields to be completed!")
    }
}

impl AtomBuilder<Pending> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_index(self, index: usize) -> Self {
        Self {
            symbol: self.symbol,
            atomic_number: self.atomic_number,
            cartesian_coord: self.cartesian_coord,
            index: Some(index),
            state: PhantomData,
        }
    }
    pub fn with_symbol(self, symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.into()),
            atomic_number: self.atomic_number,
            cartesian_coord: self.cartesian_coord,
            index: self.index,
            state: PhantomData,
        }
    }
    pub fn with_atomic_number(self, atomic_number: u8) -> Self {
        Self {
            symbol: self.symbol,
            atomic_number: Some(atomic_number),
            cartesian_coord: self.cartesian_coord,
            index: self.index,
            state: PhantomData,
        }
    }
    pub fn with_coord(self, cartesian_coord: &Point3<f64>) -> Self {
        Self {
            symbol: self.symbol,
            atomic_number: self.atomic_number,
            cartesian_coord: Some(*cartesian_coord),
            index: self.index,
            state: PhantomData,
        }
    }
    pub fn ready(self) -> AtomBuilder<Ready> {
        AtomBuilder {
            symbol: self.symbol,
            atomic_number: self.atomic_number,
            cartesian_coord: self.cartesian_coord,
            index: self.index,
            state: PhantomData,
        }
    }
}

impl AtomBuilder<Ready> {
    pub fn build(self) -> Atom {
        let Self {
            symbol,
            atomic_number,
            cartesian_coord,
            index,
            state: _,
        } = self;
        // The input symbol is prioritized. If the symbol is wrong, fallback to Hydrogen.
        if let Some(name) = symbol {
            let element = ELEMENT_TABLE
                .get_by_symbol(&name)
                .unwrap_or(ELEMENT_TABLE.get_by_atomic_number(0).unwrap());
            Atom {
                symbol: element.symbol().into(),
                atomic_number: element.atomic_number(),
                cartesian_coord: cartesian_coord.unwrap_or(Point3::origin()),
                index: index.unwrap_or(0),
            }
        } else if let Some(num) = atomic_number {
            // If no symbol, but only atomic number, use atomic number.
            let element = ELEMENT_TABLE
                .get_by_atomic_number(num)
                .unwrap_or(ELEMENT_TABLE.get_by_atomic_number(0).unwrap()); // fallback to Hydrogen if the number is wrong.
            Atom {
                symbol: element.symbol().into(),
                atomic_number: element.atomic_number(),
                cartesian_coord: cartesian_coord.unwrap_or(Point3::origin()),
                index: index.unwrap_or(0),
            }
        } else {
            // When symbol and atomic number are both absent, fallback to Hydrogen.
            Atom {
                symbol: "H".into(),
                atomic_number: 0,
                cartesian_coord: cartesian_coord.unwrap_or(Point3::origin()),
                index: index.unwrap_or(0),
            }
        }
    }
}

impl Default for AtomBuilder<Pending> {
    fn default() -> AtomBuilder<Pending> {
        AtomBuilder {
            symbol: None,
            atomic_number: None,
            cartesian_coord: None,
            index: None,
            state: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Point3;

    use crate::data::atom::Atom;

    #[test]
    fn test_atom_builder() {
        let atom: Atom = Atom::new_builder()
            .with_index(1)
            .with_atomic_number(0)
            .with_symbol("H")
            .with_coord(&Point3::new(0.0, 0.0, 0.0))
            .ready()
            .build();
        println!("{:?}", atom);
        let atom_2 = Atom::new_builder().with_symbol("Co").ready().build();
        println!("{:?}", atom_2);
    }
}
