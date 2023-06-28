use std::fs::File;
use std::io::{self, BufRead, BufReader};

use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use crate::cell_settings::{
    CellSettingExport, ExternalEField, ExternalPressure, FixAllCell, FixCOM, IonicConstraints,
    KPointsList, SpeciesCharacteristics,
};
use crate::{ModelFormat, StructureFile};

#[derive(Debug, Clone, Copy, Default)]
/// A unit struct to mark `cell`format.
pub struct Cell;

impl ModelFormat for Cell {}

/// Methods for `CellFormat`
impl Cell {
    pub fn write_block(block: (String, String)) -> String {
        let (block_name, content) = block;
        format!(
            "%BLOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, content, block_name
        )
    }
}

impl StructureFile<Cell> {
    pub fn write_lattice_vectors(&self) -> String {
        let formatted_vector: Vec<String> = self
            .lattice_model
            .lattice_vectors()
            .unwrap()
            .data()
            .column_iter()
            .map(|col| format!("{:24.18}{:24.18}{:24.18}\n", col.x, col.y, col.z))
            .collect();
        let formatted_vector = formatted_vector.concat();
        Cell::write_block(("LATTICE_CART".to_string(), formatted_vector))
    }
    pub fn write_atoms(&self) -> String {
        let cart_to_frac_matrix = self
            .lattice_model
            .lattice_vectors()
            .unwrap()
            .mat_cart_to_frac();
        let atom_frac_coords_text: Vec<String> = self
            .lattice_model
            .atoms()
            .iter()
            .map(|atom| {
                let frac_xyz = cart_to_frac_matrix * atom.cartesian_coord();
                let symbol = atom.symbol();
                let spin = ELEMENT_TABLE.get_by_symbol(symbol).unwrap().spin();
                let spin_str = if spin > 0 {
                    format!(" SPIN={:14.10}", spin)
                } else {
                    "".into()
                };
                format!(
                    "{:>3}{:20.16}{:20.16}{:20.16}{spin_str}\n",
                    symbol, frac_xyz.x, frac_xyz.y, frac_xyz.z
                )
            })
            .collect();
        let all_atom_fracs = atom_frac_coords_text.concat();
        Cell::write_block(("POSITIONS_FRAC".into(), all_atom_fracs))
    }
    pub fn spin_total(&self) -> u8 {
        self.lattice_model
            .element_list()
            .iter()
            .map(|symbol| ELEMENT_TABLE.get_by_symbol(&symbol).unwrap().spin())
            .reduce(|total, next| total + next)
            .unwrap()
    }
    pub fn get_final_cutoff_energy(&self, potentials_loc: &str) -> Result<f64, io::Error> {
        let mut energy: f64 = 0.0;
        self.lattice_model
            .element_list()
            .iter()
            .try_for_each(|elm| -> Result<(), io::Error> {
                let potential_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
                let potential_path = format!("{potentials_loc}/{potential_file}");
                if let Ok(file) = File::open(&potential_path) {
                    let file = BufReader::new(file);
                    let fine_energy: u32 = file
                        .lines()
                        .find(|line| line.as_ref().unwrap().contains("FINE"))
                        .map(|line| {
                            let num_str = line.as_ref().unwrap().split_whitespace().next().unwrap();
                            num_str.parse::<u32>().unwrap()
                        })
                        .unwrap();
                    let round_bigger_tenth = |num: u32| -> f64 {
                        match num % 10 {
                            0 => num as f64,
                            _ => ((num / 10 + 1) * 10) as f64,
                        }
                    };
                    let ultra_fine_energy = round_bigger_tenth((fine_energy as f64 * 1.1) as u32);
                    energy = if energy > ultra_fine_energy {
                        energy
                    } else {
                        ultra_fine_energy
                    };
                    Ok(())
                } else {
                    panic!(
                        "Error while reading potential file for element: {}, {}",
                        elm, potential_path
                    )
                }
            })?;
        Ok(energy)
    }
    pub fn export_geom_cell(&self) -> String {
        let lattice_cart = self.write_lattice_vectors();
        let atoms = self.write_atoms();
        let kpts_list = KPointsList::default().write_kpoints_list();
        let fix_constraints = FixCOM::default().write_to_cell();
        let fix_all_cell = FixAllCell::default().write_to_cell();
        let ionic_cons = IonicConstraints::default().write_to_cell();
        let extern_field = ExternalEField::default().write_to_cell();
        let extern_pressure = ExternalPressure::default().write_to_cell();
        let species_characters =
            SpeciesCharacteristics::new(self.lattice_model.element_list()).write_to_cell();
        let text = vec![
            lattice_cart,
            atoms,
            kpts_list,
            fix_constraints,
            fix_all_cell,
            ionic_cons,
            extern_field,
            extern_pressure,
            species_characters,
        ];
        text.concat()
    }
    pub fn export_bs_cell(&self) -> String {
        let lattice_cart = self.write_lattice_vectors();
        let atoms = self.write_atoms();
        let bs_kpts_list = KPointsList::default().write_bs_kpoints_list();
        let kpts_list = KPointsList::default().write_kpoints_list();
        let fix_constraints = FixCOM::default().write_to_cell();
        let fix_all_cell = FixAllCell::default().write_to_cell();
        let ionic_cons = IonicConstraints::default().write_to_cell();
        let extern_field = ExternalEField::default().write_to_cell();
        let extern_pressure = ExternalPressure::default().write_to_cell();
        let species_characters =
            SpeciesCharacteristics::new(self.lattice_model.element_list()).write_to_cell();
        let text = vec![
            lattice_cart,
            atoms,
            bs_kpts_list,
            kpts_list,
            fix_constraints,
            fix_all_cell,
            ionic_cons,
            extern_field,
            extern_pressure,
            species_characters,
        ];
        text.concat()
    }
}
