//! RTP Packet Filtering

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};
use netpix_common::rtp::RtpPacket;

pub struct FilterContext<'a> {
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
    pub alias: &'a str,
    pub packet: &'a RtpPacket,
}

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Alias(String),
        Padding(String),
        Extension(String),
        Marker(String),
        SequenceNumber(u16),
        Timestamp(ComparisonFilter<u32>),
        Payload(ComparisonFilter<usize>)
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
            FilterType::Alias(value) => {
                if value.is_empty() { return true; }
                ctx.alias.to_lowercase().contains(value)
            }
            FilterType::Padding(value) => match value.as_str() {
                "+" => ctx.packet.padding,
                "-" => !ctx.packet.padding,
                _ => true,
            },
            FilterType::Extension(value) => match value.as_str() {
                "+" => ctx.packet.extension,
                "-" => !ctx.packet.extension,
                _ => true,
            },
            FilterType::Marker(value) => match value.as_str() {
                "+" => ctx.packet.marker,
                "-" => !ctx.packet.marker,
                _ => true,
            },
            FilterType::SequenceNumber(value) => ctx.packet.sequence_number == *value,
            FilterType::Timestamp(filter) => {
                let ts = ctx.packet.timestamp;
                match filter {
                    ComparisonFilter::GreaterThan(val) => ts > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => ts >= *val,
                    ComparisonFilter::LessThan(val) => ts < *val,
                    ComparisonFilter::LessOrEqualThan(val) => ts <= *val,
                    ComparisonFilter::Equals(value) => ts.to_string() == *value,
                }
            }
            FilterType::Payload(filter) => {
                let size = ctx.packet.payload_length;
                match filter {
                    ComparisonFilter::GreaterThan(val) => size > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => size >= *val,
                    ComparisonFilter::LessThan(val) => size < *val,
                    ComparisonFilter::LessOrEqualThan(val) => size <= *val,
                    ComparisonFilter::Equals(value) => size.to_string() == *value,
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
        match prefix.trim() {
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "alias" => Ok(FilterType::Alias(value.to_lowercase())),
            "padding" => match value {
                "+" => Ok(FilterType::Padding("+".into())),
                "-" => Ok(FilterType::Padding("-".into())),
                _ => Err(ParseError::InvalidSyntax("Use padding:+ or padding:-".into())),
            },
            "extension" => match value {
                "+" => Ok(FilterType::Extension("+".into())),
                "-" => Ok(FilterType::Extension("-".into())),
                _ => Err(ParseError::InvalidSyntax("Use extension:+ or extension:-".into())),
            },
            "marker" => match value {
                "+" => Ok(FilterType::Marker("+".into())),
                "-" => Ok(FilterType::Marker("-".into())),
                _ => Err(ParseError::InvalidSyntax("Use marker:+ or marker:-".into())),
            },
            "seq" => value.parse().map(FilterType::SequenceNumber)
                .map_err(|_| ParseError::InvalidSyntax("Invalid sequence number".into())),
            "timestamp" => ComparisonFilter::parse(value)
                .map(FilterType::Timestamp)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid timestamp filter".into())),
            "payload" => ComparisonFilter::parse(value)
                .map(FilterType::Payload)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid payload filter".into())),
            _ => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter: {}. Use: source, dest, alias, padding, extension, marker, seq, timestamp, payload",
                prefix
            ))),
        }
    }
}
