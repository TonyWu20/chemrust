use std::fmt::Display;

mod electronic_minimizers;
mod exchange_functional;

pub use electronic_minimizers::*;
pub use exchange_functional::XCFunctional;

#[derive(Debug)]
pub enum FiniteBasisCorr {
    No,
    Manual,
    Auto,
}

impl Display for FiniteBasisCorr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => write!(f, "0"),
            Self::Manual => write!(f, "1"),
            Self::Auto => write!(f, "2"),
        }
    }
}

#[derive(Debug)]
pub enum OptStrategy {
    Default,
    Speed,
    Memory,
}

impl Display for OptStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Speed => write!(f, "Speed"),
            Self::Memory => write!(f, "Memory"),
        }
    }
}
