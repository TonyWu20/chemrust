//! The core module of `chemrust` is responsible for the abstraction of the data input/output for
//! the chemical models, regardless of the detailed file formats. The designs and implementations
//! of the structs, traits and patterns serve the internal running logics of the library. Only
//! essential data structures and data manipulations are presented here.
#![allow(dead_code)]

/// This module provides the basic supports for builder patterns.
// pub mod builder_state;
/// This module settles the abstraction of essential data in the chemical molecule and lattice models
pub mod data;
