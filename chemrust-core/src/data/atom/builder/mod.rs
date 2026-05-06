use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::CoordData;

use self::error::AtomBuilderIncomplete;

use super::AtomSite;

mod error;

#[derive(Debug, Clone)]
pub struct AtomSiteBuilder {
    symbol: Option<ElementSymbol>,
    coord: Option<CoordData>,
    index: Option<usize>,
    label: Option<String>,
}

impl AtomSiteBuilder {
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
    pub fn with_coord(&mut self, coord: CoordData) -> &mut Self {
        self.coord = Some(coord);
        self
    }
    pub fn with_label(&mut self, label: &Option<String>) -> &mut Self {
        self.label = label.clone();
        self
    }
    pub fn build(&self) -> Result<AtomSite, AtomBuilderIncomplete> {
        if self.symbol.is_some() && self.index.is_some() && self.coord.is_some() {
            Ok(AtomSite {
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

impl Default for AtomSiteBuilder {
    fn default() -> AtomSiteBuilder {
        AtomSiteBuilder {
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

    use crate::data::{atom::AtomSite, geom::coordinates::CoordData};

    #[test]
    fn test_atom_builder() {
        let atom = AtomSite::new_builder()
            .with_index(1)
            .with_coord(CoordData::Fractional(Point3::new(0.0, 0.5, 0.5)))
            .with_symbol(ElementSymbol::H)
            .build();
        assert!(atom.is_ok());
        println!("{}", atom.unwrap());
    }
}
