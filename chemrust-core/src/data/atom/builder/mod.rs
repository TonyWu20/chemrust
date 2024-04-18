use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::{Cartesian, Coordinate, CoordinateType, Fractional};

use self::error::AtomBuilderIncomplete;

use super::Atom;

mod error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AtomBuilder<T: CoordinateType> {
    symbol: Option<ElementSymbol>,
    coord: Option<Coordinate<T>>,
    index: Option<usize>,
    label: Option<String>,
}

impl<T: CoordinateType> AtomBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_index(&mut self, index: usize) -> &mut Self {
        self.index = Some(index);
        self
    }
    pub fn with_symbol(&mut self, symbol: ElementSymbol) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }
    pub fn build(&self) -> Result<Atom<T>, AtomBuilderIncomplete> {
        if self.symbol.is_some() && self.index.is_some() && self.coord.is_some() {
            Ok(Atom {
                symbol: self.symbol.unwrap(),
                coord: self.coord.unwrap(),
                index: self.index.unwrap(),
                label: self.label.clone(),
            })
        } else {
            Err(AtomBuilderIncomplete)
        }
    }
}

impl AtomBuilder<Fractional> {
    pub fn with_frac_coord(&mut self, frac_coord: Coordinate<Fractional>) -> &mut Self {
        self.coord = Some(frac_coord);
        self
    }
}

impl AtomBuilder<Cartesian> {
    pub fn with_cart_coord(&mut self, cart_coord: Coordinate<Cartesian>) -> &mut Self {
        self.coord = Some(cart_coord);
        self
    }
}

impl<T: CoordinateType> Default for AtomBuilder<T> {
    fn default() -> AtomBuilder<T> {
        AtomBuilder {
            symbol: Some(ElementSymbol::H),
            coord: None,
            index: None,
            label: None,
        }
    }
}

#[cfg(test)]
mod test {
    use castep_periodic_table::element::ElementSymbol;
    use nalgebra::Point3;

    use crate::data::{
        atom::Atom,
        geom::coordinates::{Coordinate, Fractional},
    };

    #[test]
    fn test_atom_builder() {
        let atom = Atom::<Fractional>::new_builder()
            .with_index(1)
            .with_frac_coord(Coordinate::<Fractional>::new(Point3::new(0.0, 0.5, 0.5)))
            .with_symbol(ElementSymbol::H)
            .build();
        assert!(atom.is_ok())
    }
}
