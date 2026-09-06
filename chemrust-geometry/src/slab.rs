use castep_periodic_table::element::ElementSymbol;
use nalgebra::Matrix3;

use crate::coords::FracCoord;
use crate::lattice::LatticeVectors;
use crate::structure::Structure;
use crate::transform::TransformMatrix;

/// Build FCC bulk conventional cell.
///
/// The conventional cell has 4 atoms (Fm-3m, #225):
/// - Cu at (0, 0, 0), (0, 1/2, 1/2), (1/2, 0, 1/2), (1/2, 1/2, 0)
pub fn fcc_bulk(a: f64, species: ElementSymbol) -> Structure {
    let cell = LatticeVectors::new(Matrix3::identity() * a);
    let frac_coords = vec![
        FracCoord::new(0.0, 0.0, 0.0),
        FracCoord::new(0.0, 0.5, 0.5),
        FracCoord::new(0.5, 0.0, 0.5),
        FracCoord::new(0.5, 0.5, 0.0),
    ];
    let n = frac_coords.len();
    Structure::new(
        vec![species; n],
        frac_coords,
        Some(cell),
        [true, true, true],
        vec![0_i32; n],
        vec![None; n],
        None,
    )
}

/// Compute the transform for adding vacuum along c.
pub fn vacuum_gap_matrix(cell: &LatticeVectors, gap_ang: f64) -> TransformMatrix {
    let c_old = cell.lengths().2;
    let c_new = c_old + gap_ang;
    let scale = c_old / c_new;
    TransformMatrix::from_linear(Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, scale,
    ))
}

/// Build a Cu(111) 4-layer slab with a 2x2 surface cell and 12 A vacuum.
///
/// Pipeline:
///   fcc_bulk -> SurfaceRotation(111) -> supercell(2,2,1) -> replicate_along_c(4)
///   -> align_axes -> add_vacuum_gap -> wrap_frac_coords
///
/// The SurfaceRotation produces a hexagonal in-plane cell for (111).
/// The supercell replicates atoms so the 2x2 cell contains 16 Cu atoms.
/// replicate_along_c(4) stacks 4 layers along c.
pub fn cu111_4layer(a: f64) -> Structure {
    use crate::transform::SurfaceRotation;

    fcc_bulk(a, ElementSymbol::Cu)
        .transform(SurfaceRotation::new(1, 1, 1))
        .apply()
        .supercell(2, 2, 1)
        .replicate_along_c(4)
        .align_axes()
        .add_vacuum_gap(12.0)
        .wrap_frac_coords()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fcc_has_4_atoms() {
        let s = fcc_bulk(3.615, ElementSymbol::Cu);
        assert_eq!(s.num_atoms(), 4);
        assert!(s.cell.is_some());
    }

    #[test]
    fn cu111_slab_has_4_layers() {
        let s = cu111_4layer(3.615);
        let n_cu = s.species.iter().filter(|&sp| *sp == ElementSymbol::Cu).count();
        // 4 atoms after rotation × 4 in-plane copies × 4 layers = 64 Cu atoms
        assert_eq!(n_cu, 64, "expected 64 Cu, got {n_cu}");
        let n_top = s.tags.iter().filter(|&&t| t == 3).count();
        assert_eq!(n_top, 16, "expected 16 top-layer atoms, got {n_top}");
    }
}
