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

/// Build a Cu(111) slab with N layers and a 2x2 surface cell.
///
/// Uses the full pipeline:
///   fcc_bulk -> SurfaceRotation(111) -> Supercell(2,2,1) -> replicate_along_c(3)
///   -> explicit 4th layer -> add_vacuum_gap
///
/// The trilayer (replicate_along_c(3)) gives ABC stacking at z=0, 1/3, 2/3.
/// The 4th layer duplicates atoms at integer z-values and shifts them by +1.
pub fn cu111_4layer(a: f64) -> Structure {
    use crate::transform::{SurfaceRotation, Supercell};

    // Build trilayer (ABC): after surface rotation, 2x2 supercell, replicate 3x
    let mut surf = fcc_bulk(a, ElementSymbol::Cu)
        .transform(SurfaceRotation::new(1, 1, 1))
        .transform(Supercell::new(2, 2, 1))
        .apply()
        .replicate_along_c(3);

    // Add 4th A' layer: duplicate atoms at integer z, shift z -> z+1
    let mut add_sp = Vec::new();
    let mut add_coords = Vec::new();
    let mut add_tags = Vec::new();
    let mut add_labels = Vec::new();

    for i in 0..surf.num_atoms() {
        let z = surf.frac_coords[i][2];
        // Check if z is near an integer (0, 1, 2, ...)
        if (z - z.round()).abs() < 1e-6 {
            add_sp.push(surf.species[i]);
            add_coords.push(FracCoord::new(surf.frac_coords[i][0], surf.frac_coords[i][1], z.round() + 1.0));
            add_tags.push(3_i32); // layer 4
            add_labels.push(None);
        }
    }

    surf = surf.with_atoms(add_sp, add_coords, add_tags, add_labels);
    surf = surf.align_axes();
    surf.add_vacuum_gap(12.0)
}

/// Build the full Cu(111)+CO system.
///
/// - Cu(111) 4-layer slab, 2x2 surface cell, ~12 A vacuum
/// - CO adsorbate at atop position (above center of top Cu layer)
/// - Returns 18 atoms: 16 Cu + 1 C + 1 O
pub fn cu111_co_system(a: f64) -> Structure {
    let mut sys = cu111_4layer(a).apply();

    // Find the top-layer Cu nearest the cell center (x,y = 0.5, 0.5)
    let top_cu = sys.species.iter()
        .zip(sys.frac_coords.iter())
        .zip(sys.tags.iter())
        .enumerate()
        .filter(|(_, ((sp, _), &tag))| **sp == ElementSymbol::Cu && tag == 3)
        .min_by(|(_, ((_, a), _)), (_, ((_, b), _))| {
            let da = (a[0] - 0.5).powi(2) + (a[1] - 0.5).powi(2);
            let db = (b[0] - 0.5).powi(2) + (b[1] - 0.5).powi(2);
            da.total_cmp(&db)
        })
        .map(|(i, _)| i)
        .expect("No top-layer Cu found");

    // CO vertical atop: convert bond lengths to fractional using c-length
    let c_len = sys.require_cell().unwrap().lengths().2;
    let z_cu = sys.frac_coords[top_cu][2];
    let z_c = z_cu + 1.9 / c_len;
    let z_o = z_cu + (1.9 + 1.15) / c_len;

    sys = sys.with_atoms(
        vec![ElementSymbol::C, ElementSymbol::O],
        vec![FracCoord::new(0.5, 0.5, z_c), FracCoord::new(0.5, 0.5, z_o)],
        vec![-1, -1],
        vec![Some("C_atop".into()), Some("O_atop".into())],
    );

    sys.pbc = [true, true, false];
    sys.wrap_frac_coords()
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
    fn cu111_slab_has_16_cu() {
        let s = cu111_4layer(3.615);
        let n_cu = s.species.iter().filter(|&sp| *sp == ElementSymbol::Cu).count();
        assert_eq!(n_cu, 16, "expected 16 Cu, got {n_cu}");
        let n_top = s.tags.iter().filter(|&&t| t == 3).count();
        assert_eq!(n_top, 4, "expected 4 top-layer atoms, got {n_top}");
    }

    #[test]
    fn cu111_co_has_18_atoms() {
        let s = cu111_co_system(3.615);
        assert_eq!(s.species.len(), 18);
        assert_eq!(s.species.iter().filter(|&sp| *sp == ElementSymbol::Cu).count(), 16);
        assert_eq!(s.species.iter().filter(|&sp| *sp == ElementSymbol::C).count(), 1);
        assert_eq!(s.species.iter().filter(|&sp| *sp == ElementSymbol::O).count(), 1);
        assert!(s.cell.is_some());
    }
}
