//! RTP Stream Filtering
//!
//! This module provides a filtering system for RTP streams with support for complex queries.
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
//! - `ssrc:value` - Matches SSRC value in hex format
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches streams from addresses containing "192.168"
//! - `ssrc:1234` - Matches streams with SSRC 1234
//! - `alias:test` - Matches streams with alias containing "test"
//!
//! Complex filters:
//! - `source:10.0.0 AND NOT alias:test` - Streams from specific network without test alias
//! - `dest:192.168 OR dest:10.0.0` - Streams going to specific networks
//! - `ssrc:1234 AND source:192.168` - Specific stream from specific source

use crate::app::rtp_streams_table::RtpStreamFilterContext;
use crate::filter_system::{CommonFilterParser, FilterExpression, FilterParser, ParseError};
use crate::{declare_filter_type, filter_system};

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
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
    type Context = RtpStreamFilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Source(value) => ctx.source_addr.to_lowercase().contains(value),
            FilterType::Destination(value) => ctx.destination_addr.to_lowercase().contains(value),
            FilterType::Alias(value) => ctx.alias.to_lowercase().contains(value),
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

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter (e.g. source:192.168.1.1)\n\
                 - dest: Destination IP filter (e.g. dest:192.168.1.1)\n\
                 - alias: Stream alias filter (e.g. alias:test_stream)\n\
                 - ssrc: SSRC value filter in hex (e.g. ssrc:1a2b)",
                unknown
            ))),
        }
    }
}
