//! chemrust-geometry — Crystal geometry toolkit.
//!
//! Core types for representing and manipulating crystal structures,
//! molecules, and surfaces. No format-specific I/O — that's handled
//! by downstream crates (e.g., `castep-cell-io` for CASTEP).
//!
//! # Main types
//! - [`Structure`] — Struct-of-arrays for any chemical system
//! - [`LatticeVectors`], [`CellConstants`] — Cell representation
//!
//! # Transform pipeline
//! ```rust,ignore
//! use chemrust_geometry::{slab::fcc_bulk, transform::{SurfaceRotation, Supercell}, Structure};
//!
//! let slab = fcc_bulk(3.615, ElementSymbol::Cu)
//!     .transform(SurfaceRotation::new(1, 1, 1))
//!     .transform(Supercell::new(2, 2, 1))
//!     .apply()
//!     .replicate_along_c(4)
//!     .add_vacuum_gap(12.0);
//! ```

pub mod coords;
pub mod error;
pub mod lattice;
pub mod slab;
pub mod structure;
pub mod transform;

pub use castep_periodic_table::element::ElementSymbol;
pub use coords::FracCoord;
pub use error::Error;
pub use lattice::{CellConstants, LatticeVectors, ReciprocalCellParams, ReciprocalCellVectors};
pub use structure::Structure;
