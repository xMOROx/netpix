use crate::{
    app::tables::rtcp_packets_table::RtcpFilterContext,
    declare_filter_type,
    filter_system::{self, CommonFilterParser, FilterExpression, FilterParser, ParseError},
};

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Type(String)
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
    type Context = RtcpFilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Source(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.source_addr.to_lowercase().contains(value)
            }
            FilterType::Destination(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.destination_addr.to_lowercase().contains(value)
            }
            FilterType::Type(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.packet.get_type_name().to_lowercase().contains(value)
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
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "type" => Ok(FilterType::Type(value.to_lowercase())),
            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - type: RTCP packet type filter",
                unknown
            ))),
        }
    }
}
