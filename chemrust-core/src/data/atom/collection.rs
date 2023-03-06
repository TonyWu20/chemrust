use nalgebra::Point3;

use super::Atom;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct AtomCollections {
    symbols: Vec<String>,
    atomic_numbers: Vec<u8>,
    cartesian_coords: Vec<Point3<f64>>,
    indexes: Vec<usize>,
    size: usize,
}

impl AtomCollections {
    pub fn new(size: usize) -> Self {
        AtomCollections {
            symbols: Vec::with_capacity(size),
            atomic_numbers: Vec::with_capacity(size),
            cartesian_coords: Vec::with_capacity(size),
            indexes: Vec::with_capacity(size),
            size,
        }
    }
    pub fn symbols(&self) -> &[String] {
        self.symbols.as_ref()
    }

    pub fn atomic_numbers(&self) -> &[u8] {
        self.atomic_numbers.as_ref()
    }

    pub fn cartesian_coords(&self) -> &[Point3<f64>] {
        self.cartesian_coords.as_ref()
    }

    pub fn indexes(&self) -> &[usize] {
        self.indexes.as_ref()
    }
    /// Retrieve an `Atom` at given 0th-based index.
    pub fn get_atom_at(&self, index: usize) -> Option<Atom> {
        if index >= self.size {
            None
        } else {
            let symbol = self.symbols().get(index).unwrap();
            let atomic_num = self.atomic_numbers().get(index).unwrap();
            let cartesian_coord = self.cartesian_coords().get(index).unwrap();
            Some(
                Atom::new_builder()
                    .with_symbol(symbol)
                    .with_atomic_number(*atomic_num)
                    .with_coord(cartesian_coord)
                    .with_index(index)
                    .finish()
                    .unwrap()
                    .build(),
            )
        }
    }
}

impl From<Vec<Atom>> for AtomCollections {
    fn from(value: Vec<Atom>) -> Self {
        let collection_size = value.len();
        let mut collections = AtomCollections::new(collection_size);
        value.iter().for_each(|atom| {
            collections.symbols.push(atom.symbol().into());
            collections.atomic_numbers.push(atom.atomic_number());
            collections.cartesian_coords.push(atom.cartesian_coord());
            collections.indexes.push(atom.index);
        });
        collections
    }
}

impl From<AtomCollections> for Vec<Atom> {
    fn from(value: AtomCollections) -> Self {
        let collection_size = value.indexes().len();
        let mut atom_vec: Vec<Atom> = Vec::with_capacity(collection_size);
        for i in 0..collection_size {
            atom_vec.push(value.get_atom_at(i).unwrap())
        }
        atom_vec
    }
}
