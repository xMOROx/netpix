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

use crate::{declare_filter_type, filter_system::*};
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

fn parse_primary(lexer: &mut Lexer) -> ParseResult<FilterType> {
    let token = lexer
        .next_token()
        .ok_or(ParseError::InvalidToken("Empty token".into()))?;

    match token {
        Token::OpenParen => {
            let expr = parse_expression(lexer, 0, parse_primary)?;
            match lexer.next_token() {
                Some(Token::CloseParen) => Ok(expr),
                Some(_other) => Err(ParseError::UnmatchedParenthesis),
                None => Err(ParseError::UnmatchedParenthesis),
            }
        }
        Token::Not => {
            let expr = parse_primary(lexer)?;
            Ok(FilterType::Not(Box::new(expr)))
        }
        Token::Filter(prefix) => {
            if lexer.next_token() != Some(Token::Colon) {
                return Err(ParseError::InvalidSyntax(format!(
                    "Missing colon after '{}' filter",
                    prefix
                )));
            }

            match lexer.next_token() {
                Some(Token::Filter(value)) => FilterType::parse_filter_value(&prefix, &value),
                _ => Err(ParseError::InvalidSyntax(format!(
                    "Missing value after '{}':",
                    prefix
                ))),
            }
        }
        other => Err(ParseError::InvalidToken(format!(
            "Unexpected token: {:?}",
            other
        ))),
    }
}

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    validate_filter_syntax(filter)?;

    let mut lexer = Lexer::new(filter);
    parse_expression(&mut lexer, 0, parse_primary).map_err(|e| e.into())
}
