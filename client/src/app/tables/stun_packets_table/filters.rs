//! STUN Packet Filtering
//!
//! This module provides a filtering system for STUN packets with support for complex queries.
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
//!
//! ## STUN-specific Filters
//! - `type:value` - Matches STUN message type (e.g., binding, allocate)
//! - `transaction:value` - Matches STUN transaction ID
//! - `length:value` - Matches STUN message length
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches packets from addresses containing "192.168"
//! - `type:binding` - Matches STUN binding requests/responses
//! - `transaction:1234` - Matches packets with specific transaction ID
//!
//! Complex filters:
//! - `source:10.0.0 AND type:binding` - Binding requests from specific network
//! - `(dest:192.168 OR dest:10.0.0) AND NOT type:allocate` - Non-allocate messages to specific networks
//! - `magic:2112A442 AND length:>100` - Large STUN messages with specific magic cookie

use crate::{
    app::tables::stun_packets_table::StunFilterContext,
    declare_filter_type,
    filter_system::{
        self, CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Type(String),
        Transaction(String),
        Length(ComparisonFilter<usize>)
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
    type Context = StunFilterContext<'a>;

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
                ctx.packet.get_message_type_name().to_lowercase().contains(value)
            }
            FilterType::Transaction(value) => {
                if value.is_empty() {
                    return true;
                }
                // Convert transaction ID bytes to hex string for comparison
                let tx_id = ctx.packet.transaction_id
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                tx_id.contains(value)
            }
            FilterType::Length(filter) => {
                let size = ctx.packet.message_length as usize;
                match filter {
                    ComparisonFilter::GreaterThan(val) => size> *val,
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
    fn parse_filter_value(key: &str, value: &str) -> Result<Self, ParseError> {
        match key {
            "source" => Ok(FilterType::Source(value.to_string())),
            "dest" => Ok(FilterType::Destination(value.to_string())),
            "type" => Ok(FilterType::Type(value.to_string())),
            "transaction" => Ok(FilterType::Transaction(value.to_string())),
            "length" => ComparisonFilter::parse(value)
                .map(FilterType::Length)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid length filter format.\nExamples:\n\
                         - length:1000 (exact match)\n\
                         - length:>500 (greater than)\n\
                         - length:<=1500 (less or equal)"
                            .into(),
                    )
                }),
            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - type: STUN message type filter\n\
                 - transaction: Transaction ID filter\n\
                 - length: Message length filter",
                unknown
            ))),
        }
    }
} 