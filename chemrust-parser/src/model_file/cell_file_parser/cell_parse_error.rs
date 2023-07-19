use std::fmt::Display;

#[derive(Debug)]
pub struct SectionNotFound(String);

impl SectionNotFound {
    pub fn new(section_name: &str) -> Self {
        Self(section_name.into())
    }
}

impl Display for SectionNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Section name: {} not found in the cell file", self.0)
    }
}
