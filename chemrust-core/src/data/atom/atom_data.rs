use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::CoordData;

/// Basic `component`s for atom site data.
/// Custom type to extend fields for `AtomData` can be fitted in `CrystalEntity` by
/// implementing this trait
pub trait CoreAtomData {
    fn indices(&self) -> &[usize];

    fn symbols(&self) -> &[ElementSymbol];

    fn coords(&self) -> &[CoordData];

    fn labels(&self) -> &[Option<String>];
}

/// Basic example struct
#[derive(Debug, Clone)]
pub struct Atoms {
    indices: Vec<usize>,
    symbols: Vec<ElementSymbol>,
    coordinates: Vec<CoordData>,
    labels: Vec<Option<String>>,
}

impl Atoms {
    pub fn new(
        indices: Vec<usize>,
        symbols: Vec<ElementSymbol>,
        coordinates: Vec<CoordData>,
        labels: Vec<Option<String>>,
    ) -> Self {
        Self {
            indices,
            symbols,
            coordinates,
            labels,
        }
    }
}

impl CoreAtomData for Atoms {
    fn indices(&self) -> &[usize] {
        &self.indices
    }

    fn symbols(&self) -> &[ElementSymbol] {
        &self.symbols
    }

    fn coords(&self) -> &[CoordData] {
        &self.coordinates
    }

    fn labels(&self) -> &[Option<String>] {
        &self.labels
    }
}
