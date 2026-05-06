use crystallographic_group::{
    database::{LookUpSpaceGroup, DEFAULT_SPACE_GROUP_SYMBOLS},
    hall_symbols::HallSymbolNotation,
};

pub trait SymmetryInfo {
    fn make_symmetry(&self) -> bool;
    /// 1-230
    fn get_space_group_it_num(&self) -> u8;
    fn get_space_group_hall_symbol(&self) -> HallSymbolNotation {
        let num = self.get_space_group_it_num() - 1;
        let hall_symbol = DEFAULT_SPACE_GROUP_SYMBOLS
            .get_hall_symbol(num as usize)
            .expect("Should have hall symbol result from it num");
        HallSymbolNotation::try_from_str(hall_symbol)
            .expect("Hall symbol from `crystallographic_group`")
    }
}
