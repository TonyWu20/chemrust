use std::fmt::Display;

#[derive(Debug)]
pub struct InconsistentLength {
    pub curr: usize,
    pub expect: usize,
}

impl Display for InconsistentLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: InconsistentLength: Current size: {}; Expected size: {}.",
            self.curr, self.expect
        )
    }
}
