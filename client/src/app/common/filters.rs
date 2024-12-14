use std::str::FromStr;
use crate::filter_system::*;

#[macro_export]
macro_rules! declare_table_filter {
    ($filter_type:ident, $context:ident, {
        $(($variant:ident, $value_type:ty, $field:expr, $compare:expr)),*
        $(,)?
    }) => {
        declare_filter_type! {
            pub enum $filter_type {
                $($variant($value_type)),*
            }
        }

        impl CommonFilterParser for $filter_type {
            fn not(expr: Self) -> Self {
                $filter_type::Not(Box::new(expr))
            }
        }

        impl<'a> FilterExpression<'a> for $filter_type {
            type Context = $context<'a>;

            fn matches(&self, ctx: &Self::Context) -> bool {
                match self {
                    $(
                        $filter_type::$variant(value) => $compare(ctx, value),
                    )*
                    $filter_type::And(left, right) => left.matches(ctx) && right.matches(ctx),
                    $filter_type::Or(left, right) => left.matches(ctx) || right.matches(ctx),
                    $filter_type::Not(filter) => !filter.matches(ctx),
                }
            }
        }

        impl FilterParser for $filter_type {
            fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError> {
                match prefix.trim() {
                    $($field => Self::parse_field_value(value).map($filter_type::$variant),)*
                    unknown => Err(ParseError::invalid_filter_type(unknown)),
                }
            }
        }
    };
}

pub fn parse_comparison_filter<T: FromStr>(value: &str) -> Option<ComparisonFilter<T>> {
    if let Some(stripped) = value.strip_prefix('>') {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::GreaterThan)
    } else if let Some(stripped) = value.strip_prefix(">=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::GreaterOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix("<=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::LessOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix('<') {
        stripped.trim().parse().ok().map(ComparisonFilter::LessThan)
    } else {
        match value.parse::<T>() {
            Ok(_) => Some(ComparisonFilter::Equals(value.to_string())),
            Err(_) => None,
        }
    }
}
