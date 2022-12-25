#![allow(dead_code)]
pub mod data;
pub mod system;
// pub mod atom;
// pub mod bond;
// pub mod builder_typestate;
// pub mod error;
// pub mod formats;
// pub mod lattice;
// pub mod params;
// #[cfg(test)]
// mod test;

use castep_periodic_table as cpt;
use nalgebra as na;

use na::UnitQuaternion;
use std::fmt::Debug;

/// Trait bound for Model Formats
pub trait ModelInfo: Debug + Clone + Default {}
/// Transformation for atoms and lattices.
pub trait Transformation {
    fn rotate(&mut self, rotate_quatd: &UnitQuaternion<f64>);
    fn translate(&mut self, translate_matrix: &na::Translation<f64, 3>);
}
