use nalgebra::Matrix3;

use crate::{kpoint::KPoint, symmetry_operations::SymmetryOperation};

/// Apply rotation operation to a k-point, return a new instance of k-point
pub fn apply_rotation_to_kpt(rotation_matrix: &Matrix3<f64>, kpoint: &KPoint) -> KPoint {
    KPoint::new(rotation_matrix * kpoint.coord())
}

/// Apply the group of symmetry operations for the given symmetry,
/// determine if the given k-point is an irreducible k-point under all symmetry operations.
pub fn is_irreducible<T: SymmetryOperation>(k: &KPoint, group: &[T]) -> bool {
    group.iter().all(|ops|
            // If the dict order is greater than it is an irreducible points
            apply_rotation_to_kpt(&ops.rotation(), k) >= *k)
}

/// Generate irreducible kpoints with given symmetry group operations.
/// # Note
/// It follows `castep`'s convention to let fully inversed kpoints (-x, -y, -z) to be degenerate with (x, y, z).
/// The output vec is sorted in the lex order of the vector coordinates and in reverse order (Greater to less).
pub fn reduce_kpoints<T: SymmetryOperation>(kpts: &[KPoint], group: &[T]) -> Vec<KPoint> {
    let first_stage_results = kpts
        .iter()
        .filter(|&kpt| is_irreducible(kpt, group))
        .cloned()
        .collect::<Vec<KPoint>>();
    let mut irreducible_kpts = first_stage_results
        .iter()
        .filter(|&kpt| {
            // Check full inverse degeneracy
            !first_stage_results.contains(&kpt.inv()) // Do not have full inversed counterparts
            || kpt >= &kpt.inv() // Or, it is greater than the counterpart, so the one to keep has more positive values in xyz
        })
        .cloned()
        .collect::<Vec<KPoint>>();
    irreducible_kpts.sort_by(|a, b| a.partial_cmp(b).expect("Have order").reverse());
    irreducible_kpts
}
