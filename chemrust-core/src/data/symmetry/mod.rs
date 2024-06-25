pub trait SymmetryInfo {
    type CrystalSystem;
    /// 1-230
    fn get_space_group_it_num(&self) -> u8;
    fn get_crystal_system(&self) -> Self::CrystalSystem;
}
