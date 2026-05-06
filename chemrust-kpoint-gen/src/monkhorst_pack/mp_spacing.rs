use chemrust_geometry::ReciprocalCellParams;

use super::MPGrid;

#[derive(Debug, Clone, Copy)]
pub struct MPSpacingParam {
    /// MP Spacing
    spacing: f64,
    /// Coordinate shift
    shift: [f64; 3],
}

impl MPSpacingParam {
    pub fn new(spacing: f64, shift:[f64;3]) -> Self {
        Self {
            spacing,
            shift
        }
    }
    pub fn build_mp_grid<R: ReciprocalCellParams>(&self, reciprocal: R) -> MPGrid {
        let grid = [
            reciprocal.length_a(),
            reciprocal.length_b(),
            reciprocal.length_c(),
        ]
        .iter()
        .map(|&length| determine_grid_num(length, self.spacing))
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
        MPGrid::new(grid, self.shift)
    }
}

type Spacing = f64;
fn determine_grid_num(length: f64, spacing: f64) -> usize {
    ((length / spacing).ceil() as usize).max(1)
}
