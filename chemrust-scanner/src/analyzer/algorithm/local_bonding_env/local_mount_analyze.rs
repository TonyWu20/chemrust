//! Analyze the local bonding environment to get possible mounting site positions.
//! To avoid wasting efforts in repeated atom pairs, which are shared by two neighboring LBEs,
//! a table of tested pairs should be documented. So a global hashset table of atom id pairs, sorted
//! by id in ascending order, will be kept during the LBEs walkthrough.
//! When examining a pair of atoms, their atom ids are sorted to return a pair, (id_smaller, id_larger),
//! then the global table is consulted. If this pair exists, skip the mounting scanning, else perform
//! the scanning and add the pair to the table.

use std::collections::{HashMap, HashSet};

use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{Element, LookupElement},
};
use chemrust_core::data::Atom;

use crate::analyzer::algorithm::BondingCircle;

use super::{
    bonding_scheme::{is_bonded, LOWER_FAC, UPPER_FAC},
    LocalBondingEnv,
};

pub struct LBEMountingChecker<'a> {
    local_bonding_envs: Vec<LocalBondingEnv<'a>>,
    target_bondlength: f64,
    target_atom_covalent_radius: f64,
    checked_spheres_result: HashMap<[usize; 2], Option<BondingCircle>>,
}

impl<'a> LBEMountingChecker<'a> {
    pub fn new(
        local_bonding_envs: Vec<LocalBondingEnv<'a>>,
        target_bondlength: f64,
        target_atom_covalent_radius: f64,
    ) -> Self {
        let checked_spheres_result = HashMap::new();
        Self {
            local_bonding_envs,
            target_bondlength,
            target_atom_covalent_radius,
            checked_spheres_result,
        }
    }
}
