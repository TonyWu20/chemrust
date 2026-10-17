//! Code-agnostic k-point sampling settings.
//!
//! [`KPointSampling`] describes how to sample the Brillouin zone without
//! referring to any electronic-structure package. Code-specific settings
//! adapters translate their native types into this spec and back.

use chemrust_geometry::ReciprocalCellParams;

use crate::functions::reduce_kpoints;
use crate::kpoints::{IrreducibleKpt, KPoint};
use crate::monkhorst_pack::{MPGrid, MPSpacingParam};
use crate::symmetry_operations::SymmetryOperation;

/// Code-agnostic k-point sampling specification.
#[derive(Debug, Clone, PartialEq)]
pub enum KPointSampling {
    /// Monkhorst-Pack grid.
    ///
    /// `grid` is the per-axis point count. `shift` follows the `MPGrid`
    /// convention: it is added before the division by `2n` in the MP mesh
    /// formula.
    Mp { grid: [u32; 3], shift: [f64; 3] },

    /// Target MP grid spacing with the same `shift` convention as
    /// [`KPointSampling::Mp`].
    ///
    /// `spacing` is compared directly against
    /// `ReciprocalCellParams::length_a/b/c()`. When the reciprocal cell is
    /// a `ReciprocalCellVectors` that value follows the 2pi convention, so
    /// code-specific spacing units (for example CASTEP 1/Angstrom) must be
    /// converted before building this variant.
    Spacing { spacing: f64, shift: [f64; 3] },

    /// Explicit points with relative weights.
    Explicit { points: Vec<KPoint>, weights: Vec<f64> },
}

/// A set of k-points with relative weights.
#[derive(Debug, Clone, PartialEq)]
pub struct KPointSet {
    points: Vec<KPoint>,
    weights: Vec<f64>,
}

impl KPointSet {
    /// Build a set. Panics when the slice lengths differ.
    pub fn new(points: Vec<KPoint>, weights: Vec<f64>) -> Self {
        assert_eq!(
            points.len(),
            weights.len(),
            "KPointSet points and weights must have the same length"
        );
        Self { points, weights }
    }

    /// K-points of the set.
    pub fn points(&self) -> &[KPoint] {
        &self.points
    }

    /// Relative weights of the set.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Number of points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Copy with weights normalized so they sum to one.
    ///
    /// A total weight of zero or less leaves the weights unchanged.
    pub fn normalized(&self) -> Self {
        let total: f64 = self.weights.iter().sum();
        if total > 0.0 {
            let weights = self.weights.iter().map(|w| w / total).collect();
            Self::new(self.points.clone(), weights)
        } else {
            Self::new(self.points.clone(), self.weights.clone())
        }
    }
}

impl KPointSampling {
    /// Expand the specification into an explicit point set.
    ///
    /// `Mp` and `Spacing` produce uniform weights (1/N). `Explicit` passes
    /// the points and weights through unchanged. `reciprocal` is needed by
    /// the `Spacing` variant to size the grid.
    pub fn expand<R: ReciprocalCellParams>(&self, reciprocal: R) -> KPointSet {
        match self {
            KPointSampling::Mp { grid, shift } => {
                let mp = MPGrid::new(grid.map(|n| n as usize), *shift);
                uniform_set(mp.generate_reducible_kpts())
            }
            KPointSampling::Spacing { spacing, shift } => {
                let mp = MPSpacingParam::new(*spacing, *shift).build_mp_grid(reciprocal);
                uniform_set(mp.generate_reducible_kpts())
            }
            KPointSampling::Explicit { points, weights } => {
                KPointSet::new(points.clone(), weights.clone())
            }
        }
    }

    /// Expand, then reduce by a symmetry group.
    ///
    /// The resulting weights are the normalized degeneracies.
    pub fn reduce<R: ReciprocalCellParams, T: SymmetryOperation>(
        &self,
        reciprocal: R,
        group: &[T],
    ) -> KPointSet {
        let set = self.expand(reciprocal);
        let reduced: Vec<IrreducibleKpt> = reduce_kpoints(set.points(), group);
        let total: f64 = reduced.iter().map(|k| k.degeneracy() as f64).sum();
        let points = reduced.iter().map(|k| k.kpt()).collect();
        let weights = reduced
            .iter()
            .map(|k| {
                if total > 0.0 {
                    k.degeneracy() as f64 / total
                } else {
                    0.0
                }
            })
            .collect();
        KPointSet::new(points, weights)
    }
}

/// Uniform-weight set: weight 1/N per point.
fn uniform_set(kpts: Vec<KPoint>) -> KPointSet {
    let n = kpts.len().max(1) as f64;
    let weights = vec![1.0 / n; kpts.len()];
    KPointSet::new(kpts, weights)
}

#[cfg(test)]
mod tests {
    use nalgebra::Matrix3;

    use super::*;
    use crate::kpoints::KPoint;
    use chemrust_geometry::{LatticeVectors, ReciprocalCellVectors};

    struct Identity;
    impl SymmetryOperation for Identity {
        fn rotation(&self) -> Matrix3<f64> {
            Matrix3::identity()
        }
    }

    #[test]
    fn expand_mp_grid() {
        let sampling = KPointSampling::Mp { grid: [2, 2, 1], shift: [0.0; 3] };
        let set = sampling.expand(ReciprocalCellVectors::from(lattice()));
        assert_eq!(set.len(), 4);
        let sum: f64 = set.weights().iter().sum();
        assert!((sum - 1.0).abs() < 1e-12);
    }

    #[test]
    fn expand_spacing() {
        let sampling = KPointSampling::Spacing { spacing: 0.4, shift: [0.0; 3] };
        let set = sampling.expand(ReciprocalCellVectors::from(lattice()));
        // cubic 5.4 A cell: reciprocal length 2pi/5.4 ~ 1.1636
        // ceil(1.1636 / 0.4) = 3 per axis
        assert_eq!(set.len(), 27);
    }

    #[test]
    fn reduce_with_identity_group() {
        let sampling = KPointSampling::Mp { grid: [2, 2, 1], shift: [0.0; 3] };
        let group = [Identity];
        let set = sampling.reduce(ReciprocalCellVectors::from(lattice()), &group);
        // P-1 style: 4 points pair up under full inversion, 2 remain
        assert_eq!(set.len(), 2);
        let sum: f64 = set.weights().iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert_eq!(set.weights().to_vec(), [0.5, 0.5]);
    }

    #[test]
    fn explicit_passthrough() {
        let kpt = KPoint::new(nalgebra::Vector3::new(0.5, 0.5, 0.5));
        let sampling = KPointSampling::Explicit {
            points: vec![kpt],
            weights: vec![1.0],
        };
        let set = sampling.expand(ReciprocalCellVectors::from(lattice()));
        assert_eq!(set.len(), 1);
        assert_eq!(set.weights().to_vec(), [1.0]);
    }

    fn lattice() -> LatticeVectors {
        LatticeVectors::new(Matrix3::<f64>::new(5.4, 0.0, 0.0, 0.0, 5.4, 0.0, 0.0, 0.0, 5.4))
    }
}
