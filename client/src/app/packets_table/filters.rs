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

use netpix_common::packet::{Packet, SessionProtocol};
use std::str::FromStr;
use crate::{declare_filter_type, filter_system};
use crate::filter_system::{CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError};

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
        match prefix.trim() {
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "proto" | "protocol" => Ok(FilterType::Protocol(value.to_lowercase())),
            "length" => ComparisonFilter::parse(value)
                .map(FilterType::Length)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid length filter format".into())),
            "type" => SessionProtocol::from_str(value)
                .map(FilterType::Type)
                .map_err(|_| ParseError::InvalidSyntax("Invalid protocol type".into())),
            _ => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'",
                prefix
            ))),
        }
    }
}
