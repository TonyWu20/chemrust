use std::fmt::Display;

use super::kpoint::KPoint;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IrreducibleKpt {
    kpt: KPoint,
    degeneracy: usize,
}

impl IrreducibleKpt {
    pub fn new(kpt: KPoint, degeneracy: usize) -> Self {
        Self { kpt, degeneracy }
    }

    pub fn kpt(&self) -> KPoint {
        self.kpt
    }

    pub fn degeneracy(&self) -> usize {
        self.degeneracy
    }
}

impl Display for IrreducibleKpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kpt(), self.degeneracy())
    }
}
