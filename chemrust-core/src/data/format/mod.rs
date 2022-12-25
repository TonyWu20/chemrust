use std::fmt::Debug;

pub mod cell;
pub mod msi;

/// Trait bound for Model Formats
pub trait DataFormat: Debug + Clone + Default {}

pub trait BlockWriter {
    fn format_block(block_name: &str, block_content: &str) -> String;
}

#[macro_export]
macro_rules! impl_display {
    ($target: ty, $format: expr) => {
        impl Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, self.content())
            }
        }
    };
    ($target: ty, $format: expr, $($field:ident), *) => {
        impl Display for $target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, $(self.content().$field,)*)
            }
        }
    };
}
