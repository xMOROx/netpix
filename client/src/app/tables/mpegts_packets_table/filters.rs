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

use crate::{
    declare_filter_type,
    filter_system::{self, *},
    streams::mpegts_stream::packet_info::MpegTsPacketInfo,
};
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use std::str::FromStr;

pub struct FilterContext<'a> {
    pub packet: &'a MpegTsPacketInfo,
    pub pmt_pids: &'a [PIDTable],
    pub es_pids: &'a [PIDTable],
    pub pcr_pids: &'a [PIDTable],
    pub stream_alias: Option<String>,
    pub es_pids_info: &'a [(u16, &'a str)], // (PID, stream type category)
}

pub enum PacketType {
    Pat,
    Pmt,
    PcrEs,
    Es,
    Pcr,
}

pub enum StreamType {
    Audio,
    Video,
    Other,
}

impl FromStr for StreamType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AUDIO" => Ok(StreamType::Audio),
            "VIDEO" => Ok(StreamType::Video),
            "OTHER" => Ok(StreamType::Other),
            _ => Err(()),
        }
    }
}

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Alias(String),
        Payload(ComparisonFilter<usize>),
        PacketPid(usize, String),
        Pid(u16),
        Type(PacketType),
        StreamType(StreamType),
    }
}

impl CommonFilterParser for FilterType {
    fn not(expr: Self) -> Self {
        FilterType::Not(Box::new(expr))
    }
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

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    filter_system::parse_filter(filter)
}

impl<'a> FilterExpression<'a> for FilterType {
    type Context = FilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
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
                    ComparisonFilter::GreaterThan(size) => payload_size > *size,
                    ComparisonFilter::GreaterOrEqualThan(size) => payload_size >= *size,
                    ComparisonFilter::LessThan(size) => payload_size < *size,
                    ComparisonFilter::LessOrEqualThan(size) => payload_size <= *size,
                    ComparisonFilter::Equals(value) => payload_size.to_string() == *value,
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
            FilterType::StreamType(stream_type) => {
                ctx.packet.content.fragments.iter().any(|fragment| {
                    if let PIDTable::PID(pid) = fragment.header.pid {
                        let stream_type_category = ctx
                            .es_pids_info
                            .iter()
                            .find(|(es_pid, _)| *es_pid == pid)
                            .map(|(_, category)| *category);

                        matches!(
                            (stream_type, stream_type_category),
                            (StreamType::Audio, Some("Audio"))
                                | (StreamType::Video, Some("Video"))
                                | (StreamType::Other, Some("Other"))
                        )
                    } else {
                        false
                    }
                })
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

            "alias" => value
                .is_empty()
                .then_some(Err(ParseError::InvalidSyntax(
                    "Alias filter cannot be empty (e.g. alias:stream_a)".into(),
                )))
                .unwrap_or_else(|| Ok(FilterType::Alias(value.to_lowercase()))),

            "payload" => parse_payload_filter(value).ok_or_else(|| {
                ParseError::InvalidSyntax(
                    "Invalid payload filter format.\nExamples:\n\
                     - payload:188 (exact match)\n\
                     - payload:>100 (greater than)\n\
                     - payload:<=188 (less or equal)\n\
                     - payload:<150 (less than)\n\
                     - payload:>=100 (greater or equal)"
                        .into(),
                )
            }),

            p if p.starts_with('p') && p.len() == 2 => {
                let index_char = p.chars().nth(1).unwrap();
                ('1'..='7')
                    .contains(&index_char)
                    .then(|| {
                        let packet_index = (index_char as usize - '0' as usize) - 1;
                        (!value.is_empty())
                            .then_some(Ok(FilterType::PacketPid(packet_index, value.to_string())))
                            .unwrap_or_else(|| {
                                Err(ParseError::InvalidSyntax(
                                    "PID filter value cannot be empty (e.g. p1:256)".into(),
                                ))
                            })
                    })
                    .unwrap_or_else(|| {
                        Err(ParseError::InvalidSyntax(
                            "Invalid PID position. Must be p1 through p7 (e.g. p1:256)".into(),
                        ))
                    })
            }

            "pid" => value.parse::<u16>().map(FilterType::Pid).map_err(|_| {
                ParseError::InvalidSyntax(
                    "Invalid PID number. Must be 0-8191 (e.g. pid:256)".into(),
                )
            }),

            "type" => PacketType::from_str(value)
                .map(FilterType::Type)
                .map_err(|_| {
                    ParseError::InvalidSyntax(
                        "Invalid packet type.\nMust be one of:\n\
                     - type:PAT (Program Association Table)\n\
                     - type:PMT (Program Map Table)\n\
                     - type:PCR+ES (PCR and Elementary Stream)\n\
                     - type:ES (Elementary Stream)\n\
                     - type:PCR (Program Clock Reference)"
                            .into(),
                    )
                }),

            "stream" => StreamType::from_str(value)
                .map(FilterType::StreamType)
                .map_err(|_| {
                    ParseError::InvalidSyntax(
                        "Invalid stream type. Must be one of:\n\
                         - stream:audio (Audio streams)\n\
                         - stream:video (Video streams)\n\
                         - stream:other (Other streams)"
                            .into(),
                    )
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - alias: Stream alias filter\n\
                 - payload: Payload size filter\n\
                 - p1-p7: PID position filter\n\
                 - pid: PID number filter\n\
                 - type: Packet type filter\n\
                 - stream: Stream type filter",
                unknown
            ))),
        }
    }
}

fn parse_payload_filter(value: &str) -> Option<FilterType> {
    let result = if let Some(stripped) = value.strip_prefix('>') {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::GreaterThan)
    } else if let Some(stripped) = value.strip_prefix(">=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::GreaterOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix("<=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(ComparisonFilter::LessOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix('<') {
        stripped.trim().parse().ok().map(ComparisonFilter::LessThan)
    } else {
        match value.parse::<usize>() {
            Ok(_) => Some(ComparisonFilter::Equals(value.to_string())),
            Err(_) => None,
        }
    };

    result.map(FilterType::Payload)
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
