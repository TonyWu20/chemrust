use std::{cmp::Ordering, fmt::Display};

use crystallographic_group::database::CrystalSystem;
use nalgebra::Matrix3;
#[derive(Debug, Clone, Copy)]
/// Lattice constants.
/// Consider using newtype to wrap the angles f64 to ensure in radian form.
pub struct CellConstants {
    pub(crate) a: f64,
    pub(crate) b: f64,
    pub(crate) c: f64,
    pub(crate) alpha: f64,
    pub(crate) beta: f64,
    pub(crate) gamma: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct LatticeVectors {
    pub(crate) tensor: Matrix3<f64>,
}

pub trait UnitCellParameters {
    fn cell_volume(&self) -> f64 {
        self.lattice_bases().determinant()
    }
    fn lattice_bases(&self) -> Matrix3<f64>;
    fn metric_tensor(&self) -> Matrix3<f64> {
        let mat = self.lattice_bases();
        let mat_transpose = mat.transpose();
        mat_transpose * mat
    }
    fn length_a(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).a
    }
    fn length_b(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).b
    }
    fn length_c(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).c
    }
    /// Should return radians!
    fn angle_alpha(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).alpha
    }
    /// Should return radians!
    fn angle_beta(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).beta
    }

    /// Should return radians!
    fn angle_gamma(&self) -> f64 {
        CellConstants::from(self.lattice_bases()).gamma
    }
    fn get_crystal_system(&self) -> CrystalSystem {
        let (length_a, length_b, length_c) = (self.length_a(), self.length_b(), self.length_c());
        let (alpha, beta, gamma) = (self.angle_alpha(), self.angle_beta(), self.angle_gamma());
        let axis_length_equal_count = [
            compare_f64(length_a, length_b),
            compare_f64(length_b, length_c),
            compare_f64(length_a, length_c),
        ]
        .iter()
        .filter(|ord| matches!(ord, Ordering::Equal))
        .count();

        let angle_eq_90_count = [
            compare_f64(90.0, alpha),
            compare_f64(90.0, beta),
            compare_f64(90.0, gamma),
        ]
        .iter()
        .filter(|ord| matches!(ord, Ordering::Equal))
        .count();

        let angle_eq_120_count = [
            compare_f64(120.0, alpha),
            compare_f64(120.0, beta),
            compare_f64(120.0, gamma),
        ]
        .iter()
        .filter(|ord| matches!(ord, Ordering::Equal))
        .count();
        match axis_length_equal_count {
            3 => {
                if angle_eq_90_count == 3 {
                    CrystalSystem::Cubic
                } else {
                    CrystalSystem::Trigonal // Rhombohedral belongs to Trigonal
                }
            }
            1 => {
                if angle_eq_90_count == 3 {
                    CrystalSystem::Tetragonal
                } else if angle_eq_90_count == 2 && angle_eq_120_count == 1 {
                    CrystalSystem::Hexagonal
                } else {
                    CrystalSystem::Triclinic
                }
            }
            2 => {
                // Floating point accuracy issue : a = b && b = c in tol but a != c
                CrystalSystem::Triclinic
            }
            _ => match angle_eq_90_count {
                3 => CrystalSystem::Orthorhombic,
                2 => CrystalSystem::Monoclinic,
                _ => CrystalSystem::Triclinic,
            },
        }
    }
}

impl CellConstants {
    /// angles should be specified in degrees
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

impl UnitCellParameters for CellConstants {
    fn cell_volume(&self) -> f64 {
        let CellConstants {
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

    fn lattice_bases(&self) -> Matrix3<f64> {
        let CellConstants {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        } = self;
        let volume = self.cell_volume();
        let cos_a = alpha.cos();
        let cos_b = beta.cos();
        let cos_y = gamma.cos();
        let sin_y = gamma.sin();
        //     [a         bcosy                     ccosB]
        // A = [0         bsiny   c(cosa - cosbcosy)/siny]
        //     [0             0                v/(absiny)]
        // The columns are `a`, `b` and `c` vectors;
        Matrix3::new(
            *a,
            b * cos_y,
            c * cos_b,
            0.0,
            b * sin_y,
            c * (cos_a - cos_b * cos_y) / sin_y,
            0.0,
            0.0,
            volume / (a * b * sin_y),
        )
    }

    fn length_a(&self) -> f64 {
        self.a
    }

    fn length_b(&self) -> f64 {
        self.b
    }

    fn length_c(&self) -> f64 {
        self.c
    }

    fn angle_alpha(&self) -> f64 {
        self.alpha
    }

    fn angle_beta(&self) -> f64 {
        self.beta
    }

    fn angle_gamma(&self) -> f64 {
        self.gamma
    }
}

impl From<Matrix3<f64>> for CellConstants {
    fn from(mat: Matrix3<f64>) -> Self {
        let (v_a, v_b, v_c) = (mat.column(0), mat.column(1), mat.column(2));
        let (a, b, c) = (v_a.norm(), v_b.norm(), v_c.norm());
        let (alpha, beta, gamma) = (v_b.angle(&v_c), v_a.angle(&v_c), v_a.angle(&v_b));
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
}

impl From<&Matrix3<f64>> for CellConstants {
    fn from(value: &Matrix3<f64>) -> Self {
        Self::from(*value)
    }
}

impl From<[[f64; 3]; 3]> for CellConstants {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Self::from(Matrix3::from(value))
    }
}

impl Display for CellConstants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "a_length: {:>20.18}; b_length: {:>20.18}; c_length: {:>20.18}; alpha: {} beta: {} gamma: {}",
            self.a, self.b, self.c, self.alpha, self.beta, self.gamma
        )
    }
}

impl LatticeVectors {
    pub fn new(tensor: Matrix3<f64>) -> Self {
        Self { tensor }
    }

    pub fn tensor(&self) -> Matrix3<f64> {
        self.tensor
    }
}

impl UnitCellParameters for LatticeVectors {
    fn cell_volume(&self) -> f64 {
        self.tensor().determinant()
    }

    fn lattice_bases(&self) -> Matrix3<f64> {
        self.tensor()
    }
}

impl From<CellConstants> for LatticeVectors {
    fn from(constants: CellConstants) -> Self {
        Self::new(constants.lattice_bases())
    }
}

impl From<[[f64; 3]; 3]> for LatticeVectors {
    fn from(value: [[f64; 3]; 3]) -> Self {
        Self::new(Matrix3::from(value))
    }
}

fn compare_f64(v1: f64, v2: f64) -> Ordering {
    if (v1 - v2).abs() < 1e-6 {
        Ordering::Equal
    } else if v1 - v2 < -1e-6 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
mod test {
    use nalgebra::{Point3, Rotation3, Vector3};

    use crate::{
        data::lattice::cell_param::unit_cell::UnitCellParameters,
        systems::crystal_model::rotated_lattice_tensor,
    };

    use super::CellConstants;

    #[test]
    fn cell_repr() {
        let lattice_cart = [
            // a
            [
                18.931530020488704480,
                -0.000000000000003553,
                0.000000000000000000,
            ],
            // b
            [
                -9.465765010246645517,
                16.395185930251127360,
                0.000000000000000000,
            ],
            // c
            [
                0.000000000000000000,
                0.000000000000000000,
                9.999213039981000861,
            ],
        ];
        let cell = CellConstants::from(lattice_cart);
        println!("{}", cell);
        println!("{:#>20.18}", cell.lattice_bases());
        let p = Point3::new(0.07560343470042601, 0.0756034355668187, 0.5000000004346841);
        let o: Point3<f64> = Point3::origin();
        let b = cell.lattice_bases().column(1).xyz();
        let j = Vector3::y_axis();
        let rot = Rotation3::rotation_between(&b, &j).unwrap();
        let mat = rotated_lattice_tensor(&cell, rot);
        let cart_p = mat * p;
        println!("{:#.5}", mat);
        println!("{:?}", j);
        println!("{:#}", cart_p);
        let frac_p =
            cell.lattice_bases().try_inverse().unwrap() * rot.matrix() * cell.lattice_bases() * p;
        println!("{:#}", frac_p);
        println!("{:#}", cell.lattice_bases() * frac_p);
        let po_cart = (cart_p - o).norm_squared();
        let metric_tensor = cell.metric_tensor();
        let po = frac_p - o;
        let po_norm_squared = po.transpose() * metric_tensor * po;
        println!("{:#}", metric_tensor);
        println!(
            "V^2: {}, det(G) : {}",
            cell.cell_volume().powi(2),
            metric_tensor.determinant()
        );
        println!(
            "cart_length: {}, frac_length by metric tensor: {}",
            po_cart, po_norm_squared.x
        );
    }
}
