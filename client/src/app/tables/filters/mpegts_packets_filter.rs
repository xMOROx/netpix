//! Filter for MPEG-TS packets table

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};
use netpix_common::{MpegtsPacket, Packet};
use netpix_common::mpegts::header::PIDTable;

pub struct FilterContext<'a> {
    pub packet: &'a Packet,
    pub mpegts: &'a MpegtsPacket,
}

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Pid(u16),
        Fragments(ComparisonFilter<usize>)
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
            FilterType::Source(value) => ctx
                .packet
                .source_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Destination(value) => ctx
                .packet
                .destination_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Pid(pid) => ctx.mpegts.fragments.iter().any(|f| {
                match f.header.pid {
                    PIDTable::PID(p) => p == *pid,
                    _ => false,
                }
            }),
            FilterType::Fragments(comp) => {
                let count = ctx.mpegts.number_of_fragments as usize;
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
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "pid" => value
                .parse::<u16>()
                .map(FilterType::Pid)
                .map_err(|_| ParseError::InvalidSyntax("Invalid PID value".into())),
            "fragments" | "frags" => ComparisonFilter::parse(value)
                .map(FilterType::Fragments)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid fragments filter format.\nExamples:\n\
                         - fragments:7 (exact match)\n\
                         - fragments:>5 (greater than)\n\
                         - fragments:<10 (less than)"
                            .into(),
                    )
                }),
            _ => Err(ParseError::InvalidSyntax(
                "Unknown filter type.\nAvailable filters:\n\
                - source: Match source IP (e.g., source:192.168)\n\
                - dest: Match destination IP (e.g., dest:10.0.0)\n\
                - pid: Match PID (e.g., pid:100)\n\
                - fragments: Match fragment count (e.g., fragments:>5)"
                    .to_string(),
            )),
        }
    }
}

/// Build help text for filter
pub fn build_filter_help() -> &'static str {
    r#"MPEG-TS Packets Filter Help:
    
• source:<ip> - Filter by source IP address
• dest:<ip> - Filter by destination IP address  
• pid:<number> - Filter by PID
• fragments:<num> - Filter by fragment count
• fragments:>N - More than N fragments
• fragments:<N - Less than N fragments

Operators: AND, OR, NOT, parentheses

Examples:
• source:192.168 AND pid:100
• fragments:>5 OR pid:0
• NOT dest:10.0.0.1"#
}
