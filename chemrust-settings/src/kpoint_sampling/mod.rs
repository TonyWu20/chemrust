use std::f64::consts::PI;

use chemrust_core::data::LatticeVectors;
use crystallographic_group::{
    BravaisLattice, CrystalSystem, SpaceGroup,
};
use nalgebra::Matrix3;

use self::helper_functions::mp_grid_generate;

mod helper_functions;

#[derive(Debug, Clone, Copy)]
pub struct ReciprocalLatticeVectors(Matrix3<f64>);

#[derive(Debug, Clone)]
pub struct ReciprocalLatticeSpace<S: CrystalSystem, B: BravaisLattice> {
    reciprocal_vector: ReciprocalLatticeVectors,
    space_group: SpaceGroup<S, B>,
}

impl<S: CrystalSystem, B: BravaisLattice> ReciprocalLatticeSpace<S, B> {
    pub fn new(lattice_vectors: LatticeVectors, space_group: SpaceGroup<S, B>) -> Self {
        let reciprocal_vector = lattice_vectors.into();
        Self {
            reciprocal_vector,
            space_group,
        }
    }
}

impl ReciprocalLatticeVectors {
    pub fn data(&self) -> &Matrix3<f64> {
        &self.0
    }
    pub fn norm(&self) -> [f64; 3] {
        let a = self.0.column(0);
        let b = self.0.column(1);
        let c = self.0.column(2);
        let a_star = b.cross(&c).scale(1_f64 / a.dot(&b.cross(&c)));
        let b_star = c.cross(&a).scale(1_f64 / b.dot(&c.cross(&a)));
        let c_star = a.cross(&b).scale(1_f64 / c.dot(&a.cross(&b)));
        [a_star.norm(), b_star.norm(), c_star.norm()]
    }
}

impl From<LatticeVectors> for ReciprocalLatticeVectors {
    fn from(value: LatticeVectors) -> Self {
        Self(
            value
                .data()
                .try_inverse()
                .unwrap()
                .scale(2_f64 * PI)
                .transpose(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KPointQuality {
    Coarse,
    Medium,
    Fine,
}

#[derive(Debug, Clone, Copy)]
pub struct KPointSampler {
    is_metal: bool,
    quality: KPointQuality,
}

impl KPointSampler {
    pub fn new(is_metal: bool, quality: KPointQuality) -> Self {
        Self { is_metal, quality }
    }
    fn kpoint_separation(&self) -> f64 {
        if self.is_metal {
            match self.quality {
                KPointQuality::Coarse => 0.07,
                KPointQuality::Medium => 0.05,
                KPointQuality::Fine => 0.04,
            }
        } else {
            match self.quality {
                KPointQuality::Coarse => 0.1,
                KPointQuality::Medium => 0.08,
                KPointQuality::Fine => 0.07,
            }
        }
    }
    fn derive_kpoints<S: CrystalSystem, B: BravaisLattice>(
        &self,
        reciprocal_lattice_space: &ReciprocalLatticeSpace<S, B>,
    ) {
        let spacing = self.kpoint_separation();
        let grid_size = reciprocal_lattice_space.reciprocal_vector.norm();
        let _mp_grid = mp_grid_generate(&grid_size, spacing);
        todo!()
    }
}
