//! Filter for MPEG-TS streams table

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
    streams::mpegts_stream::MpegTsStream,
};

pub struct FilterContext<'a> {
    pub stream: &'a MpegTsStream,
}

declare_filter_type! {
    pub enum FilterType {
        Alias(String),
        Source(String),
        Destination(String),
        Substreams(ComparisonFilter<usize>),
        Packets(ComparisonFilter<usize>)
    }
}

impl CommonFilterParser for FilterType {
    fn not(expr: Self) -> Self {
        FilterType::Not(Box::new(expr))
    }
}

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    crate::filter_system::parse_filter(filter)
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = FilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Alias(value) => ctx
                .stream
                .alias
                .to_lowercase()
                .contains(&value.to_lowercase()),
            FilterType::Source(value) => ctx
                .stream
                .stream_info
                .packet_association_table
                .source_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Destination(value) => ctx
                .stream
                .stream_info
                .packet_association_table
                .destination_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Substreams(comp) => {
                let count = ctx.stream.substreams.len();
                match comp {
                    ComparisonFilter::GreaterThan(size) => count > *size,
                    ComparisonFilter::GreaterOrEqualThan(size) => count >= *size,
                    ComparisonFilter::LessThan(size) => count < *size,
                    ComparisonFilter::LessOrEqualThan(size) => count <= *size,
                    ComparisonFilter::Equals(value) => count.to_string() == *value,
                }
            }
            FilterType::Packets(comp) => {
                let count = ctx.stream.stream_info.packets.len();
                match comp {
                    ComparisonFilter::GreaterThan(size) => count > *size,
                    ComparisonFilter::GreaterOrEqualThan(size) => count >= *size,
                    ComparisonFilter::LessThan(size) => count < *size,
                    ComparisonFilter::LessOrEqualThan(size) => count <= *size,
                    ComparisonFilter::Equals(value) => count.to_string() == *value,
                }
            }
            FilterType::And(left, right) => left.matches(ctx) && right.matches(ctx),
            FilterType::Or(left, right) => left.matches(ctx) || right.matches(ctx),
            FilterType::Not(filter) => !filter.matches(ctx),
        }
    }
}

impl FilterParser for FilterType {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError> {
        if value.trim().is_empty() {
            return Err(ParseError::InvalidSyntax(format!(
                "Empty value for filter '{}'.\nExample: {}:value",
                prefix, prefix
            )));
        }

        match prefix.trim() {
            "alias" | "name" => Ok(FilterType::Alias(value.to_string())),
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "substreams" | "subs" => ComparisonFilter::parse(value)
                .map(FilterType::Substreams)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid substreams filter format.\nExamples:\n\
                         - substreams:5 (exact match)\n\
                         - substreams:>3 (greater than)\n\
                         - substreams:<10 (less than)"
                            .into(),
                    )
                }),
            "packets" | "pkts" => ComparisonFilter::parse(value)
                .map(FilterType::Packets)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid packets filter format.\nExamples:\n\
                         - packets:100 (exact match)\n\
                         - packets:>50 (greater than)"
                            .into(),
                    )
                }),
            _ => Err(ParseError::InvalidSyntax(
                "Unknown filter type.\nAvailable filters:\n\
                - alias: Match stream alias (e.g., alias:stream1)\n\
                - source: Match source IP (e.g., source:192.168)\n\
                - dest: Match destination IP (e.g., dest:10.0.0)\n\
                - substreams: Match substream count (e.g., substreams:>5)\n\
                - packets: Match packet count (e.g., packets:>100)"
                    .to_string(),
            )),
        }
    }
}

/// Build help text for filter
pub fn build_filter_help() -> &'static str {
    r#"MPEG-TS Streams Filter Help:
    
• alias:<name> - Filter by stream alias
• source:<ip> - Filter by source IP address
• dest:<ip> - Filter by destination IP address  
• substreams:<num> - Filter by substream count
• substreams:>N - More than N substreams
• packets:<num> - Filter by packet count
• packets:>N - More than N packets

Operators: AND, OR, NOT, parentheses

Examples:
• alias:stream1 AND substreams:>5
• source:192.168 OR dest:10.0
• packets:>100"#
}
