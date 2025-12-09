//! RTCP Packet Filtering
//!
//! This module provides a filtering system for RTCP packets with support for complex queries.
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
//! ## Type Filters
//! - `type:sender` - Matches sender report packets
//! - `type:receiver` - Matches receiver report packets
//! - `type:sdes` - Matches source description packets
//! - `type:bye` - Matches goodbye packets
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches packets from addresses containing "192.168"
//! - `type:sender` - Matches sender report packets
//!
//! Complex filters:
//! - `source:192.168 AND type:sender` - Sender reports from specific address
//! - `dest:10.0.0 OR dest:192.168` - Packets going to specific networks
//! - `NOT type:bye` - All packets except goodbye messages

use crate::{
    app::tables::rtcp_packets_table::RtcpFilterContext,
    declare_filter_type,
    filter_system::{self, CommonFilterParser, FilterExpression, FilterParser, ParseError},
};

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Type(String),
        Direction(String),
        SSRC(String),
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
            FilterType::Direction(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.direction.to_lowercase().contains(value)
            }
            FilterType::SSRC(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.ssrc.to_lowercase().contains(value)
            }
            FilterType::Alias(value) => {
                if value.is_empty() {
                    return true;
                }
                ctx.alias.to_lowercase() == value.to_lowercase()
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

            "type" => (!value.is_empty())
                .then_some(Ok(FilterType::Type(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid RTCP packet type.\nAvailable types:\n\
                         - type:sender (Sender Report)\n\
                         - type:receiver (Receiver Report)\n\
                         - type:sdes (Source Description)\n\
                         - type:bye (Goodbye)"
                            .into(),
                    ))
                }),

            "dir" => (!value.is_empty())
                .then_some(Ok(FilterType::Direction(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid direction must be incoming or outgoing".into(),
                    ))
                }),

            "ssrc" => (!value.is_empty())
                .then_some(Ok(FilterType::SSRC(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid SSRC format (e.g. ssrc:1a99de77e".into(),
                    ))
                }),

            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Invalid Alias format (e.g. alias:A)".into(),
                    ))
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter (e.g. source:192.168)\n\
                 - dest: Destination IP filter (e.g. dest:10.0.0)\n\
                 - type: RTCP packet type filter (e.g. type:sender)\n\
                 - direction: e.g. direction:incoming direction:outgoing\n\
                 - ssrc: Identifying ID for data\n\
                 - alias: Filter SSRCs by alias",
                unknown
            ))),
        }
    }
}
