use std::str::FromStr;

use serde::Deserialize;

use crate::error::MatchError;

#[derive(Debug, Default, Deserialize)]
pub enum LengthUnit {
    Bohr,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

impl FromStr for LengthUnit {
    type Err = MatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bohr" => Ok(Self::Bohr),
            "a0" => Ok(Self::Bohr),
            "m" => Ok(Self::Meter),
            "cm" => Ok(Self::Centimeter),
            "nm" => Ok(Self::Nanometer),
            "ang" => Ok(Self::Ang),
            _ => Err(MatchError::NotAvailable),
        }
    }
}
