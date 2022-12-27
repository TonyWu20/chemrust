use crate::data::{
    format::DataFormat,
    param::{KPoint, ModelParameters},
};

impl<T> Default for ModelParameters<T>
where
    T: DataFormat,
{
    fn default() -> Self {
        Self {
            kpoints_list: vec![KPoint::default()],
            kpoints_grid: Default::default(),
            kpoints_mp_spacing: Default::default(),
            kpoints_mp_offset: Default::default(),
            fix_all_cell: Default::default(),
            fix_com: Default::default(),
            external_efield: Default::default(),
            external_pressure: Default::default(),
            cry_display: Default::default(),
            periodic_type: Default::default(),
            space_group: Default::default(),
            cry_tolerance: Default::default(),
            format_marker: Default::default(),
        }
    }
}
