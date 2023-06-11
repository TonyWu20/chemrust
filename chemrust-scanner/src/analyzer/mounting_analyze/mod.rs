use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{Element, LookupElement},
};
use chemrust_core::data::Atom;

use crate::analyzer::algorithm::{ideal_bondlength, is_bonded};

pub const LOWER_FAC: f64 = 0.6;
pub const UPPER_FAC: f64 = 1.15;

#[derive(Debug, Clone)]
pub struct MountingChecker {
    mount_element: Element,
    mount_distance: f64,
}

impl MountingChecker {
    pub fn new_builder() -> MountingCheckerBuilder {
        MountingCheckerBuilder::new()
    }
    fn available_atoms<'a>(&self, atoms: &'a [Atom]) -> Vec<Atom> {
        atoms
            .iter()
            .filter(|atom| {
                let ideal_bondlength =
                    ideal_bondlength(atom.atomic_number(), self.mount_element.atomic_number());
                is_bonded(self.mount_distance, ideal_bondlength, LOWER_FAC, UPPER_FAC)
            })
            .map(|atom| atom.clone())
            .collect()
    }
    pub fn sphere_stages(&self, atoms: &[Atom]) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct MountingCheckerBuilder {
    mount_element: Option<Element>,
    mount_distance: f64,
}

impl MountingCheckerBuilder {
    pub fn new() -> Self {
        Self {
            mount_element: None,
            mount_distance: 0.0,
        }
    }
    pub fn with_element(self, element: &Element) -> Self {
        Self {
            mount_element: Some(element.clone()),
            ..self
        }
    }
    pub fn with_bondlength(self, bond_length: f64) -> Self {
        Self {
            mount_distance: bond_length,
            ..self
        }
    }
    pub fn build(self) -> MountingChecker {
        let mount_element = self
            .mount_element
            .unwrap_or(ELEMENT_TABLE.get_by_symbol("H").unwrap().clone());
        MountingChecker {
            mount_element,
            mount_distance: self.mount_distance,
        }
    }
}
