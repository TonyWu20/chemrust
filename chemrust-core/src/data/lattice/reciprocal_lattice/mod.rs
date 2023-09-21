use std::f64::consts::PI;

use crystallographic_group::{BravaisLattice, CrystalSystem, SpaceGroup};

use nalgebra::{Matrix3, Matrix4};

use super::LatticeVectors;

mod mp_space;

#[derive(Debug, Clone, Copy)]
pub struct ReciprocalLatVec(Matrix3<f64>);

#[derive(Debug, Clone)]
pub struct SymmetryLattice<C, B>
where
    C: CrystalSystem,
    B: BravaisLattice,
{
    pub(crate) reciprocal_lattice_vector: ReciprocalLatVec,
    pub(crate) space_group: SpaceGroup<C, B>,
}

impl<C, B> SymmetryLattice<C, B>
where
    C: CrystalSystem,
    B: BravaisLattice,
{
    pub fn new(lattice_vectors: &LatticeVectors, space_group: SpaceGroup<C, B>) -> Self {
        Self {
            reciprocal_lattice_vector: lattice_vectors.clone().into(),
            space_group,
        }
    }
    pub fn symmetry_ops(&self) -> &[Matrix4<f64>] {
        self.space_group.generators()
    }
}

impl From<LatticeVectors> for ReciprocalLatVec {
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

impl ReciprocalLatVec {
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
