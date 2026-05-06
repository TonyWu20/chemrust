#![allow(unused)]
use nalgebra::Vector3;

use crate::kpoints::KPoint;

mod mp_spacing;

pub use mp_spacing::MPSpacingParam;

#[derive(Debug, Clone, Copy)]
pub struct MPGrid {
    grid: [usize; 3],
    shift: [f64; 3],
}

impl MPGrid {
    pub fn new(grid: [usize; 3], shift: [f64; 3]) -> Self {
        Self { grid, shift }
    }

    pub fn generate_reducible_kpts(&self) -> Vec<KPoint> {
        let [nx, ny, nz] = self.grid;
        let [sx, sy, sz] = self.shift;
        (1..=nx)
            .flat_map(move |ix| {
                (1..=ny).flat_map(move |iy| {
                    (1..=nz).map(move |iz| {
                        let x = mp_mesh_point(ix, nx, sx);
                        let y = mp_mesh_point(iy, ny, sy);
                        let z = mp_mesh_point(iz, nz, sz);
                        KPoint::new(Vector3::new(x, y, z))
                    })
                })
            })
            .collect()
    }
}

///  (2r - q - 1 + sx) / 2q
fn mp_mesh_point(cur_point: usize, num_point: usize, shift: f64) -> f64 {
    ((2 * cur_point as isize - num_point as isize - 1) as f64 + shift) / ((2 * num_point) as f64)
}
