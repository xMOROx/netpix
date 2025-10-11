//! TURN Channel Data  Packet Filtering
//!
//! This module provides a filtering system for TURN Channel Data packets with support for complex queries
//!
//! # Filter Syntax
//!
//! The basic filter synatax is: `field:value`
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
//! - `channel_id:value` - Matches channel id containing the value
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches packets from addresses containing "192.168"
//! - `channel_id:<=100` - Matches streams with packet count less or equal to 100
//!
//! Complex filters:
//! - `dest:192.168 OR dest:10.0.0` - Packets going to specific networks
//! - `(channel_id:>1000 OR channel_id:<100) AND NOT source:192.168` - Packets from specific source with channel_id in specified range

use crate::{
    app::tables::turn_channel_table::TurnChannelFilterContext,
    declare_filter_type,
    filter_system::{
        self, CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        ChannelId(u16),
        Data(ComparisonFilter<u32>),
        Timestamp(ComparisonFilter<u32>)
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
    type Context = TurnChannelFilterContext<'a>;

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
            FilterType::ChannelId(value) => *ctx.channel_number == *value,
            FilterType::Timestamp(filter) => {
                // No timestamp in TurnChannelFilterContext; treat as unsupported and always true
                let ts: u32 = 0;
                match filter {
                    ComparisonFilter::GreaterThan(val) => ts > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => ts >= *val,
                    ComparisonFilter::LessThan(val) => ts < *val,
                    ComparisonFilter::LessOrEqualThan(val) => ts <= *val,
                    ComparisonFilter::Equals(value) => ts.to_string() == *value,
                }
            }
            FilterType::Data(filter) => {
                // No payload_length in TurnChannelFilterContext; treat as unsupported and always true
                let size: u32 = 0;
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

            "data" => ComparisonFilter::parse(value)
                .map(FilterType::Data)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid payload filter format.\nExamples:\n\
                         - data:1000 (exact match)\n\
                         - data:>500 (greater than)\n\
                         - data:<=1500 (less or equal)"
                            .into(),
                    )
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - channel_id: Channel ID filter\n\
                 - timestamp: Packet timestamp filter\n\
                 - data: Data size filter",
                unknown
            ))),
        }
    }
}
