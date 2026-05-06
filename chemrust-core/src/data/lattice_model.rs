use super::atom::CoreAtomData;
use super::lattice::UnitCellParameters;
use super::symmetry::SymmetryInfo;
use crystallographic_group::database::SpaceGroupHallSymbol;
/// Minimal data struct to represent a lattice model.
/// This is made for internal data operations between downstream crates based on
/// this crate.
#[derive(Debug, Clone)]
pub struct LatticeModel<U: UnitCellParameters, C: CoreAtomData> {
    lattice_param: U,
    atoms: C,
    space_group: SpaceGroupHallSymbol,
}

impl<U: UnitCellParameters, C: CoreAtomData> LatticeModel<U, C> {
    pub fn lattice_param(&self) -> &U {
        &self.lattice_param
    }

    pub fn atoms(&self) -> &C {
        &self.atoms
    }

    pub fn space_group(&self) -> SpaceGroupHallSymbol {
        self.space_group
    }
}

impl<U: UnitCellParameters, C: CoreAtomData> SymmetryInfo for LatticeModel<U, C> {
    fn make_symmetry(&self) -> bool {
        !matches!(self.space_group, SpaceGroupHallSymbol::P_1)
    }

    fn get_space_group_it_num(&self) -> u8 {
        let number_code = self.space_group.get_space_group_number_code();
        if number_code.contains(":") {
            number_code
                .split(":")
                .next()
                .and_then(|v| v.parse::<u8>().ok())
                .expect("The space group number (1-230) should be within range of `u8`")
        } else {
            number_code
                .parse::<u8>()
                .expect("The space group number (1-230) should be within range of `u8`")
        }
    }
}
