use castep_periodic_table::element::ElementSymbol;
use derive_builder::Builder;

use crate::data::geom::coordinates::CoordData;

/// Basic `component`s for atom site data.
/// Custom type to extend fields for `AtomData` can be fitted in `CrystalModel` by
/// implementing this trait
pub trait CoreAtomData {
    fn indices_repr(&self) -> Vec<usize>;
    fn symbols_repr(&self) -> Vec<ElementSymbol>;
    fn coords_repr(&self) -> Vec<CoordData>;
    fn labels_repr(&self) -> Vec<Option<String>>;
}

/// Basic `component`s for atom site data.
/// This is for borrowing data only to reduce clone cost.
/// Custom type to extend fields for `AtomData` can be fitted in `CrystalModel` by
/// implementing this trait
pub trait CoreAtomDataView {
    fn indices_view(&self) -> &[usize];
    fn symbols_view(&self) -> &[ElementSymbol];
    fn coords_view(&self) -> &[CoordData];
    fn labels_view(&self) -> &[Option<String>];
}

/// Basic example struct
#[derive(Debug, Clone, PartialEq, PartialOrd, Builder)]
pub struct Atoms {
    pub indices: Vec<usize>,
    pub symbols: Vec<ElementSymbol>,
    pub coordinates: Vec<CoordData>,
    pub labels: Vec<Option<String>>,
    /// The cell vectors should be stored in column-major order.
    pub cell: Option<[[f64; 3]; 3]>,
}

impl CoreAtomData for Atoms {
    fn indices_repr(&self) -> Vec<usize> {
        self.indices.clone()
    }

    fn symbols_repr(&self) -> Vec<ElementSymbol> {
        self.symbols.clone()
    }

    fn coords_repr(&self) -> Vec<CoordData> {
        self.coordinates.clone()
    }

    fn labels_repr(&self) -> Vec<Option<String>> {
        self.labels.clone()
    }
}

impl CoreAtomDataView for Atoms {
    fn indices_view(&self) -> &[usize] {
        &self.indices
    }

    fn symbols_view(&self) -> &[ElementSymbol] {
        &self.symbols
    }

    fn coords_view(&self) -> &[CoordData] {
        &self.coordinates
    }

    fn labels_view(&self) -> &[Option<String>] {
        &self.labels
    }
}
