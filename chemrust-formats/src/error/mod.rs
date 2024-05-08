use std::fmt::Display;

#[derive(Debug)]
pub enum MatchError {
    NotAvailable,
}

impl Display for MatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MatchError {}
