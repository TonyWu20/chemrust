use castep_periodic_table::element::ElementSymbol;

use crate::data::geom::coordinates::CoordData;

use super::atom_site::AtomSiteData;

/// Basic Struct of Array for atom site data.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AtomData {
    pub(crate) indices: Vec<usize>,
    pub(crate) symbol: Vec<ElementSymbol>,
    pub(crate) coord: Vec<CoordData>,
    pub(crate) label: Vec<Option<String>>,
}

/// Custom type to extend fields for `AtomData` can be fitted in `CrystalModel` by
/// implementing this trait
pub trait AtomDataArray {
    fn indices(&self) -> &[usize];

    fn symbol(&self) -> &[ElementSymbol];

    fn coord(&self) -> &[CoordData];

    fn label(&self) -> &[Option<String>];
}

impl AtomDataArray for AtomData {
    fn indices(&self) -> &[usize] {
        self.indices.as_ref()
    }

    fn symbol(&self) -> &[ElementSymbol] {
        self.symbol.as_ref()
    }

    fn coord(&self) -> &[CoordData] {
        self.coord.as_ref()
    }

    fn label(&self) -> &[Option<String>] {
        self.label.as_ref()
    }
}

impl<T: AtomSiteData> From<Vec<T>> for AtomData {
    fn from(atom_sites: Vec<T>) -> Self {
        let number_of_atoms = atom_sites.len();
        let mut atom_data = Self {
            indices: Vec::with_capacity(number_of_atoms),
            symbol: Vec::with_capacity(number_of_atoms),
            coord: Vec::with_capacity(number_of_atoms),
            label: Vec::with_capacity(number_of_atoms),
        };
        atom_sites.iter().for_each(|atom| {
            atom_data.indices.push(atom.index());
            Vec::push(&mut atom_data.symbol, atom.symbol());
            atom_data.coord.push(atom.coord());
            Vec::push(&mut atom_data.label, atom.label().clone());
        });
        atom_data
    }
}
