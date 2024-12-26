//! RTP Packet Filtering
//!
//! This module provides a filtering system for RTP packets with support for complex queries.
//!
//! # Filter Syntax
//!
//! The basic filter syntax is: `field:value`
//!
//! Multiple filters can be combined using:
//! - `AND` - both conditions must match
//! - `OR` - either condition must match
//! - `NOT` - negates the condition
//! - Parentheses `()` for grouping
//!
//! # Available Filters
//!
//! ## Basic Filters
//! - `source:value` - Matches source IP address containing the value
//! - `dest:value` - Matches destination IP address containing the value
//! - `alias:value` - Matches stream alias containing the value
//!
//! ## RTP-specific Filters
//! - `ssrc:value` - Matches SSRC value
//! - `seq:number` - Matches sequence number
//! - `timestamp:number` - Matches RTP timestamp
//!
//! ## Payload Filters
//! - `payload:number` - Matches exact payload size
//! - `payload:>number` - Matches payload size greater than number
//! - `payload:>=number` - Matches payload size greater or equal to number
//! - `payload:<number` - Matches payload size less than number
//! - `payload:<=number` - Matches payload size less or equal to number
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches packets from addresses containing "192.168"
//! - `ssrc:1234` - Matches packets with SSRC 1234
//! - `seq:1000` - Matches packets with sequence number 1000
//! - `payload:>1000` - Matches packets with payload larger than 1000 bytes
//!
//! Complex filters:
//! - `source:10.0.0 AND payload:>1000` - Large packets from specific network
//! - `(dest:192.168 OR dest:10.0.0) AND NOT seq:0` - Non-initial packets to specific networks
//! - `ssrc:1234 AND timestamp:>1000000` - Packets from specific stream after timestamp

use crate::{
    app::tables::rtp_packets_table::RtpFilterContext,
    declare_filter_type,
    filter_system::{
        self, CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};

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
    filter_system::parse_filter(filter)
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = RtpFilterContext<'a>;

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
            FilterType::Alias(value) => {
                if value.is_empty() {
                    return true;
                }
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
            "source" => value
                .contains('.')
                .then_some(Ok(FilterType::Source(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid IP address format (e.g. source:192.168.1.1)".into(),
                    ))
                }),

            "dest" => value
                .contains('.')
                .then_some(Ok(FilterType::Destination(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid IP address format (e.g. dest:192.168.1.1)".into(),
                    ))
                }),

            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Alias filter cannot be empty (e.g. alias:stream_a)".into(),
                    ))
                }),

            "padding" => match value {
                "+" => Ok(FilterType::Padding("+".into())),
                "-" => Ok(FilterType::Padding("-".into())),
                _ => Err(ParseError::InvalidSyntax(
                    "Invalid padding filter value (e.g. padding:+)".into(),
                )),
            },

            "extension" => match value {
                "+" => Ok(FilterType::Extension("+".into())),
                "-" => Ok(FilterType::Extension("-".into())),
                _ => Err(ParseError::InvalidSyntax(
                    "Invalid extension filter value (e.g. extension:+)".into(),
                )),
            },

            "marker" => match value {
                "+" => Ok(FilterType::Marker("+".into())),
                "-" => Ok(FilterType::Marker("-".into())),
                _ => Err(ParseError::InvalidSyntax(
                    "Invalid marker filter value (e.g. marker:+)".into(),
                )),
            },

            "seq" => value
                .parse()
                .map(FilterType::SequenceNumber)
                .map_err(|_| ParseError::InvalidSyntax("Invalid sequence number".into())),

            "timestamp" => ComparisonFilter::parse(value)
                .map(FilterType::Timestamp)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid timestamp filter format.\nExamples:\n\
                         - timestamp:1234 (exact match)\n\
                         - timestamp:>1000 (greater than)\n\
                         - timestamp:<=2000 (less or equal)"
                            .into(),
                    )
                }),

            "payload" => ComparisonFilter::parse(value)
                .map(FilterType::Payload)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid payload filter format.\nExamples:\n\
                         - payload:1000 (exact match)\n\
                         - payload:>500 (greater than)\n\
                         - payload:<=1500 (less or equal)"
                            .into(),
                    )
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - alias: Stream alias filter\n\
                 - ssrc: SSRC value filter\n\
                 - seq: Sequence number filter\n\
                 - timestamp: RTP timestamp filter\n\
                 - payload: Payload size filter",
                unknown
            ))),
        }
    }
}
