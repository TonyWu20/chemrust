use std::fmt::Display;

use crate::param_writer::castep_param::Task;

/// Parameters in `Geometry Optimization` only.
#[derive(Debug, Clone, Copy)]
pub struct GeomOptParam {
    geom_energy_tol: f64,
    geom_force_tol: f64,
    geom_stress_tol: f64,
    geom_disp_tol: f64,
    geom_max_iter: u32,
    geom_method: GeomMethod,
    fixed_npw: bool,
    popn_bond_cutoff: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum GeomMethod {
    BFGS,
    Delocalized,
    DampedMD,
}

impl Display for GeomMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeomMethod::BFGS => write!(f, "BFGS"),
            GeomMethod::Delocalized => write!(f, "Delocalized"),
            GeomMethod::DampedMD => write!(f, "DampedMD"),
        }
    }
}

impl Task for GeomOptParam {}

impl Default for GeomOptParam {
    fn default() -> Self {
        Self {
            geom_energy_tol: 5e-5,
            geom_force_tol: 0.1,
            geom_stress_tol: 0.2,
            geom_disp_tol: 0.005,
            geom_max_iter: 6000,
            geom_method: GeomMethod::BFGS,
            fixed_npw: false,
            popn_bond_cutoff: 3.0,
        }
    }
}

impl Display for GeomOptParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = format!(
            r#"geom_energy_tol :   {:22.15e}
geom_force_tol :        {:18.15}
geom_stress_tol :        {:18.15}
geom_disp_tol :        {:18.15}
geom_max_iter :     {}
geom_method : {}
fixed_npw : {}
popn_bond_cutoff :        {:18.15}"#,
            self.geom_energy_tol,
            self.geom_force_tol,
            self.geom_stress_tol,
            self.geom_disp_tol,
            self.geom_max_iter,
            self.geom_method,
            self.fixed_npw,
            self.popn_bond_cutoff
        );
        write!(f, "{}", content)
    }
}
