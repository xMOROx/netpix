//! Packet Table Filter
//!
//! Provides filtering for the network packets table.

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
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
    crate::filter_system::parse_filter(filter)
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

            "protocol" => (!value.is_empty())
                .then(|| value.to_lowercase())
                .map(FilterType::Protocol)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid protocol filter format.\nAvailable filters:\n\
                         - protocol:udp (UDP protocol)\n\
                         - protocol:tcp (TCP protocol)\n\
                         - protocol:rtp (RTP protocol)\n\
                         - protocol:rtcp (RTCP protocol)\n
                         - protocol:mpeg-ts (MPEG Transport Stream)"
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
                - protocol: Match protocol (e.g., protocol:udp)\n\
                - length: Match packet size (e.g., length:>100)\n\
                - type: Match session type (e.g., type:rtp)"
                    .to_string(),
            )),
        }
    }
}
