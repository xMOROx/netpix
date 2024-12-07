mod comparison;
mod error;
mod lexer;
mod parser;
mod traits;
mod types;

pub use comparison::ComparisonFilter;
pub use error::ParseError;
pub use lexer::Lexer;
pub use parser::{parse_expression, parse_filter, validate_filter_syntax};
pub use traits::*;
pub use types::*;

#[macro_export]
macro_rules! declare_filter_type {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident$(($($type:ty),+))?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant$(($($type),+))?,
            )*
            And(Box<$name>, Box<$name>),
            Or(Box<$name>, Box<$name>),
            Not(Box<$name>),
        }

        impl<'a> $crate::filter_system::FilterCombinator<'a> for $name {
            fn and(left: Self, right: Self) -> Self {
                Self::And(Box::new(left), Box::new(right))
            }

            fn or(left: Self, right: Self) -> Self {
                Self::Or(Box::new(left), Box::new(right))
            }
        }
    };
}
