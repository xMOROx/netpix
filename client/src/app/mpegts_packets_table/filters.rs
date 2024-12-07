//! MPEG-TS Packet Filtering
//!
//! This module provides a filtering system for MPEG-TS packets with support for complex queries.
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
//! - `desc:value` - Matches packet ID containing the value
//! - `source:value` - Matches source IP address containing the value
//! - `dest:value` - Matches destination IP address containing the value
//! - `alias:value` - Matches stream alias containing the value
//!
//! ## PID Filters
//! - `pid:number` - Matches exact PID number
//! - `p1:value` to `p7:value` - Matches PID at specific position (1-7) containing the value
//!
//! ## Payload Size Filters
//! - `payload:number` - Matches exact payload size
//! - `payload:>number` - Matches payload size greater than number
//! - `payload:>=number` - Matches payload size greater or equal to number
//! - `payload:<number` - Matches payload size less than number
//! - `payload:<=number` - Matches payload size less or equal to number
//!
//! ## Packet Type Filters
//! - `type:PAT` - Program Association Table packets
//! - `type:PMT` - Program Map Table packets
//! - `type:PCR+ES` - Packets containing both PCR and Elementary Stream
//! - `type:ES` - Elementary Stream packets
//! - `type:PCR` - Program Clock Reference packets
//!
//! # Examples
//!
//! Simple filters:
//! - `desc:123` - Matches packets with ID containing "123"
//! - `source:192.168` - Matches packets from IP addresses containing "192.168"
//! - `pid:256` - Matches packets with PID 256
//! - `type:PAT` - Matches PAT packets
//!
//! Complex filters:
//! - `type:ES AND payload:>184` - ES packets with payload larger than 184 bytes
//! - `(source:10.0.0 OR source:192.168) AND NOT type:PCR` - Non-PCR packets from specific networks
//! - `pid:256 AND (type:ES OR type:PCR+ES)` - ES or PCR+ES packets with PID 256
//! - `alias:A AND payload:>=188` - Packets from stream aliased as "A" with full payloads

use crate::filter_system::{
    parse_expression, FilterCombinator, FilterExpression, Lexer, ParseError, ParseResult, Token,
};
use crate::streams::mpegts_stream::packet_info::MpegTsPacketInfo;
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use std::collections::VecDeque;
use std::str::FromStr;

pub trait PacketFilter {
    fn matches(&self, info: &FilterContext) -> bool;
}

pub struct FilterContext<'a> {
    pub packet: &'a MpegTsPacketInfo,
    pub pmt_pids: &'a [PIDTable],
    pub es_pids: &'a [PIDTable],
    pub pcr_pids: &'a [PIDTable],
    pub stream_alias: Option<String>,
}

pub enum FilterType {
    Description(String),
    Source(String),
    Destination(String),
    Alias(String),
    Payload(PayloadFilter),
    PacketPid(usize, String),
    Pid(u16),
    Type(PacketType),
    And(Box<FilterType>, Box<FilterType>),
    Or(Box<FilterType>, Box<FilterType>),
    Not(Box<FilterType>),
}

pub enum PacketType {
    Pat,
    Pmt,
    PcrEs,
    Es,
    Pcr,
}

pub enum PayloadFilter {
    GreaterOrEqualThan(usize),
    GreaterThan(usize),
    LessOrEqualThan(usize),
    LessThan(usize),
    Equals(String),
}

impl FromStr for PacketType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAT" => Ok(PacketType::Pat),
            "PMT" => Ok(PacketType::Pmt),
            "PCR+ES" => Ok(PacketType::PcrEs),
            "ES" => Ok(PacketType::Es),
            "PCR" => Ok(PacketType::Pcr),
            _ => Err(()),
        }
    }
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = FilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Description(value) => ctx.packet.id.to_string().contains(value),
            FilterType::Source(value) => ctx
                .packet
                .packet_association_table
                .source_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Destination(value) => ctx
                .packet
                .packet_association_table
                .destination_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Alias(value) => ctx
                .stream_alias
                .as_ref()
                .map(|alias| alias.to_lowercase().contains(value))
                .unwrap_or(false),
            FilterType::Payload(payload_filter) => {
                let payload_size = calculate_payload_size(ctx.packet);
                match payload_filter {
                    PayloadFilter::GreaterThan(size) => payload_size > *size,
                    PayloadFilter::GreaterOrEqualThan(size) => payload_size >= *size,
                    PayloadFilter::LessThan(size) => payload_size < *size,
                    PayloadFilter::LessOrEqualThan(size) => payload_size <= *size,
                    PayloadFilter::Equals(value) => payload_size.to_string() == *value,
                }
            }
            FilterType::PacketPid(index, value) => ctx
                .packet
                .content
                .fragments
                .get(*index)
                .map(|fragment| match &fragment.header.pid {
                    PIDTable::PID(pid) => pid.to_string().contains(value),
                    _ => fragment
                        .header
                        .pid
                        .to_string()
                        .to_lowercase()
                        .contains(value),
                })
                .unwrap_or(false),
            FilterType::Pid(pid_value) => ctx.packet.content.fragments.iter().any(
                |fragment| matches!(fragment.header.pid, PIDTable::PID(pid) if pid == *pid_value),
            ),
            FilterType::Type(packet_type) => match_packet_type(ctx, packet_type),
            FilterType::And(left, right) => left.matches(ctx) && right.matches(ctx),
            FilterType::Or(left, right) => left.matches(ctx) || right.matches(ctx),
            FilterType::Not(filter) => !filter.matches(ctx),
        }
    }
}

impl<'a> FilterCombinator<'a> for FilterType {
    fn and(left: Self, right: Self) -> Self {
        FilterType::And(Box::new(left), Box::new(right))
    }

    fn or(left: Self, right: Self) -> Self {
        FilterType::Or(Box::new(left), Box::new(right))
    }
}

pub fn parse_filter(filter: &str) -> Option<FilterType> {
    let mut lexer = Lexer::new(filter);
    parse_expression(&mut lexer, 0, parse_primary).ok()
}

fn parse_primary(lexer: &mut Lexer) -> ParseResult<FilterType> {
    let token = lexer.next_token().ok_or(ParseError::InvalidToken)?;
    match token {
        Token::OpenParen => {
            let expr = parse_expression(lexer, 0, parse_primary)?;
            match lexer.next_token() {
                Some(Token::CloseParen) => Ok(expr),
                _ => Err(ParseError::UnmatchedParenthesis),
            }
        }
        Token::Not => {
            let expr = parse_primary(lexer)?;
            Ok(FilterType::Not(Box::new(expr)))
        }
        Token::Filter(prefix) => {
            if lexer.next_token() != Some(Token::Colon) {
                return Err(ParseError::InvalidToken);
            }
            match lexer.next_token() {
                Some(Token::Filter(value)) => {
                    parse_filter_with_value(&prefix, &value).ok_or(ParseError::InvalidToken)
                }
                _ => Err(ParseError::InvalidToken),
            }
        }
        _ => Err(ParseError::InvalidToken),
    }
}

fn parse_filter_with_value(prefix: &str, value: &str) -> Option<FilterType> {
    match prefix.trim() {
        "desc" => Some(FilterType::Description(value.to_string())),
        "source" => Some(FilterType::Source(value.to_lowercase())),
        "dest" => Some(FilterType::Destination(value.to_lowercase())),
        "alias" => Some(FilterType::Alias(value.to_lowercase())),
        "payload" => parse_payload_filter(value),
        p if p.starts_with('p') && p.len() == 2 => {
            let packet_index = p[1..].parse::<usize>().ok()?.saturating_sub(1);
            Some(FilterType::PacketPid(packet_index, value.to_string()))
        }
        "pid" => value.parse().ok().map(FilterType::Pid),
        "type" => PacketType::from_str(value).ok().map(FilterType::Type),
        _ => None,
    }
}

fn parse_payload_filter(value: &str) -> Option<FilterType> {
    if let Some(stripped) = value.strip_prefix('>') {
        stripped.trim().parse().ok().map(PayloadFilter::GreaterThan)
    } else if let Some(stripped) = value.strip_prefix(">=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(PayloadFilter::GreaterOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix("<=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(PayloadFilter::LessOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix('<') {
        stripped.trim().parse().ok().map(PayloadFilter::LessThan)
    } else {
        Some(PayloadFilter::Equals(value.to_string()))
    }
    .map(FilterType::Payload)
}

fn calculate_payload_size(packet: &MpegTsPacketInfo) -> usize {
    packet
        .content
        .fragments
        .iter()
        .filter(|f| {
            f.header.adaptation_field_control != AdaptationFieldControl::AdaptationFieldOnly
        })
        .filter_map(|f| f.payload.as_ref())
        .map(|p| p.data.len())
        .sum()
}

fn match_packet_type(ctx: &FilterContext, packet_type: &PacketType) -> bool {
    ctx.packet
        .content
        .fragments
        .iter()
        .any(|fragment| match (packet_type, &fragment.header.pid) {
            (PacketType::Pat, PIDTable::ProgramAssociation) => true,
            (_, PIDTable::PID(pid)) => {
                let is_pmt = ctx.pmt_pids.contains(&PIDTable::PID(*pid));
                let is_es = ctx.es_pids.contains(&PIDTable::PID(*pid));
                let is_pcr = ctx.pcr_pids.contains(&PIDTable::PID(*pid));

                matches!(
                    (packet_type, is_pmt, is_es, is_pcr),
                    (PacketType::Pmt, true, _, _)
                        | (PacketType::PcrEs, _, true, true)
                        | (PacketType::Es, _, true, false)
                        | (PacketType::Pcr, _, false, true)
                )
            }
            _ => false,
        })
}
