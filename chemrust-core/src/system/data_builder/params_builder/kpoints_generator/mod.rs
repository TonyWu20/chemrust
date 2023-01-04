use std::{f64::consts::PI, marker::PhantomData};

use nalgebra::Matrix3;

use crate::data::{format::DataFormat, lattice::LatticeVectors};

use self::helper_functions::mp_grid_generate;
use self::param_markers::{Coarse, IsMetal, SamplingQuality, Yes};

use super::KPoint;

mod helper_functions;
mod param_markers;
mod symmetry_reduction;

pub use symmetry_reduction::*;

pub struct ReciprocalLatticeVector<T: DataFormat> {
    data: Matrix3<f64>,
    format: PhantomData<T>,
}

impl<T> From<LatticeVectors<T>> for ReciprocalLatticeVector<T>
where
    T: DataFormat,
{
    fn from(value: LatticeVectors<T>) -> Self {
        Self {
            data: value
                .data()
                .try_inverse()
                .unwrap()
                .scale(2_f64 * PI)
                .transpose(),
            format: PhantomData,
        }
    }
}

impl<T: DataFormat> ReciprocalLatticeVector<T> {
    fn norms(&self) -> [f64; 3] {
        let a = self.data.column(0);
        let b = self.data.column(1);
        let c = self.data.column(2);
        let a_star = b.cross(&c).scale(1_f64 / a.dot(&b.cross(&c)));
        let b_star = c.cross(&a).scale(1_f64 / b.dot(&c.cross(&a)));
        let c_star = a.cross(&b).scale(1_f64 / c.dot(&a.cross(&b)));
        [a_star.norm(), b_star.norm(), c_star.norm()]
    }
}

pub struct KPointSampler<M, Q>
where
    M: IsMetal,
    Q: SamplingQuality,
{
    is_metal: PhantomData<M>,
    quality: PhantomData<Q>,
}

pub trait KPointGenerator<T: DataFormat> {
    fn derive_kpoints(
        &self,
        reciprocal_vectors: &ReciprocalLatticeVector<T>,
        symmetry_ops: &[Matrix3<f64>],
    ) -> Vec<KPoint<T>>;
}

impl<T: DataFormat> KPointGenerator<T> for KPointSampler<Yes, Coarse> {
    fn derive_kpoints(
        &self,
        reciprocal_vectors: &ReciprocalLatticeVector<T>,
        symmetry_ops: &[Matrix3<f64>],
    ) -> Vec<KPoint<T>> {
        let spacing = 0.07;
        let grid_size = reciprocal_vectors.norms();
        let mp_grid = mp_grid_generate(&grid_size, spacing);
        let space = SymSpace::new(mp_grid, symmetry_ops, false);
        space.weighted_kpoints()
    }
}
