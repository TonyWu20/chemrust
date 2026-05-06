mod kpoints;
mod monkhorst_pack;
mod symmetry_operations;

mod functions;

#[cfg(test)]
mod tests {
    use chemrust_core::data::lattice::{LatticeVectors, ReciprocalCellVectors};
    use crystallographic_group::{
        database::{LookUpSpaceGroup, DEFAULT_SPACE_GROUP_SYMBOLS},
        *,
    };
    use nalgebra::Matrix3;

    use crate::{
        functions::reduce_kpoints,
        monkhorst_pack::{MPGrid, MPSpacingParam},
    };
    fn get_core_ops(hall_symbol: &HallSymbolNotation) -> Vec<SeitzMatrix> {
        let gen = hall_symbol.general_positions();
        gen.core_position_set().to_vec()
    }

    #[test]
    fn it_works() {
        let symmetry_group_hm_symbol = DEFAULT_SPACE_GROUP_SYMBOLS
            .get_hall_symbol(20)
            .expect("Space group number 20 is within 230");
        dbg!(symmetry_group_hm_symbol);
        let symmetry_group = HallSymbolNotation::try_from_str(symmetry_group_hm_symbol).unwrap();
        let binding = symmetry_group.general_positions();
        let symmetry_operations = binding.core_position_set();
        let lattice = LatticeVectors::new(Matrix3::<f64>::new(
            5.4, 0.0, 0.0, 0.0, 5.4, 0.0, 0.0, 0.0, 5.4,
        ));
        let reciprocal = ReciprocalCellVectors::from(lattice);
        let mp_spacing = MPSpacingParam::new(0.07, [0.0, 0.0, 0.0]);
        let mp_grid = mp_spacing.build_mp_grid(reciprocal);
        let reducible_kpts = mp_grid.generate_reducible_kpts();
        let irreducible_kpts = reduce_kpoints(&reducible_kpts, symmetry_operations);
        dbg!(irreducible_kpts.len());
        let symmetry_group = HallSymbolNotation::try_from_str("P 1").unwrap();
        let core_ops = get_core_ops(&symmetry_group);
        let mp_grid = MPGrid::new([1, 2, 3], [0.0; 3]);
        let reducible_kpts = mp_grid.generate_reducible_kpts();
        let irreducible_kpts = reduce_kpoints(&reducible_kpts, &core_ops);
        let total_degeneracy = irreducible_kpts
            .iter()
            .fold(0, |acc, x| acc + x.degeneracy());
        println!("Grid : {:?}", mp_grid);
        irreducible_kpts.iter().for_each(|k| {
            println!(
                "{} {}",
                k.kpt(),
                k.degeneracy() as f64 / (total_degeneracy as f64)
            )
        });
        let new_grid = MPGrid::new([2, 2, 2], [0.0; 3]);
        let irreducible_kpts = reduce_kpoints(&new_grid.generate_reducible_kpts(), &core_ops);
        println!("Grid : {:?}", new_grid);
        let total_degeneracy = irreducible_kpts
            .iter()
            .fold(0, |acc, x| acc + x.degeneracy());
        irreducible_kpts.iter().for_each(|k| {
            println!(
                "{} {}",
                k.kpt(),
                k.degeneracy() as f64 / (total_degeneracy as f64)
            )
        });
    }
}
