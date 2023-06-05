//! # This module is responsible for analyzing and building `LocalBondingEnv`
//! for all atoms in the given `LatticeModel`.
//! - The local bonding environment is determined as follows:
//!     1. Build an indexed Kd-tree for the coordinates of the atoms.
//!     2. Iterate the coordinates, by looking up the nearest neighbors of the coordinate in the kd-tree.
//!         1. The initial choice of the number of nearest neighbors is 4.
//!             1. Check the element of the coordinate's atom, retrive the `covalent_radius`
//!             2. Check if the distance satisfies `lower fac * ideal distance < distance < upper fac * ideal distance`,
//!         while the `ideal distance` is the sum of `covalent_radius` of two atoms.
//!             3. If number of bonds which meet the criteria is less than 4, report the exact number. If all satisfy,
//!         increment the choice of the number of nearest neighbors by 2, repeat
//!         2. The final number of valid bonds is returned.
//!     3. After a complete iteration, the local bonding environments inside the given structure will be determined.
//!     4. Returns an array of local bonding environments. The new bonding site search will be conducted in each LBE.
use chemrust_core::data::{Atom, LatticeModel};
use kd_tree::KdMap;
use nalgebra::Point3;

use crate::analyzer::algorithm::local_bonding_env::bonding_scheme::{
    ideal_bondlength, is_bonded, LOWER_FAC, UPPER_FAC,
};

use super::LocalBondingEnv;
/// Struct to build a `LocalBondingEnv`
pub struct LocalBondingEnvBuilder<'a> {
    atoms: &'a [Atom],
    coord_kdtree: KdMap<Point3<f64>, CheckAtom>,
}

/// Struct to use for kd-tree. Holds necessary information from `Atom`
#[derive(Debug, Clone, Copy)]
struct CheckAtom {
    atomic_number: u8,
    atom_index: usize,
    coord: Point3<f64>,
}

impl CheckAtom {
    pub fn new(atomic_number: u8, atom_index: usize, coord: Point3<f64>) -> Self {
        Self {
            atomic_number,
            atom_index,
            coord,
        }
    }

    pub fn atomic_number(&self) -> u8 {
        self.atomic_number
    }

    pub fn atom_index(&self) -> usize {
        self.atom_index
    }

    pub fn coord(&self) -> Point3<f64> {
        self.coord
    }
}

impl<'a> LocalBondingEnvBuilder<'a> {
    /// Initiate a builder instance. The lifetime is tied to that of the input `LatticeModel`.
    pub fn new(lattice_model: &'a LatticeModel) -> Self {
        let items = lattice_model
            .atoms()
            .iter()
            .map(|atom| -> (Point3<f64>, CheckAtom) {
                (
                    atom.cartesian_coord(),
                    CheckAtom::new(atom.atomic_number(), atom.index(), atom.cartesian_coord()),
                )
            })
            .collect();
        let coord_kdtree = KdMap::build_by_ordered_float(items);
        Self {
            atoms: lattice_model.atoms(),
            coord_kdtree,
        }
    }
    /// Returns number of bonded atoms around the given atom at given number of neighbors
    /// # Args:
    /// - atom: `&Atom` - Reference of the inspected atom.
    /// - initial_neighbor_num: `usize` - Default will be 5 (means 4 neighbors excluding itself),
    fn local_bonding_env_bonded_num(&self, atom: &Atom, initial_neighbor_num: usize) -> usize {
        let coord = atom.cartesian_coord();
        let found = self.coord_kdtree.nearests(&coord, initial_neighbor_num); // 5 because includes itself
        found
            .iter()
            .skip(1)
            .filter(|item_dist| {
                let atomic_number_found = item_dist.item.1.atomic_number;
                let atomic_number_this = atom.atomic_number();
                let ideal_bondlength = ideal_bondlength(atomic_number_this, atomic_number_found);
                is_bonded(
                    item_dist.squared_distance,
                    ideal_bondlength,
                    LOWER_FAC,
                    UPPER_FAC,
                )
            })
            .count()
    }
    /// Determine the bonding neighbors by definition in `super::bonding_scheme`.
    /// # Args:
    /// - &self
    /// - atom: &Atom
    /// # Returns: the final actual bonding neighbors.
    /// # Notes:
    /// - Starts by checking number of atoms in model. If the model has less than 5 atoms, adjust to fit the case.
    /// - If all neighbors are bonded, increment the `initial_neighbor_num` by 2 per round, stops when `bonding_num` < `initial_neighbor_num`
    fn determine_bonding_neighbors_num(&self, atom: &Atom) -> usize {
        let mut initial_neighbor_num = if self.atoms.len() < 5 {
            self.atoms.len()
        } else {
            5
        };
        let mut bonded_num = self.local_bonding_env_bonded_num(atom, initial_neighbor_num);
        while bonded_num == initial_neighbor_num {
            initial_neighbor_num += 2;
            bonded_num = self.local_bonding_env_bonded_num(atom, initial_neighbor_num);
        }
        bonded_num
    }
    fn get_local_bonding_env(&'a self, atom: &'a Atom) -> LocalBondingEnv {
        let bonding_neighbors_num = self.determine_bonding_neighbors_num(atom);
        let found = self
            .coord_kdtree
            .nearests(&atom.cartesian_coord(), bonding_neighbors_num);
        let atoms: Vec<&Atom> = found
            .iter()
            .map(|res| {
                let index = res.item.1.atom_index();
                self.atoms.get(index).unwrap()
            })
            .collect();
        LocalBondingEnv {
            center_atom: atom,
            number_of_bonding_atoms: bonding_neighbors_num,
            atoms,
        }
    }
    /// Build `LocalBondingEnv` for all `Atom` in `LatticeModel`
    pub fn build_local_bonding_envs(&self) -> Vec<LocalBondingEnv> {
        self.atoms
            .iter()
            .map(|atom| -> LocalBondingEnv { self.get_local_bonding_env(atom) })
            .collect()
    }
}
