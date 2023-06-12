use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use crate::{ModelFormat, StructureFile};

#[derive(Debug, Clone, Default)]
/// A unit struct to mark `cell`format.
pub struct Cell;

impl ModelFormat for Cell {}

/// Methods for `CellFormat`
impl Cell {
    pub fn write_block(block: (String, String)) -> String {
        let (block_name, content) = block;
        format!(
            "%BlOCK {}\n{}%ENDBLOCK {}\n\n",
            block_name, content, block_name
        )
    }
}

impl StructureFile<Cell> {
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
    /// No constraints. Future: adapt to settings
    fn ionic_constraints(&self) -> String {
        Cell::write_block(("IONIC_CONSTRAINTS".to_string(), "".to_string()))
    }
    /// Miscellaneous parameters
    fn misc_options(&self) -> String {
        todo!();
        //         let fix = format!(
        //             "FIX_ALL_CELL : {}\n\nFIX_COM : {}\n{}",
        //             self.settings().fix_all_cell(),
        //             self.settings().fix_com(),
        //             self.ionic_constraints()
        //         );
        //         let [ex, ey, ez] = self.settings().external_efield();
        //         let external_efield = Cell::write_block((
        //             "EXTERNAL_EFIELD".to_string(),
        //             format!("{:16.10}{:16.10}{:16.10}\n", ex, ey, ez),
        //         ));
        //         let [rxx, rxy, rxz, ryy, ryz, rzz] = self.settings().external_pressure();
        //         let external_pressure = Cell::write_block((
        //             "EXTERNAL_PRESSURE".to_string(),
        //             format!(
        //                 r#"{:16.10}{:16.10}{:16.10}
        //                 {:16.10}{:16.10}
        //                                 {:16.10}
        // "#,
        //                 rxx, rxy, rxz, ryy, ryz, rzz
        //             ),
        //         ));
        //         let mut misc = String::new();
        //         misc.push_str(&fix);
        //         misc.push_str(&external_efield);
        //         misc.push_str(&external_pressure);
        //         misc
    }
    /**
    Species and mass table
    # Example:
    ```
    %BLOCK SPECIES_MASS
           O     15.9989995956
          Al     26.9820003510
          Ti     47.9000015259
          Cs    132.9049987793
    %ENDBLOCK SPECIES_MASS
    ```
    */
    fn species_mass(&self) -> String {
        todo!()
        // let element_list = self.element_set();
        // let mass_strings: Vec<String> = element_list
        //     .iter()
        //     .map(|elm| -> String {
        //         let mass: f64 = ELEMENT_TABLE.get_by_symbol(elm).unwrap().mass();
        //         format!("{:>8}{:17.10}\n", elm, mass)
        //     })
        //     .collect();
        // Cell::write_block(("SPECIES_MASS".to_string(), mass_strings.concat()))
    }
    /**
    Species and potential table
    # Example:
    ```
    %BLOCK SPECIES_POT
       O  O_00.usp
      Al  Al_00.usp
      Ti  Ti_00.uspcc
      Cs  Cs_00.usp
    %ENDBLOCK SPECIES_POT
    ```
    */
    fn species_pot_str(&self) -> String {
        todo!()
        // let element_list = self.element_set();
        // let pot_strings: Vec<String> = element_list
        //     .iter()
        //     .map(|elm| {
        //         let pot_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
        //         format!("{:>8}  {}\n", elm, pot_file)
        //     })
        //     .collect();
        // Cell::write_block(("SPECIES_POT".to_string(), pot_strings.concat()))
    }
    /**
    This data block defines the size of the LCAO basis set used for population analysis.
    # Example:
    ```
    %BLOCK SPECIES_LCAO_STATES
       O         2
      Al         2
      Ti         3
      Cs         4
    %ENDBLOCK SPECIES_LCAO_STATES
    ```
    */
    fn species_lcao_str(&self) -> String {
        todo!()
        // let element_list = self.element_set();
        // let lcao_strings: Vec<String> = element_list
        //     .iter()
        //     .map(|elm| {
        //         let lcao_state = ELEMENT_TABLE.get_by_symbol(elm).unwrap().lcao();
        //         format!("{:>8}{:9}\n", elm, lcao_state)
        //     })
        //     .collect();
        // Cell::write_block(("SPECIES_LCAO_STATES".to_string(), lcao_strings.concat()))
    }
}
