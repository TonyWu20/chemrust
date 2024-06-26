use nalgebra::Point3;

use crate::analyzer::geometry::{Arc, Circle, Sphere};

mod visualize;
pub use visualize::Visualize;

/// Sphere with location atom information
#[derive(Debug, Clone, Copy)]
pub struct BondingSphere {
    sphere: Sphere,
    locating_atom_id: usize,
}

impl BondingSphere {
    pub fn new(sphere: Sphere, locating_atom_id: usize) -> Self {
        Self {
            sphere,
            locating_atom_id,
        }
    }

    pub fn sphere(&self) -> Sphere {
        self.sphere
    }

    pub fn locating_atom_id(&self) -> usize {
        self.locating_atom_id
    }
}

/// Circle with bonding atoms information
#[derive(Debug, Clone, Copy)]
pub struct BondingCircle {
    circle: Circle,
    connecting_atom_ids: [usize; 2],
}

impl BondingCircle {
    pub fn new(circle: Circle, connecting_atoms: [usize; 2]) -> Self {
        Self {
            circle,
            connecting_atom_ids: connecting_atoms,
        }
    }

    pub fn circle(&self) -> Circle {
        self.circle
    }

    pub fn connecting_atoms(&self) -> [usize; 2] {
        self.connecting_atom_ids
    }
}

/// When the circle is interfered by certain points
#[derive(Debug, Clone)]
pub struct BondingArc {
    arc: Arc,
    connecting_atom_ids: [usize; 2],
}

impl BondingArc {
    pub fn new(arc: Arc, connecting_atom_ids: [usize; 2]) -> Self {
        Self {
            arc,
            connecting_atom_ids,
        }
    }
}

/// Point with bonding atom information
#[derive(Debug, Clone)]
pub struct CoordinationPoint {
    coord: Point3<f64>,
    connecting_atom_ids: Vec<usize>,
    cn: u32,
}
impl CoordinationPoint {
    pub fn new(coord: Point3<f64>, connecting_atom_ids: Vec<usize>, cn: u32) -> Self {
        Self {
            coord,
            connecting_atom_ids,
            cn,
        }
    }

    pub fn merge_with(self, rhs: Self) -> Option<CoordinationPoint> {
        if self.coord == rhs.coord {
            let new_connecting_atom_ids = [self.connecting_atom_ids, rhs.connecting_atom_ids];
            let mut new_connecting_atom_ids = new_connecting_atom_ids.concat();
            new_connecting_atom_ids.sort();
            new_connecting_atom_ids.dedup();
            let new_cn = new_connecting_atom_ids.len();
            Some(CoordinationPoint {
                coord: self.coord,
                connecting_atom_ids: new_connecting_atom_ids,
                cn: new_cn as u32,
            })
        } else {
            None
        }
    }

    pub fn connecting_atom_ids(&self) -> &[usize] {
        self.connecting_atom_ids.as_ref()
    }

    pub fn cn(&self) -> u32 {
        self.cn
    }

    pub fn coord(&self) -> Point3<f64> {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Point3<f64>) {
        self.coord = coord;
    }
}
