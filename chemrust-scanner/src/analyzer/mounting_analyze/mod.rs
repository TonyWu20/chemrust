use std::collections::HashSet;

use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{Element, LookupElement},
};
use chemrust_core::data::{atom::AtomCollections, Atom};

use crate::{
    analyzer::algorithm::{ideal_bondlength, is_bonded},
    IntersectChecker,
};

use super::algorithm::{FinalReport, Ready};

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
    fn available_atoms(&self, atoms: &[Atom]) -> Vec<Atom> {
        atoms
            .iter()
            .filter(|atom| {
                let ideal_bondlength =
                    ideal_bondlength(atom.atomic_number(), self.mount_element.atomic_number());
                is_bonded(self.mount_distance, ideal_bondlength, LOWER_FAC, UPPER_FAC)
            })
            .cloned()
            .collect()
    }
    pub fn available_elements(&self, atoms: &[Atom]) -> HashSet<String> {
        atoms
            .iter()
            .filter(|atom| {
                let ideal_bondlength =
                    ideal_bondlength(atom.atomic_number(), self.mount_element.atomic_number());
                is_bonded(self.mount_distance, ideal_bondlength, LOWER_FAC, UPPER_FAC)
            })
            .map(|atom| atom.symbol().into())
            .collect::<Vec<String>>()
            .drain(..)
            .collect::<HashSet<String>>()
    }
    pub fn mount_search(&self, model_atoms: &[Atom], to_check_atoms: &[Atom]) -> FinalReport {
        // let available_atoms: Vec<Atom> = self.available_atoms(atoms);
        // println!("available_atoms number: {}", available_atoms.len());
        let collections: AtomCollections = model_atoms.into();
        let to_check_atom_collections: AtomCollections = to_check_atoms.into();
        let coords = collections.cartesian_coords().to_vec();
        let to_check_coords = to_check_atom_collections.cartesian_coords().to_vec();
        IntersectChecker::<Ready>::new(&coords)
            .set_check_atoms(&to_check_coords)
            .start_with_radius(self.mount_distance)
            .check_spheres()
            .analyze_circle_intersects()
            .analyze_points()
            .report()
            .clone()
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

impl Default for MountingCheckerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
