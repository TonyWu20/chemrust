mod attributes;
mod collection;
mod markers;
pub use attributes::*;
pub use collection::AtomCollection;
pub use markers::*;

// Unit tests for AtomAttr<T,N>
#[cfg(test)]
mod test {

    use nalgebra::Point3;

    use crate::{data::format::msi::Msi, system::data_view::AttrCollectionView};

    use super::{AtomCollection, AtomId, AtomicNumber, CartesianCoord, ElementSymbol};

    #[test]
    fn test_attributes() {
        let atom_id: AtomId<Msi> = AtomId::new(1_u32, 0);
        let element_symbol: ElementSymbol<Msi> = ElementSymbol::new("H".into(), 0);
        let atomic_number: AtomicNumber<Msi> = AtomicNumber::new(0, 0);
        let cart_coord: CartesianCoord<Msi> =
            CartesianCoord::new(Point3::new(0_f64, 0_f64, 0_f64), 0);
        assert_eq!(&1_u32, atom_id.content());
        assert_eq!("H", element_symbol.content());
        assert_eq!(&0_u8, atomic_number.content());
        assert_eq!(&Point3::new(0_f64, 0_f64, 0_f64), cart_coord.content());
    }
    #[test]
    fn test_collection_sort() {
        let element_symbols = vec!["Na", "H", "H", "C", "C", "H", "O", "Na"];
        let atomic_numbers: Vec<u8> = vec![23, 0, 0, 6, 6, 0, 8, 23];
        let symbols_collection: Vec<ElementSymbol<Msi>> = element_symbols
            .iter()
            .enumerate()
            .map(|(i, &symbol)| ElementSymbol::new(symbol.into(), i))
            .collect();
        let mut atomic_number_collection: Vec<AtomicNumber<Msi>> = atomic_numbers
            .iter()
            .enumerate()
            .map(|(i, &num)| AtomicNumber::new(num, i))
            .collect();
        println!("Before sorting");

        println!("Symbols: {:?}", symbols_collection);
        println!("Atomic numbers: {:?}", atomic_number_collection);
        atomic_number_collection.sort_by_key(|item| *item.content());
        println!("Atomic numbers: {:?}", atomic_number_collection);
        let symbols_collection = symbols_collection.rearrange_with(&atomic_number_collection);
        println!("Symbols after sorted: {:?}", symbols_collection);
    }
    #[test]
    fn test_build_atoms() {
        let element_symbols = vec!["Na", "H", "H", "C", "C", "H", "O", "Na"];
        let atomic_numbers: Vec<u8> = vec![23, 0, 0, 6, 6, 0, 8, 23];
        let atom_ids: Vec<u32> = (1..9).into_iter().collect();
        let xyz_coords: Vec<Point3<f64>> = (0..8)
            .into_iter()
            .map(|i| Point3::new(i as f64, 0.0, 0.0))
            .collect();
        let symbols: Vec<ElementSymbol<Msi>> = element_symbols
            .into_iter()
            .enumerate()
            .map(|(i, symbol)| ElementSymbol::new(symbol.into(), i))
            .collect();
        let atomic_nums: Vec<AtomicNumber<Msi>> = atomic_numbers
            .into_iter()
            .enumerate()
            .map(|(i, num)| AtomicNumber::new(num, i))
            .collect();
        let xyz_coords: Vec<CartesianCoord<Msi>> = xyz_coords
            .into_iter()
            .enumerate()
            .map(|(i, xyz)| CartesianCoord::new(xyz, i))
            .collect();
        let atom_ids: Vec<AtomId<Msi>> = atom_ids
            .into_iter()
            .enumerate()
            .map(|(i, id)| AtomId::new(id, i))
            .collect();
        let atoms = AtomCollection::builder(8)
            .with_xyz(&xyz_coords)
            .unwrap()
            .with_symbols(&symbols)
            .unwrap()
            .with_atom_ids(&atom_ids)
            .unwrap()
            .with_atomic_number(&atomic_nums)
            .unwrap()
            .finish()
            .build();
        println!("{}", atoms);
    }
}
