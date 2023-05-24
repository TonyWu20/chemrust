use std::fmt::Display;

use self::density_mixing::DensityMixing;

mod density_mixing;

#[derive(Debug)]
pub(crate) enum MetalsMethod {
    Dm(DensityMixing),
    Edft,
}

impl Display for MetalsMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dm(dm) => write!(f, "metals_method : dm\n{}", dm),
            Self::Edft => write!(f, "metals_method : edft\nnum_occ_cycles : 6"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::structure_files::cell::cell_param::settings::electronic_minimizers::density_mixing::DensityMixingBuilder;

    use super::{density_mixing::Pulay, MetalsMethod};

    #[test]
    fn test_metals_method() {
        let method = MetalsMethod::Dm(DensityMixingBuilder::<Pulay>::new().build());
        println!("{}", method);
        let edft = MetalsMethod::Edft;
        println!("{}", edft);
    }
}
