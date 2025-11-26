//! RTCP Packet Filtering

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, FilterExpression, FilterParser, ParseError,
    },
};
use netpix_common::rtcp::RtcpPacket;

pub struct FilterContext<'a> {
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
    pub packet: &'a RtcpPacket,
}

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
    crate::filter_system::parse_filter(filter)
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = FilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Source(value) => {
                if value.is_empty() { return true; }
                ctx.source_addr.to_lowercase().contains(value)
            }
            FilterType::Destination(value) => {
                if value.is_empty() { return true; }
                ctx.destination_addr.to_lowercase().contains(value)
            }
            FilterType::Type(value) => {
                if value.is_empty() { return true; }
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
            _ => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter: {}. Use: source, dest, type (sender/receiver/sdes/bye)",
                prefix
            ))),
        }
    }
}
