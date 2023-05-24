use std::fmt::Display;

use crate::structure_files::cell::cell_param::{settings::XCFunctional, Task};

/// Parameters in `Band Structure` task only.
pub struct BandStructureParam {
    bs_nextra_bands: u32,
    bs_xc_functional: XCFunctional,
    bs_eigenvalue_tol: f64,
    bs_write_eigenvalues: bool,
}

impl Task for BandStructureParam {}

impl Default for BandStructureParam {
    fn default() -> Self {
        Self {
            bs_nextra_bands: 72,
            bs_xc_functional: XCFunctional::default(),
            bs_eigenvalue_tol: 1e-5,
            bs_write_eigenvalues: true,
        }
    }
}

impl Display for BandStructureParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = format!(
            r#"bs_nextra_bands :       {}
bs_xc_functional : {}
bs_eigenvalue_tol :   {:22.15e}
bs_write_eigenvalues : {}"#,
            self.bs_nextra_bands,
            self.bs_xc_functional,
            self.bs_eigenvalue_tol,
            self.bs_write_eigenvalues
        );
        write!(f, "{}", content)
    }
}
