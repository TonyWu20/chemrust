use nalgebra::Matrix3;
use std::f64::consts::{PI, FRAC_PI_2};

/// Lattice constants (a, b, c in Angstrom, angles in radians).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellConstants {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl CellConstants {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        Self { a, b, c, alpha, beta, gamma }
    }
}

/// Lattice vectors. Columns of the 3×3 matrix are the a, b, c vectors
/// in Cartestian coordinates (Angstrom).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatticeVectors(Matrix3<f64>);

impl LatticeVectors {
    pub fn new(tensor: Matrix3<f64>) -> Self {
        Self(tensor)
    }

    /// Build from cell constants using the standard convention:
    /// a along x, b in xy-plane, c with components from angles.
    pub fn from_constants(c: CellConstants) -> Self {
        let CellConstants { a, b, c, alpha, beta, gamma } = c;
        let cos_a = alpha.cos();
        let cos_b = beta.cos();
        let cos_y = gamma.cos();
        let sin_y = gamma.sin();
        let volume = a * b * c * (1.0
            - cos_a.powi(2) - cos_b.powi(2) - cos_y.powi(2)
            + 2.0 * cos_a * cos_b * cos_y)
            .sqrt();
        // Column-major: each column is a lattice vector
        Self(Matrix3::new(
            a,                b * cos_y,              c * cos_b,
            0.0,              b * sin_y,              c * (cos_a - cos_b * cos_y) / sin_y,
            0.0,              0.0,                    volume / (a * b * sin_y),
        ))
    }

    pub fn tensor(&self) -> &Matrix3<f64> {
        &self.0
    }

    pub fn cell_volume(&self) -> f64 {
        self.0.determinant()
    }

    /// Metric tensor G = C^T * C
    pub fn metric_tensor(&self) -> Matrix3<f64> {
        self.0.transpose() * self.0
    }

    /// Reciprocal lattice vectors: 2π * (C⁻¹)^T
    pub fn reciprocal(&self) -> Matrix3<f64> {
        2.0 * PI * self.0.try_inverse().unwrap().transpose()
    }

    pub fn lengths(&self) -> (f64, f64, f64) {
        (
            self.0.column(0).norm(),
            self.0.column(1).norm(),
            self.0.column(2).norm(),
        )
    }

    pub fn angles(&self) -> (f64, f64, f64) {
        let col0 = self.0.column(0);
        let col1 = self.0.column(1);
        let col2 = self.0.column(2);
        (
            col1.angle(&col2),
            col0.angle(&col2),
            col0.angle(&col1),
        )
    }

    /// Reference: `crystallographic_group::database::CrystalSystem` usage.
    pub fn crystal_system(&self) -> crystallographic_group::database::CrystalSystem {
        let (a, b, c) = self.lengths();
        let (alpha, beta, gamma) = self.angles();
        let eq = |x: f64, y: f64| (x - y).abs() < 1e-6;
        let n_90 = [alpha, beta, gamma].iter().filter(|&&v| eq(v, FRAC_PI_2)).count();
        let n_120 = [alpha, beta, gamma].iter().filter(|&&v| eq(v, 2.0 * PI / 3.0)).count();
        if eq(a, b) && eq(b, c) {
            if n_90 == 3 { return crystallographic_group::database::CrystalSystem::Cubic; }
            if (n_90 == 0 && eq(alpha, beta) && eq(beta, gamma)) || n_120 == 0 {
                return crystallographic_group::database::CrystalSystem::Trigonal;
            }
        }
        if eq(a, b) && n_90 == 3 { return crystallographic_group::database::CrystalSystem::Tetragonal; }
        if eq(a, b) && n_90 == 2 && n_120 == 1 { return crystallographic_group::database::CrystalSystem::Hexagonal; }
        if n_90 == 3 { return crystallographic_group::database::CrystalSystem::Orthorhombic; }
        if n_90 == 2 { return crystallographic_group::database::CrystalSystem::Monoclinic; }
        crystallographic_group::database::CrystalSystem::Triclinic
    }
}

impl From<CellConstants> for LatticeVectors {
    fn from(c: CellConstants) -> Self {
        Self::from_constants(c)
    }
}

impl From<LatticeVectors> for CellConstants {
    fn from(lv: LatticeVectors) -> Self {
        let (a, b, c) = lv.lengths();
        let (alpha, beta, gamma) = lv.angles();
        Self { a, b, c, alpha, beta, gamma }
    }
}

impl From<Matrix3<f64>> for LatticeVectors {
    fn from(m: Matrix3<f64>) -> Self {
        Self(m)
    }
}

impl From<[[f64; 3]; 3]> for LatticeVectors {
    fn from(rows: [[f64; 3]; 3]) -> Self {
        Self(Matrix3::from(rows))
    }
}

// ── Reciprocal lattice ───────────────────────────────────────────

/// Common trait for reciprocal cell parameter access.
/// Used by downstream crates (chemrust-kpoint-gen) for MP grid generation.
pub trait ReciprocalCellParams {
    fn length_a(&self) -> f64;
    fn length_b(&self) -> f64;
    fn length_c(&self) -> f64;
    fn lattice_bases(&self) -> Matrix3<f64>;
}

/// Reciprocal lattice vectors. Columns are a*, b*, c*.
#[derive(Debug, Clone, Copy)]
pub struct ReciprocalCellVectors(pub Matrix3<f64>);

impl ReciprocalCellParams for ReciprocalCellVectors {
    fn length_a(&self) -> f64 {
        self.0.column(0).norm()
    }
    fn length_b(&self) -> f64 {
        self.0.column(1).norm()
    }
    fn length_c(&self) -> f64 {
        self.0.column(2).norm()
    }
    fn lattice_bases(&self) -> Matrix3<f64> {
        self.0
    }
}

impl From<LatticeVectors> for ReciprocalCellVectors {
    fn from(lv: LatticeVectors) -> Self {
        Self(lv.reciprocal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubic_cell() {
        let lv = LatticeVectors::new(Matrix3::identity() * 3.615);
        assert!((lv.cell_volume() - 3.615_f64.powi(3)).abs() < 1e-10);
        assert_eq!(lv.lengths(), (3.615, 3.615, 3.615));
    }

    #[test]
    fn reciprocal() {
        let lv = LatticeVectors::new(Matrix3::identity() * 3.615);
        let rv = ReciprocalCellVectors::from(lv);
        // a* = 2π / a
        let a_star = 2.0 * PI / 3.615;
        assert!((rv.length_a() - a_star).abs() < 1e-10);
    }
}
