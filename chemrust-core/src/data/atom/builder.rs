use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};

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
    pub fn finish(self) -> Result<AtomBuilder<Ready>, AtomBuilderIncomplete> {
        if let Self {
            symbol: Some(symbol),
            atomic_number: Some(atomic_number),
            cartesian_coord: Some(cart),
            index: Some(index),
            state: _,
        } = self
        {
            Ok(AtomBuilder {
                symbol: Some(symbol),
                atomic_number: Some(atomic_number),
                cartesian_coord: Some(cart),
                index: Some(index),
                state: PhantomData,
            })
        } else {
            Err(AtomBuilderIncomplete)
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
        Atom {
            symbol: symbol.unwrap(),
            atomic_number: atomic_number.unwrap(),
            cartesian_coord: cartesian_coord.unwrap(),
            index: index.unwrap(),
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
            .finish()
            .unwrap()
            .build();
        println!("{:?}", atom);
    }
}
