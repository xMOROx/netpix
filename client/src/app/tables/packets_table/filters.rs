//! Network Packet Filtering
//!
//! This module provides a filtering system for network packets with support for complex queries.
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
//! ## Protocol Filters
//! - `proto:value` or `protocol:value` - Matches transport or session protocol name
//! - `type:value` - Matches specific session protocol (RTP, RTCP, etc.)
//!
//! ## Size Filters
//! - `length:number` - Matches exact packet length
//! - `length:>number` - Matches length greater than number
//! - `length:>=number` - Matches length greater or equal to number
//! - `length:<number` - Matches length less than number
//! - `length:<=number` - Matches length less or equal to number
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches packets from addresses containing "192.168"
//! - `proto:udp` - Matches UDP packets
//! - `type:rtp` - Matches RTP packets
//! - `length:>100` - Matches packets larger than 100 bytes
//!
//! Complex filters:
//! - `proto:udp AND length:>100` - UDP packets larger than 100 bytes
//! - `source:10.0.0 OR source:192.168` - Packets from specific networks
//! - `(type:rtp OR type:rtcp) AND NOT dest:10.0.0.1` - RTP/RTCP packets not going to specific host
//! - `proto:tcp AND length:>=1500` - TCP packets with maximum size

use crate::{
    declare_filter_type,
    filter_system::{
        self, CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};
use netpix_common::packet::{Packet, SessionProtocol};
use std::str::FromStr;

pub struct FilterContext<'a> {
    pub packet: &'a Packet,
}

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Protocol(String),
        Length(ComparisonFilter<usize>),
        Type(SessionProtocol)
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
            FilterType::Protocol(value) => {
                ctx.packet
                    .transport_protocol
                    .to_string()
                    .to_lowercase()
                    .contains(value)
                    || ctx
                        .packet
                        .session_protocol
                        .to_string()
                        .to_lowercase()
                        .contains(value)
            }
            FilterType::Length(comp) => {
                let length = ctx.packet.length as usize;
                match comp {
                    ComparisonFilter::GreaterThan(size) => length > *size,
                    ComparisonFilter::GreaterOrEqualThan(size) => length >= *size,
                    ComparisonFilter::LessThan(size) => length < *size,
                    ComparisonFilter::LessOrEqualThan(size) => length <= *size,
                    ComparisonFilter::Equals(value) => length.to_string() == *value,
                }
            }
            FilterType::Type(protocol) => ctx.packet.session_protocol == *protocol,
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

            "proto" | "protocol" => (!value.is_empty())
                .then(|| value.to_lowercase())
                .map(FilterType::Protocol)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid protocol filter format.\nAvailable filters:\n\
                         - proto:udp (UDP protocol)\n\
                         - proto:tcp (TCP protocol)\n\
                         - proto:rtp (RTP protocol)\n\
                         - proto:rtcp (RTCP protocol)\n
                         - proto:mpeg-ts (MPEG Transport Stream)"
                            .into(),
                    )
                }),

            "length" => ComparisonFilter::parse(value)
                .map(FilterType::Length)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid length filter format.\nExamples:\n\
                         - length:188 (exact match)\n\
                         - length:>100 (greater than)\n\
                         - length:<=188 (less or equal)\n\
                         - length:<150 (less than)\n\
                         - length:>=100 (greater or equal)"
                            .into(),
                    )
                }),
            "type" => SessionProtocol::from_str(value)
                .map(FilterType::Type)
                .map_err(|_| {
                    ParseError::InvalidSyntax(
                        "Invalid packet type.\nMust be one of:\n\
                     - type:RTP (Real-time Transport Protocol)\n\
                     - type:RTCP (Real-time Control Protocol)\n\
                     - type:MPEG-TS (MPEG Transport Stream)\n"
                            .into(),
                    )
                }),
            _ => Err(ParseError::InvalidSyntax(
                "Unknown filter type.\nAvailable filters:\n\
                - source: Match source IP (e.g., source:192.168)\n\
                - dest: Match destination IP (e.g., dest:10.0.0)\n\
                - proto/protocol: Match protocol (e.g., proto:udp)\n\
                - length: Match packet size (e.g., length:>100)\n\
                - type: Match session type (e.g., type:rtp)"
                    .to_string(),
            )),
        }
    }
}
