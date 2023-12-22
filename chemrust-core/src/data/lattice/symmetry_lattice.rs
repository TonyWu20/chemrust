use std::{f64::consts::PI, marker::PhantomData};

use crystallographic_group::{Basis, BravaisLattice, CrystalSystem, SpaceGroup, Standard, HexBasis};
use itertools::Itertools;
use nalgebra::{Matrix3, Matrix4};

use super::LatticeVectors;

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

#[derive(Debug, Clone, Copy)]
pub struct MPGrid<B: Basis>([u32; 3], PhantomData<B>);

pub struct MPGridGenerator<B: Basis> {
    norms: [f64; 3],
    spacing: f64,
    crystal_system: PhantomData<B>,
}

impl<B> MPGridGenerator<B>
where
    B: Basis,
{
    fn grid_size_determine(length: f64, spacing: f64) -> u32 {
        let div = length / spacing;
        let rounded = div.round();
        if rounded >= 1.0 {
            rounded as u32
        } else {
            1
        }
    }
}

impl MPGridGenerator<Standard> {
    pub fn generate(&self) -> MPGrid<Standard> {
        let grid: Vec<u32> = self
            .norms
            .iter()
            .map(|&i| Self::grid_size_determine(i, self.spacing))
            .collect();
        MPGrid(grid.try_into().unwrap(), PhantomData)
    }
}

impl MPGridGenerator<HexBasis> {
    pub fn generate(&self) -> MPGrid<HexBasis> {
        let 
    }

}

impl<B: Basis> MPGrid<B> {
    pub fn data(&self) -> &[u32; 3] {
        &self.0
    }
    pub fn new(norms: [f64; 3], spacing: f64) -> Self {
        let generator = MPGridGenerator { norms, spacing };
        generator.generate()
    }
    pub fn reducible_kpts(&self) -> Vec<[f64; 3]> {
        let x_coords = fractional_num(self.data()[0]);
        let y_coords = fractional_num(self.data()[1]);
        let z_coords = fractional_num(self.data()[2]);
        // Cartesian product of x_coords x y_coords x z_coords
        x_coords
            .iter()
            .cartesian_product(y_coords.iter())
            .cartesian_product(z_coords.iter())
            .map(|((&a, &b), &c)| [a, b, c])
            .collect()
    }
}

fn fractional_num(q: u32) -> Vec<f64> {
    (1..=q)
        .into_iter()
        .map(|r| (2_f64 * (r as f64) - (q as f64) - 1_f64) / (2_f64 * (q as f64)))
        .collect()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn test_cart_prod() {
        let a = [0, 1, 2, 3];
        let b = [0, 1, 2];
        let c = [0, 1];
        let products: Vec<[i32; 3]> = a
            .iter()
            .cartesian_product(b.iter())
            .cartesian_product(c.iter())
            .map(|((a, b), c)| [*a, *b, *c])
            .collect();
        println!("{:?}", products);
    }
}
