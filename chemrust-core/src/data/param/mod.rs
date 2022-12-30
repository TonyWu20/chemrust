mod attributes;
mod kpoints;
mod markers;
mod model_params;

pub use attributes::*;
pub use model_params::ModelParameters;

#[cfg(test)]
mod test {
    use crate::data::format::cell::Cell;

    use super::{attributes::KPoint, ModelParameters};

    #[test]
    fn test_param_attr() {
        let kpt = KPoint::<Cell>::default();
        assert_eq!(&[0.0, 0.0, 0.0, 1.0], kpt.content());
    }
    #[test]
    fn test_params() {
        println!("{:?}", ModelParameters::<Cell>::default());
    }
}
