use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AtomBuilderIncomplete;

impl Display for AtomBuilderIncomplete {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The AtomBuilder has one or more fields to be completed!")
    }
}
