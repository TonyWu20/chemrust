use nalgebra::Matrix3;

use crate::{
    kpoints::{IrreducibleKpt, KPoint},
    symmetry_operations::SymmetryOperation,
};

/// Apply rotation operation to a k-point, return a new instance of k-point
pub(crate) fn apply_rotation_to_kpt(rotation_matrix: &Matrix3<f64>, kpoint: &KPoint) -> KPoint {
    KPoint::new(rotation_matrix * kpoint.coord())
}

/// Apply the group of symmetry operations for the given symmetry,
/// determine if the given k-point is an irreducible k-point under all symmetry operations.
fn is_irreducible<T: SymmetryOperation>(k: &KPoint, group: &[T]) -> bool {
    group.iter().all(|ops|
            // If the dict order is greater than it is an irreducible points
            apply_rotation_to_kpt(&ops.rotation(), k) >= *k)
}

/// Generate irreducible kpoints with given symmetry group operations.
///
/// It follows `castep`'s convention to let fully inversed kpoints
/// (-x, -y, -z) be degenerate with (x, y, z). For each k / -k pair
/// the representative is the MORE-NEGATIVE member. This matches the
/// CASTEP `cell_reduce_kpoints_internal` default (`exclude_minus_k`,
/// cell.f90): the pair merges into the first point of the sort
/// order, which for an equidistant pair is the more-negative one.
/// A point whose partner -k is absent from the set stands alone.
/// A self-invariant point (k == -k, e.g. Gamma on an odd-n grid)
/// carries no partner term in its degeneracy.
///
/// The output vec is sorted in the lex order of the vector
/// coordinates and in reverse order (Greater to less).
pub fn reduce_kpoints<T: SymmetryOperation>(kpts: &[KPoint], group: &[T]) -> Vec<IrreducibleKpt> {
    let first_stage_results = kpts
        .iter()
        .filter(|&kpt| is_irreducible(kpt, group))
        .cloned()
        .collect::<Vec<KPoint>>();
    let mut irreducible_kpts = first_stage_results
        .iter()
        .filter_map(|&kpt| {
            // Live pair bookkeeping under strict coordinate equality.
            // `KPoint`'s `PartialEq` is inversion-aware (a point
            // compares equal to its own inversion), so the check
            // must not use `contains(&kpt.inv())`.
            let partner = first_stage_results.iter().find(|p| p.coord_eq(&kpt.inv()));
            // Partner term of the degeneracy: present only when a
            // distinct -kpt is in the set. A self-invariant point
            // (k == -k) pairs with itself and adds nothing.
            let partner_term = match partner {
                None => 0,
                Some(p) if p.coord_eq(&kpt) => 0,
                Some(_) => 1,
            };
            // No partner: keep the point. Partner present: keep the
            // more-negative member of the k / -k pair.
            let kept = partner.is_none() || kpt <= kpt.inv();
            kept.then_some(IrreducibleKpt::new(kpt, group.len() + partner_term))
        })
        .collect::<Vec<IrreducibleKpt>>();
    irreducible_kpts.sort_by(|a, b| a.kpt().partial_cmp(&b.kpt()).expect("Have order").reverse());
    irreducible_kpts
}

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix3, Vector3};

    use crate::{
        functions::reduce_kpoints,
        kpoints::KPoint,
        symmetry_operations::SymmetryOperation,
    };

    /// A minimal point-group op: the identity or the inversion.
    enum PGOp {
        Identity,
        Inversion,
    }
    impl SymmetryOperation for PGOp {
        fn rotation(&self) -> Matrix3<f64> {
            match self {
                Self::Identity => Matrix3::identity(),
                Self::Inversion => {
                    Matrix3::from([[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]])
                }
            }
        }
    }

    /// CASTEP reference (cell_reduce_kpoints_internal, exclude_minus_k
    /// default, Cu111_CO fixture): the MP pair (+-0.25, 0, 0) merges
    /// into ONE k-point at (-0.25, 0, 0) with the summed weight.
    /// Under the C1 group the pair carries the partner term.
    #[test]
    fn c1_pair_keeps_more_negative_representative() {
        let kpts = [
            KPoint::new(Vector3::new(0.25, 0.0, 0.0)),
            KPoint::new(Vector3::new(-0.25, 0.0, 0.0)),
        ];
        let group = [PGOp::Identity];
        let reduced = reduce_kpoints(&kpts, &group);
        assert_eq!(reduced.len(), 1, "pair merges to one k-point");
        let k = reduced[0].kpt();
        assert!((k.coord().x - -0.25_f64).abs() < 1e-9, "kx = {}", k.coord().x);
        // C1 group (len 1) plus the partner term: 1 + 1 = 2.
        assert_eq!(reduced[0].degeneracy(), 2);
    }

    /// A self-invariant point (k == -k, Gamma) has no distinct
    /// partner and gets no partner term: group.len(), not group.len() + 1.
    #[test]
    fn self_invariant_point_gets_no_partner_term() {
        let kpts = [
            KPoint::new(Vector3::new(0.0, 0.0, 0.0)),
            KPoint::new(Vector3::new(0.25, 0.0, 0.0)),
            KPoint::new(Vector3::new(-0.25, 0.0, 0.0)),
        ];
        let group = [PGOp::Identity];
        let reduced = reduce_kpoints(&kpts, &group);
        assert_eq!(reduced.len(), 2);
        let gamma = reduced
            .iter()
            .find(|k| (k.kpt().coord().x).abs() < 1e-9)
            .expect("Gamma present");
        assert_eq!(gamma.degeneracy(), 1, "Gamma: group.len() = 1");
        let pair = reduced
            .iter()
            .find(|k| (k.kpt().coord().x - -0.25_f64).abs() < 1e-9)
            .expect("-0.25 present");
        assert_eq!(pair.degeneracy(), 2, "pair: group.len() + 1 = 2");
    }

    /// A point whose partner -k is absent from the set stands alone:
    /// it is kept and carries no partner term. The old
    /// `contains(&kpt.inv())` check was dead under the inversion-aware
    /// `PartialEq` and misfired into the comparator, dropping it.
    #[test]
    fn singleton_without_partner_is_kept() {
        let kpts = [KPoint::new(Vector3::new(0.25, 0.0, 0.0))];
        let group = [PGOp::Identity];
        let reduced = reduce_kpoints(&kpts, &group);
        assert_eq!(reduced.len(), 1);
        assert_eq!(reduced[0].degeneracy(), 1, "no partner term: group.len()");
    }

    /// Passing the inversion op in the group must not double-count the
    /// partner term: the first stage already consumes the pair, and the
    /// second stage sees the survivor as a singleton.
    #[test]
    fn inversion_in_group_does_not_double_count() {
        let kpts = [
            KPoint::new(Vector3::new(0.25, 0.0, 0.0)),
            KPoint::new(Vector3::new(-0.25, 0.0, 0.0)),
        ];
        let group = [PGOp::Identity, PGOp::Inversion];
        let reduced = reduce_kpoints(&kpts, &group);
        assert_eq!(reduced.len(), 1, "the pair still merges to one k-point");
        let k = reduced[0].kpt();
        assert!((k.coord().x - -0.25_f64).abs() < 1e-9, "kx = {}", k.coord().x);
        // The pair is consumed by the inversion op (group.len() = 2).
        // The partner term is not added again: degeneracy = 2.
        assert_eq!(reduced[0].degeneracy(), 2);
    }
}
