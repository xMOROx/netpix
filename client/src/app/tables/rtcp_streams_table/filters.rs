use crate::{
    declare_filter_type,
    filter_system::{self, CommonFilterParser, FilterExpression, FilterParser, ParseError},
};
use crate::app::tables::RtcpStreamFilterContext;

declare_filter_type! {
    pub enum FilterType {
        Direction(String),
        SSRC(String),
        Alias(String),
    }
}

impl CommonFilterParser for FilterType {
    fn not(expr: Self) -> Self {
        FilterType::Not(Box::new(expr))
    }
}

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    filter_system::parse_filter(filter)
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = RtcpStreamFilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Direction(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.direction.to_lowercase().contains(value)
            }
            FilterType::SSRC(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.ssrc.to_lowercase().contains(value)
            }
            FilterType::Alias(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.alias.to_lowercase() == value.to_lowercase()
            }
            FilterType::And(left, right) => left.matches(ctx) && right.matches(ctx),
            FilterType::Or(left, right) => left.matches(ctx) || right.matches(ctx),
            FilterType::Not(filter) => !filter.matches(ctx),
        }
    }
}

impl FilterParser for FilterType {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError> {
        match prefix.trim() {
            "dir" => (!value.is_empty())
                .then_some(Ok(FilterType::Direction(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid direction must be incoming or outgoing".into(),
                    ))
                }),

            "ssrc" => (!value.is_empty())
                .then_some(Ok(FilterType::SSRC(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid SSRC format (e.g. ssrc:1a99de77e".into(),
                    ))
                }),

            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid Alias format (e.g. alias:A)".into(),
                    ))
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - direction: e.g. direction:incoming direction:outgoing\n\
                 - ssrc: Identifying ID for data\n\
                 - alias: Filter SSRCs by alias",
                unknown
            ))),
        }
    }
}
