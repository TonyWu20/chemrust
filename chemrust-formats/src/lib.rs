#![allow(dead_code)]
extern crate castep_periodic_table as cpt;

mod param_writer;
mod structure_files;
pub use param_writer::{castep_param, cell_settings};
pub use structure_files::*;

pub trait ModelFormat {}
