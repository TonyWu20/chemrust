/// Module to handle *Monkhorst-Pack* mesh
use std::marker::PhantomData;

use crystallographic_group::{Basis, HexBasis, Standard};
use nalgebra::{Matrix3, Point3, Vector3};

use super::ReciprocalLatVec;

/// The MP-grid size is determined by the reciprocal lattice
/// vector norms and the given spacing.
#[derive(Debug, Clone, Copy)]
pub struct MPGrid([u32; 3]);

impl MPGrid {
    fn cart_to_frac_matrix(&self) -> Matrix3<f64> {
        let diagonal: Vec<f64> = self.0.iter().map(|&n| 0.5 / n as f64).collect();
        Matrix3::from_diagonal(&Vector3::from_vec(diagonal))
    }
}

fn grid_size_determine(length: f64, spacing: f64) -> u32 {
    let div = length / spacing;
    let rounded = div.round();
    if rounded >= 1.0 {
        rounded as u32
    } else {
        1
    }
}

#[derive(Debug)]
pub struct MPMesh<B: Basis> {
    grid: MPGrid,
    mesh: [Vec<i32>; 3],
    coordinate_basis: PhantomData<B>,
}

#[derive(Debug)]
pub struct MPMeshGenerator<B: Basis> {
    grid: MPGrid,
    coordinate_basis: PhantomData<B>,
}

impl<B: Basis> MPMeshGenerator<B> {
    pub fn new(grid: MPGrid) -> Self {
        Self {
            grid,
            coordinate_basis: PhantomData,
        }
    }
    fn fractional_point(&self, grid_point: &Point3<i32>) -> Point3<f64> {
        let p_f64 = Point3::new(
            grid_point.x as f64,
            grid_point.y as f64,
            grid_point.z as f64,
        );
        self.grid.cart_to_frac_matrix() * p_f64
    }
}

impl MPMeshGenerator<Standard> {
    fn mesh(num_points: u32) -> Vec<i32> {
        let q = num_points as i32;
        (1..=q).into_iter().rev().map(|r| 2 * r - q - 1).collect()
    }
    pub fn generate(&self) -> MPMesh<Standard> {
        let meshes = self
            .grid
            .0
            .iter()
            .map(|&q| Self::mesh(q))
            .collect::<Vec<Vec<i32>>>()
            .try_into()
            .unwrap();
        MPMesh {
            grid: self.grid,
            mesh: meshes,
            coordinate_basis: PhantomData,
        }
    }
}

impl MPMeshGenerator<HexBasis> {
    fn mesh(num_points: u32) -> Vec<i32> {
        let q = num_points as i32;
        (1..=q).into_iter().map(|r| 2 * r - q - 2).collect()
    }
    pub fn generate(&self) -> MPMesh<HexBasis> {
        let meshes = self
            .grid
            .0
            .iter()
            .map(|&q| Self::mesh(q))
            .collect::<Vec<Vec<i32>>>()
            .try_into()
            .unwrap();
        MPMesh {
            grid: self.grid,
            mesh: meshes,
            coordinate_basis: PhantomData,
        }
    }
}

impl ReciprocalLatVec {
    pub fn mp_grid_generate(&self, spacing: f64) -> MPGrid {
        MPGrid(
            self.norm()
                .iter()
                .map(|&i| grid_size_determine(i, spacing))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap(),
        )
    }
}
