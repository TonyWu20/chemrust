use kd_tree::KdTree;
use nalgebra::Point3;

#[cfg(test)]
mod test {
    use std::{
        collections::{HashMap, HashSet},
        fs::read_to_string,
    };

    use chemrust_core::data::atom::AtomCollections;
    use chemrust_parser::CellParser;
    use kd_tree::{KdIndexTree, KdMap, KdPoint, KdTree};
    use nalgebra::Point3;

    #[test]
    fn test_kd_tree() {
        let cell = read_to_string("../chemrust-parser/SAC_GDY_V.cell").unwrap();
        let lattice = CellParser::new(&cell)
            .to_lattice_cart()
            .to_positions()
            .build_lattice();
        let atom_collections: AtomCollections = lattice.atoms().into();
        let coords: Vec<Point3<f64>> = atom_collections.cartesian_coords().to_vec();
        let kdtree = KdIndexTree::build_by_ordered_float(&coords);
        let radius = 1.5;
        let mut walked_over_set: HashSet<usize> = HashSet::new();
        coords.iter().enumerate().for_each(|(i, p)| {
            let found = kdtree.nearests(p, 2);
            let found_id = found.get(1).unwrap().item;
            if let None = walked_over_set.get(found_id) {
                let distance = found.get(1).unwrap().squared_distance;
                if (2.0 * radius - distance).abs() <= 0.001 {
                    println!("One intersect between atom {} and {}", i, found_id);
                } else if 2.0 * radius - distance > 0.001 {
                    println!("Two intersects between atom {} and {}", i, found_id)
                } else {
                    println!("No intersect between atom {} and {}", i, found_id)
                }
                walked_over_set.insert(i);
            }
        });
        let found = kdtree.within_radius(&coords[72], 1.6);
        println!("Found {} atoms", found.len() - 1);
    }
}
