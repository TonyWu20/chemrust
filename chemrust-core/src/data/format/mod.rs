use std::fmt::Debug;

/// Trait bound for Model Formats
pub trait DataFormat: Debug + Clone + Default {}
