use nalgebra::{Matrix3, Vector3};

/// A 3×3 linear transformation + translation, operating on fractional coordinates.
///
/// This is the decomposed form of a 4×4 augmented matrix:
///
/// ```text
/// [L  t]    x' = L * x + t
/// [0  1]
/// ```
///
/// where L is the 3×3 linear part and t is the translation column.
/// For crystal-geometry transforms (SurfaceRotation, Supercell), the
/// translation is typically zero — the struct exists for generality
/// and to enable unified lazy composition.
///
/// Why not nalgebra's `Affine3`? These transforms operate in
/// **fractional-coordinate space**, not Cartesian space. The 3×3
/// linear part can include non-uniform scaling (Supercell, VacuumGap)
/// or arbitrary basis changes (SurfaceRotation P⁻¹), which excludes
/// `Isometry3` and `Similarity3`. While `Affine3` has the same
/// capabilities, a custom struct keeps field access direct and avoids
/// version-dependent nalgebra API coupling.
#[derive(Debug, Clone, Copy)]
pub struct TransformMatrix {
    pub linear: Matrix3<f64>,
    pub translation: Vector3<f64>,
}

impl TransformMatrix {
    /// Construct from a linear part only, with zero translation.
    pub fn from_linear(linear: Matrix3<f64>) -> Self {
        Self { linear, translation: Vector3::zeros() }
    }

    /// Compose `self` after `other`: `result = self ∘ other`.
    ///
    /// x → self.linear * (other.linear * x + other.translation) + self.translation
    ///   = (self.linear * other.linear) * x + (self.linear * other.translation + self.translation)
    pub fn compose(self, other: Self) -> Self {
        Self {
            linear: self.linear * other.linear,
            translation: self.linear * other.translation + self.translation,
        }
    }
}

/// A geometric operation expressed as a 3×3 linear transform + translation,
/// operating on fractional coordinates: x' = L * x + t.
///
/// The cell update is derived from the linear part: C' = C * L⁻¹
/// (coordinates transform inversely to the basis).
pub trait Transform {
    fn matrix(&self) -> TransformMatrix;
}

/// Rotate the conventional cell so the (hkl) plane normal aligns with c.
///
/// The resulting a and b vectors span the (hkl) plane; c points along [hkl].
/// For cubic crystals, the (111) rotation matrix is:
///   a_surf = [ 1, -1,  0] / 2  (in conventional frac)
///   b_surf = [ 0,  1, -1] / 2
///   c_surf = [ 1,  1,  1]
pub struct SurfaceRotation {
    pub h: i32,
    pub k: i32,
    pub l: i32,
}

impl SurfaceRotation {
    pub fn new(h: i32, k: i32, l: i32) -> Self {
        Self { h, k, l }
    }

    /// Build P⁻¹ for cubic (hkl) surfaces.
    ///
    /// P = [a_surf | b_surf | c_surf] in conventional fractional coordinates.
    ///   a_surf = (-k, h, 0)                    (shortest in-plane vector)
    ///   b_surf = (h, k, l) × (-k, h, 0)       (second in-plane vector)
    ///   c_surf = (h, k, l)                     (surface normal)
    ///
    /// M = P⁻¹ maps conventional fractional to surface fractional: x_surf = M * x_conv.
    /// Cell updates as C_surf = C_conv * M⁻¹ = C_conv * P.
    fn cubic_surface_matrix(h: i32, k: i32, l: i32) -> Matrix3<f64> {
        let (h, k, l) = (h as f64, k as f64, l as f64);
        let a = Vector3::new(-k, h, 0.0);
        let c = Vector3::new(h, k, l);

        // Handle degenerate case where (h,k,l) is along z
        if a.norm() < 1e-10 {
            return Matrix3::identity();
        }

        let b = c.cross(&a);  // second in-plane surface vector
        // P = [a | b | c] as columns
        let p = Matrix3::from_columns(&[a, b, c]);
        // Return P⁻¹
        p.try_inverse().expect("Surface rotation matrix is singular")
    }
}

impl Transform for SurfaceRotation {
    fn matrix(&self) -> TransformMatrix {
        TransformMatrix::from_linear(Self::cubic_surface_matrix(self.h, self.k, self.l))
    }
}

/// Diagonal supercell scaling.
///
/// Transforms: x' = diag(1/nx, 1/ny, 1/nz) * x
/// Cell: C' = C * diag(nx, ny, nz)
pub struct Supercell {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
}

impl Supercell {
    pub fn new(nx: usize, ny: usize, nz: usize) -> Self {
        Self { nx, ny, nz }
    }
}

impl Transform for Supercell {
    fn matrix(&self) -> TransformMatrix {
        TransformMatrix::from_linear(Matrix3::new(
            1.0 / self.nx as f64, 0.0, 0.0,
            0.0, 1.0 / self.ny as f64, 0.0,
            0.0, 0.0, 1.0 / self.nz as f64,
        ))
    }
}

// VacuumGap is not a Transform — it's a method on Structure because it
// depends on the current cell's c-length, which can't be known at
// composition time.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supercell_matrix_2x2x1() {
        let sc = Supercell::new(2, 2, 1);
        let tm = sc.matrix();
        // x, y scaled by 1/2, z unchanged
        assert!((tm.linear[(0, 0)] - 0.5).abs() < 1e-10);
        assert!((tm.linear[(1, 1)] - 0.5).abs() < 1e-10);
        assert!((tm.linear[(2, 2)] - 1.0).abs() < 1e-10);
        // no translation
        assert_eq!(tm.translation, Vector3::zeros());
    }

    #[test]
    fn surface_rotation_111_is_invertible() {
        let sr = SurfaceRotation::new(1, 1, 1);
        let tm = sr.matrix();
        let det = tm.linear.determinant();
        assert!(det.abs() > 1e-10, "Surface rotation matrix is singular");
    }

    #[test]
    fn surface_rotation_111_layers() {
        // FCC conventional cell atoms after (111) rotation should give
        // z-coordinates that correspond to A and B layers.

        let sr = SurfaceRotation::new(1, 1, 1);
        let tm = sr.matrix();

        // FCC conventional cell in conventional frac
        let fcc_atoms = [
            [0.0, 0.0, 0.0],
            [0.0, 0.5, 0.5],
            [0.5, 0.0, 0.5],
            [0.5, 0.5, 0.0],
        ];

        // Transform and check z-coordinates
        for (i, atom) in fcc_atoms.iter().enumerate() {
            let pt = nalgebra::Point3::new(atom[0], atom[1], atom[2]);
            let new = tm.linear * pt;
            if i == 0 {
                // Corner atom at z = 0
                assert!((new.z).abs() < 1e-10, "corner atom z should be 0, got {}", new.z);
            } else {
                // Face-centered atoms should be at z = 1/3
                assert!((new.z - 1.0/3.0).abs() < 1e-10,
                    "face atom {} z should be 1/3, got {}", i, new.z);
            }
        }
    }
}
