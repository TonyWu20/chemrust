use nalgebra::{Matrix3, Matrix4, Vector3};

/// A geometric operation expressed as a 4×4 augmented matrix
/// operating on fractional coordinates: x' = M * x.
///
/// The matrix has the structure: [R  t] where R is 3×3 (linear part)
///                              [0  1]  and t is the translation column.
///
/// This operates on the fractional coordinates directly, NOT on the
/// Cartesian cell. The cell update is derived from the linear part:
/// C' = C * (R_3x3)⁻¹ (since coords transform inversely to basis).
pub trait Transform {
    fn matrix(&self) -> Matrix4<f64>;
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
    fn matrix(&self) -> Matrix4<f64> {
        let m33 = Self::cubic_surface_matrix(self.h, self.k, self.l);
        Matrix4::new(
            m33[(0, 0)], m33[(0, 1)], m33[(0, 2)], 0.0,
            m33[(1, 0)], m33[(1, 1)], m33[(1, 2)], 0.0,
            m33[(2, 0)], m33[(2, 1)], m33[(2, 2)], 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
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
    fn matrix(&self) -> Matrix4<f64> {
        Matrix4::new(
            1.0 / self.nx as f64, 0.0, 0.0, 0.0,
            0.0, 1.0 / self.ny as f64, 0.0, 0.0,
            0.0, 0.0, 1.0 / self.nz as f64, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
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
        let m = sc.matrix();
        // x, y scaled by 1/2, z unchanged
        assert!((m[(0, 0)] - 0.5).abs() < 1e-10);
        assert!((m[(1, 1)] - 0.5).abs() < 1e-10);
        assert!((m[(2, 2)] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn surface_rotation_111_is_invertible() {
        let sr = SurfaceRotation::new(1, 1, 1);
        let m = sr.matrix();
        let det_3x3 = m.fixed_view::<3, 3>(0, 0).determinant();
        assert!(det_3x3.abs() > 1e-10, "Surface rotation matrix is singular");
    }

    #[test]
    fn surface_rotation_111_layers() {
        // FCC conventional cell atoms after (111) rotation should give
        // z-coordinates that correspond to A and B layers.
        use nalgebra::Vector4;

        let sr = SurfaceRotation::new(1, 1, 1);
        let m = sr.matrix();

        // FCC conventional cell in conventional frac
        let fcc_atoms = [
            [0.0, 0.0, 0.0],
            [0.0, 0.5, 0.5],
            [0.5, 0.0, 0.5],
            [0.5, 0.5, 0.0],
        ];

        // Transform and check z-coordinates
        for (i, atom) in fcc_atoms.iter().enumerate() {
            let v = Vector4::new(atom[0], atom[1], atom[2], 1.0);
            let v_new = m * v;
            if i == 0 {
                // Corner atom at z = 0
                assert!((v_new.z).abs() < 1e-10, "corner atom z should be 0, got {}", v_new.z);
            } else {
                // Face-centered atoms should be at z = 1/3
                assert!((v_new.z - 1.0/3.0).abs() < 1e-10,
                    "face atom {} z should be 1/3, got {}", i, v_new.z);
            }
        }
    }
}
