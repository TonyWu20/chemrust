/// Module to handle *Monkhorst-Pack* mesh
use std::marker::PhantomData;

use crystallographic_group::{Basis, HexBasis, Standard};

use super::ReciprocalLatVec;

/// The MP-grid size is determined by the reciprocal lattice
/// vector norms and the given spacing.
#[derive(Debug, Clone, Copy)]
pub struct MPGrid([u32; 3]);

pub(crate) trait MPGrid3D {
    fn get_grid(&self) -> &[u32; 3];
}

impl MPGrid3D for MPGrid {
    fn get_grid(&self) -> &[u32; 3] {
        &self.0
    }
}

pub(crate) trait MPMeshing: MPGrid3D {
    const NUM: i32;
    fn mesh(num_points: u32) -> Vec<i32> {
        let q = num_points as i32;
        (1..=q).rev().map(|r| 2 * r - q - Self::NUM).collect()
    }
    fn generate<B: Basis>(&self) -> MPMesh<B> {
        let meshes: [Vec<i32>; 3] = self
            .get_grid()
            .iter()
            .map(|&q| Self::mesh(q))
            .collect::<Vec<Vec<i32>>>()
            .try_into()
            .unwrap();
        MPMesh {
            grid: MPGrid(*self.get_grid()),
            mesh: meshes,
            coordinate_basis: PhantomData,
        }
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

impl<B: Basis> MPGrid3D for MPMeshGenerator<B> {
    fn get_grid(&self) -> &[u32; 3] {
        self.grid.get_grid()
    }
}

impl<B: Basis> MPMeshGenerator<B> {
    pub fn new(grid: MPGrid) -> Self {
        Self {
            grid,
            coordinate_basis: PhantomData,
        }
    }
}

impl MPMeshing for MPMeshGenerator<Standard> {
    const NUM: i32 = 1;
}

impl MPMeshing for MPMeshGenerator<HexBasis> {
    const NUM: i32 = 2;
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
