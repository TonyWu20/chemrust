//! This module is to work out the local bonding environment (LBE) around each atom, a prerequiste step to intersect checking.
//! The local bonding environment is determined as follows:
//! 1. Build an indexed Kd-tree for the coordinates of the atoms.
//! 2. Iterate the coordinates, by looking up the nearest neighbors of the coordinate in the kd-tree.
//!     1. The initial choice of the number of nearest neighbors is 4.
//!         1. Check the element of the coordinate's atom, retrive the `covalent_radius`
//!         2. Check if the distance satisfies `lower fac * ideal distance < distance < upper fac * ideal distance`,
//!         while the `ideal distance` is the sum of `covalent_radius` of two atoms.
//!         3. If number of bonds which meet the criteria is less than 4, report the exact number. If all satisfy,
//!         increment the choice of the number of nearest neighbors by 2, repeat
//!     2. The final number of valid bonds is returned.
//! 3. After a complete iteration, the local bonding environments inside the given structure will be determined.
//! 4. Returns an array of local bonding environments. The new bonding site search will be conducted in each LBE.

use chemrust_core::data::Atom;

mod bonding_scheme;
mod builder;
// mod local_mount_analyze;

pub use bonding_scheme::{ideal_bondlength, is_bonded};


#[derive(Debug, Clone)]
/// The local bonding environment around each atom.
/// The lifetime ties to the `Atom` of `LatticeModel`
pub struct LocalBondingEnv<'a> {
    center_atom: &'a Atom,
    number_of_bonding_atoms: usize,
    atoms: Vec<&'a Atom>,
}
