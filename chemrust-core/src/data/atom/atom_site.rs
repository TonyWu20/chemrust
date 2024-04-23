/// Essential data to record an atom.
/// Other complementary informations can be generated based on `symbol`
/// or wrapped in a newtype.
/// The newtype will be required to implement the `AtomSiteData` trait
/// to be put into the `CrystalModel`.
use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::CoordData;

use super::{atom_data::AtomData, builder::AtomSiteBuilder, AtomDataArray};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AtomSite {
    pub(crate) index: usize,
    /// The element symbol of the atom
    pub(crate) symbol: ElementSymbol,
    /// Coordinate of the atom:
    pub(crate) coord: CoordData,
    pub(crate) label: Option<String>,
}

pub trait AtomSiteData {
    fn index(&self) -> usize;
    fn symbol(&self) -> ElementSymbol;
    fn coord(&self) -> CoordData;
    fn label(&self) -> &Option<String>;
}

/// Only methods for `builder`
impl AtomSite {
    pub fn new_builder() -> AtomSiteBuilder {
        AtomSiteBuilder::default()
    }
}

impl AtomSiteData for AtomSite {
    fn symbol(&self) -> ElementSymbol {
        self.symbol
    }
    fn index(&self) -> usize {
        self.index
    }
    fn coord(&self) -> CoordData {
        self.coord
    }
    fn label(&self) -> &Option<String> {
        &self.label
    }
}

impl Display for AtomSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Atom {}:
  Element Symbol: {:?}
  XYZ: {:#?}"#,
            self.index + 1,
            self.symbol,
            self.coord,
        )
    }
}

impl From<AtomData> for Vec<AtomSite> {
    fn from(atom_data: AtomData) -> Self {
        atom_data
            .indices()
            .iter()
            .zip(atom_data.symbol().iter())
            .zip(atom_data.coord().iter())
            .zip(atom_data.label().iter())
            .map(
                |(((index, symbol), coord), label): (
                    ((&usize, &ElementSymbol), &CoordData),
                    &Option<String>,
                )| {
                    AtomSite::new_builder()
                        .with_index(*index)
                        .with_symbol(*symbol)
                        .with_coord(*coord)
                        .with_label(label)
                        .build()
                        .unwrap()
                },
            )
            .collect()
    }
}
