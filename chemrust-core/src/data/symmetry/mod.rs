use std::cmp::Ordering;

use crystallographic_group::database::CrystalSystem;

use super::lattice::CrystalModel;
use super::lattice::UnitCellParameters;

pub trait SymmetryInfo: CrystalModel {
    /// 1-230
    fn get_space_group_it_num(&self) -> u8;
    fn get_crystal_system(&self) -> CrystalSystem {
        let lattice_param = self.get_cell_parameters();
        let (length_a, length_b, length_c) = (
            lattice_param.length_a(),
            lattice_param.length_b(),
            lattice_param.length_c(),
        );
        let (alpha, beta, gamma) = (
            lattice_param.angle_alpha(),
            lattice_param.angle_beta(),
            lattice_param.angle_gamma(),
        );
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

fn compare_f64(v1: f64, v2: f64) -> Ordering {
    if (v1 - v2).abs() < 1e-6 {
        Ordering::Equal
    } else if v1 - v2 < -1e-6 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
