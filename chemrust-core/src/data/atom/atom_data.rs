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
