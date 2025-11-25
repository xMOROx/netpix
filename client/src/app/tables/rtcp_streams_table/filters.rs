//! RTCP Streams Filtering
//!
//! This module provides a filtering system for RTCP streams with support for complex queries.
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
//! ## Address Filters
//! - `source:value` - Matches source IP address containing the value
//! - `dest:value` - Matches destination IP address containing the value
//!

use crate::app::tables::rtcp_streams_table::RtcpStreamFilterContext;
use crate::app::tables::RtpStreamFilterContext;
use crate::{
    declare_filter_type, // Assuming this macro exists from the common filter system
    filter_system::{self, CommonFilterParser, FilterExpression, FilterParser, ParseError},
};

// Declare the types of filters specific to RTCP Streams
declare_filter_type! {
    pub enum FilterType {
        Ssrc(u32),
    }
}

// Implement the basic AND/OR/NOT logic using the common trait
impl CommonFilterParser for FilterType {
    fn not(expr: Self) -> Self {
        FilterType::Not(Box::new(expr))
    }
}

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    filter_system::parse_filter(filter)
}

// Implement how the filter expression evaluates against stream data
impl<'a> FilterExpression<'a> for FilterType {
    // The context for stream filtering is the RtcpStreamData itself
    type Context = RtcpStreamFilterContext<'a>; // Using the concrete type directly

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Ssrc(value) => ctx.stream.ssrc == *value,
            FilterType::And(left, right) => left.matches(ctx) && right.matches(ctx),
            FilterType::Or(left, right) => left.matches(ctx) || right.matches(ctx),
            FilterType::Not(filter) => !filter.matches(ctx),
        }
    }
}

// Implement the parser logic for specific filter keys (like "ssrc:")
impl FilterParser for FilterType {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError> {
        match prefix.trim() {
            "ssrc" => {
                let ssrc_val = if let Some(hex_val) = value.strip_prefix("0x") {
                    u32::from_str_radix(hex_val, 16)
                } else {
                    value.parse::<u32>()
                };

                ssrc_val
                    .map(FilterType::Ssrc)
                    .map_err(|_| ParseError::InvalidSyntax(
                        format!("Invalid SSRC value: '{}'. Use decimal (e.g., 12345) or hex (e.g., 0xabcd).", value)
                    ))
            }
            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - ssrc: Filter by SSRC (e.g. ssrc:0x1234abcd or ssrc:12345)",
                unknown
            ))),
        }
    }
}
