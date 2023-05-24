//! Options of `density_mixing`

use std::{fmt::Display, marker::PhantomData};

use super::SchemeOptions;

/// Avaliable options of density mixing schemes
#[derive(Debug, Clone, Copy)]
pub enum DensityMixing {
    Linear {
        mix_charge_amp: f64,
        mix_spin_amp: f64,
    },
    Kerker {
        mix_charge_amp: f64,
        mix_spin_amp: f64,
        mix_charge_gmax: f64,
        mix_spin_gmax: f64,
    },
    Pulay {
        mix_charge_amp: f64,
        mix_spin_amp: f64,
        mix_charge_gmax: f64,
        mix_spin_gmax: f64,
        mix_history_length: u32,
    },
    Broyden {
        mix_charge_amp: f64,
        mix_spin_amp: f64,
        mix_charge_gmax: f64,
        mix_spin_gmax: f64,
        mix_history_length: u32,
    },
}

impl Display for DensityMixing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Linear {
                mix_charge_amp,
                mix_spin_amp,
            } => format!(
                "mixing_scheme : Linear\nmix_charge_amp : {}\nmix_spin_amp : {}",
                mix_charge_amp, mix_spin_amp
            ),
            Self::Kerker {
                mix_charge_amp,
                mix_spin_amp,
                mix_charge_gmax,
                mix_spin_gmax,
            } => format!(
                r#"mixing_scheme : Kerker
mix_charge_amp : {}
mix_spin_amp : {}
mix_charge_gmax : {}
mix_spin_gmax : {}"#,
                mix_charge_amp, mix_spin_amp, mix_charge_gmax, mix_spin_gmax
            ),
            Self::Pulay {
                mix_charge_amp,
                mix_spin_amp,
                mix_charge_gmax,
                mix_spin_gmax,
                mix_history_length,
            } => format!(
                r#"mixing_scheme : Pulay
mix_charge_amp : {}
mix_spin_amp : {}
mix_charge_gmax : {}
mix_spin_gmax : {}
mix_history_length : {}"#,
                mix_charge_amp, mix_spin_amp, mix_charge_gmax, mix_spin_gmax, mix_history_length
            ),
            Self::Broyden {
                mix_charge_amp,
                mix_spin_amp,
                mix_charge_gmax,
                mix_spin_gmax,
                mix_history_length,
            } => format!(
                r#"mixing_scheme : Broyden
mix_charge_amp : {}
mix_spin_amp : {}
mix_charge_gmax : {}
mix_spin_gmax : {}
mix_history_length : {}"#,
                mix_charge_amp, mix_spin_amp, mix_charge_gmax, mix_spin_gmax, mix_history_length
            ),
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct DensityMixingBuilder<T: SchemeOptions> {
    mix_charge_amp: f64,
    mix_spin_amp: f64,
    mix_charge_gmax: Option<f64>,
    mix_spin_gmax: Option<f64>,
    mix_history_length: Option<u32>,
    mix_scheme: PhantomData<T>,
}

#[derive(Debug)]
pub struct Linear;
#[derive(Debug)]
pub struct Kerker;
#[derive(Debug)]
pub struct Pulay;
#[derive(Debug)]
pub struct Broyden;

pub trait NonLinearMixing {
    fn with_charge_gmax(self, mix_charge_gmax: f64) -> Self;
    fn with_spin_gmax(self, mix_spin_gmax: f64) -> Self;
}

pub trait WithHistory: NonLinearMixing {
    fn with_history_length(self, mix_history_length: u32) -> Self;
}

impl SchemeOptions for Linear {}
impl SchemeOptions for Kerker {}
impl SchemeOptions for Pulay {}
impl SchemeOptions for Broyden {}

impl<T: SchemeOptions> DensityMixingBuilder<T> {
    pub fn new() -> Self {
        Self {
            mix_charge_amp: 0.5,
            mix_spin_amp: 2.0,
            mix_charge_gmax: Some(1.5),
            mix_spin_gmax: Some(1.5),
            mix_history_length: Some(20),
            mix_scheme: PhantomData,
        }
    }
    pub fn with_charge_amp(self, mix_charge_amp: f64) -> Self {
        Self {
            mix_charge_amp,
            ..self
        }
    }
    pub fn with_spin_amp(self, mix_spin_amp: f64) -> Self {
        Self {
            mix_spin_amp,
            ..self
        }
    }
}

impl DensityMixingBuilder<Linear> {
    pub fn build(self) -> DensityMixing {
        DensityMixing::Linear {
            mix_charge_amp: self.mix_charge_amp,
            mix_spin_amp: self.mix_spin_amp,
        }
    }
}

impl NonLinearMixing for DensityMixingBuilder<Kerker> {
    fn with_charge_gmax(self, mix_charge_gmax: f64) -> Self {
        Self {
            mix_charge_gmax: Some(mix_charge_gmax),
            ..self
        }
    }

    fn with_spin_gmax(self, mix_spin_gmax: f64) -> Self {
        Self {
            mix_spin_gmax: Some(mix_spin_gmax),
            ..self
        }
    }
}

impl DensityMixingBuilder<Kerker> {
    pub fn build(self) -> DensityMixing {
        DensityMixing::Kerker {
            mix_charge_amp: self.mix_charge_amp,
            mix_spin_amp: self.mix_spin_amp,
            mix_charge_gmax: self.mix_charge_gmax.unwrap(),
            mix_spin_gmax: self.mix_spin_gmax.unwrap(),
        }
    }
}

impl NonLinearMixing for DensityMixingBuilder<Pulay> {
    fn with_charge_gmax(self, mix_charge_gmax: f64) -> Self {
        Self {
            mix_charge_gmax: Some(mix_charge_gmax),
            ..self
        }
    }

    fn with_spin_gmax(self, mix_spin_gmax: f64) -> Self {
        Self {
            mix_spin_gmax: Some(mix_spin_gmax),
            ..self
        }
    }
}

impl DensityMixingBuilder<Pulay> {
    pub fn build(self) -> DensityMixing {
        DensityMixing::Pulay {
            mix_charge_amp: self.mix_charge_amp,
            mix_spin_amp: self.mix_spin_amp,
            mix_charge_gmax: self.mix_charge_gmax.unwrap(),
            mix_spin_gmax: self.mix_spin_gmax.unwrap(),
            mix_history_length: self.mix_history_length.unwrap(),
        }
    }
}

impl NonLinearMixing for DensityMixingBuilder<Broyden> {
    fn with_charge_gmax(self, mix_charge_gmax: f64) -> Self {
        Self {
            mix_charge_gmax: Some(mix_charge_gmax),
            ..self
        }
    }

    fn with_spin_gmax(self, mix_spin_gmax: f64) -> Self {
        Self {
            mix_spin_gmax: Some(mix_spin_gmax),
            ..self
        }
    }
}

impl WithHistory for DensityMixingBuilder<Pulay> {
    fn with_history_length(self, mix_history_length: u32) -> Self {
        Self {
            mix_history_length: Some(mix_history_length),
            ..self
        }
    }
}

impl WithHistory for DensityMixingBuilder<Broyden> {
    fn with_history_length(self, mix_history_length: u32) -> Self {
        Self {
            mix_history_length: Some(mix_history_length),
            ..self
        }
    }
}

impl DensityMixingBuilder<Broyden> {
    pub fn build(self) -> DensityMixing {
        DensityMixing::Broyden {
            mix_charge_amp: self.mix_charge_amp,
            mix_spin_amp: self.mix_spin_amp,
            mix_charge_gmax: self.mix_charge_gmax.unwrap(),
            mix_spin_gmax: self.mix_spin_gmax.unwrap(),
            mix_history_length: self.mix_history_length.unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DensityMixingBuilder, Pulay};

    #[test]
    fn test_dm() {
        let dm_settings = DensityMixingBuilder::<Pulay>::new().build();
        println!("{}", dm_settings);
    }
}
