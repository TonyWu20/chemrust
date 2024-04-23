use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::CoordData;

/// Basic `component`s for atom site data.
/// Custom type to extend fields for `AtomData` can be fitted in `CrystalEntity` by
/// implementing this trait
pub trait CoreAtomData {
    fn indices(&self) -> &[usize];

    fn symbol(&self) -> &[ElementSymbol];

    fn coord(&self) -> &[CoordData];

    fn label(&self) -> &[Option<String>];
}
