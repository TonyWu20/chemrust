use castep_periodic_table::{data::ELEMENT_TABLE, element::LookupElement};

use crate::Cell;

use super::CellSettingExport;

#[derive(Debug, Clone)]
pub struct SpeciesCharacteristics {
    element_list: Vec<String>,
}

impl CellSettingExport for SpeciesCharacteristics {
    fn write_to_cell(&self) -> String {
        let mass = self.species_mass();
        let pot = self.species_potentials();
        let lcao = self.species_lcao_states();
        let species_output = vec![mass, pot, lcao];
        species_output.concat()
    }
}

impl SpeciesCharacteristics {
    pub fn new(element_list: Vec<String>) -> Self {
        Self { element_list }
    }
    pub fn species_mass(&self) -> String {
        let text = self
            .element_list
            .iter()
            .map(|symbol| {
                let mass = ELEMENT_TABLE.get_by_symbol(&symbol).unwrap().mass;
                format!("{:>8}{:17.10}\n", symbol, mass)
            })
            .collect::<Vec<String>>()
            .concat();
        Cell::write_block(("SPECIES_MASS".into(), text))
    }

    pub fn species_potentials(&self) -> String {
        let text = self
            .element_list
            .iter()
            .map(|symbol| {
                let pot_file = ELEMENT_TABLE.get_by_symbol(&symbol).unwrap().potential();
                format!("{:>8}  {}\n", symbol, pot_file)
            })
            .collect::<Vec<String>>()
            .concat();
        Cell::write_block(("SPECIES_POT".into(), text))
    }

    pub fn species_lcao_states(&self) -> String {
        let text = self
            .element_list
            .iter()
            .map(|symbol| {
                let lcao_state = ELEMENT_TABLE.get_by_symbol(&symbol).unwrap().lcao();
                format!("{:>8}{:9}\n", symbol, lcao_state)
            })
            .collect::<Vec<String>>()
            .concat();
        Cell::write_block(("SPECIES_LCAO".into(), text))
    }
}
