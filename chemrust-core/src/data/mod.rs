//! For a chemical structure model, we need the following hierarchy:
//! - atoms
//! - structure formed by a collection of atoms
//! - lattice if existed
//! Practically speaking, in dealing with the import/export of a
//! structure file, we need to parse the following properties for
//! atoms: element symbols/atomic number, index in collection, coordinates.
//!
//! For a structure with the lattice information, the lattice vectors are
//! mandatory; symmetry information is optional.

pub mod atom;
// pub mod custom_data_type;
pub mod geom;
pub mod lattice;
