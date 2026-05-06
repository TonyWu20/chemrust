/// Error types for chemrust-geometry operations.
#[derive(Debug, Clone)]
pub enum Error {
    /// The structure has no cell (cell=None) but the operation requires one.
    NoCell,
    /// Invalid lattice parameters.
    InvalidLattice(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoCell => write!(f, "operation requires a periodic cell"),
            Error::InvalidLattice(msg) => write!(f, "invalid lattice: {msg}"),
        }
    }
}

impl std::error::Error for Error {}
