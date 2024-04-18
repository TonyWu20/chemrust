use nalgebra::Matrix3;
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub(crate) a: f32,
    pub(crate) b: f32,
    pub(crate) c: f32,
    pub(crate) alpha: f32,
    pub(crate) beta: f32,
    pub(crate) gamma: f32,
}

impl Cell {
    pub fn new(a: f32, b: f32, c: f32, alpha: f32, beta: f32, gamma: f32) -> Self {
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
    pub fn cell_volume(&self) -> f32 {
        let Cell {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let cos_a = alpha.cos();
        let cos_b = beta.cos();
        let cos_y = gamma.cos();
        a * b
            * c
            * (1.0 - cos_a * cos_a - cos_b * cos_b - cos_y * cos_y + 2.0 * cos_a * cos_b * cos_y)
                .sqrt()
    }
    /// Matrix (3x3) representation of the cell lattice parameters (lattice vectors)
    /// ```
    ///     [a bcosy                    ccosB]
    /// A = [0 bsiny c (cosa - cosbcosy)/siny]
    ///   = [0     0               v/(absiny)]
    /// ```
    pub fn matrix_repr(&self) -> Matrix3<f32> {
        let Cell {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let volume = self.cell_volume();
        Matrix3::new(
            *a,
            b * gamma.cos(),
            c * beta.cos(),
            0.0,
            b * gamma.sin(),
            c * (alpha.cos() - beta.cos() * gamma.cos()) / gamma.sin(),
            0.0,
            0.0,
            volume / (a * b * gamma.sin()),
        )
    }
}
