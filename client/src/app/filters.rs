use crate::streams::mpegts_stream::MpegTsPacketInfo;
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
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
    PacketPID(usize, String),
    PID(u16),
    Type(PacketType),
}

pub enum PacketType {
    PAT,
    PMT,
    PCRES,
    ES,
    PCR,
}

pub enum PayloadFilter {
    GreaterThan(usize),
    LessThan(usize),
    Equals(String),
}

impl FromStr for PacketType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAT" => Ok(PacketType::PAT),
            "PMT" => Ok(PacketType::PMT),
            "PCR+ES" => Ok(PacketType::PCRES),
            "ES" => Ok(PacketType::ES),
            "PCR" => Ok(PacketType::PCR),
            _ => Err(()),
        }
    }
}

impl PacketFilter for FilterType {
    fn matches(&self, ctx: &FilterContext) -> bool {
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
                    PayloadFilter::LessThan(size) => payload_size < *size,
                    PayloadFilter::Equals(value) => payload_size.to_string().contains(value),
                }
            }
            FilterType::PacketPID(index, value) => ctx
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
            FilterType::PID(pid_value) => ctx.packet.content.fragments.iter().any(
                |fragment| matches!(fragment.header.pid, PIDTable::PID(pid) if pid == *pid_value),
            ),
            FilterType::Type(packet_type) => match_packet_type(ctx, packet_type),
        }
    }
}

pub fn parse_filter(filter: &str) -> Option<FilterType> {
    if let Some((prefix, value)) = filter.split_once(':') {
        let value = value.trim().to_lowercase();
        match prefix.trim() {
            "desc" => Some(FilterType::Description(value)),
            "source" => Some(FilterType::Source(value)),
            "dest" => Some(FilterType::Destination(value)),
            "alias" => Some(FilterType::Alias(value)),
            "payload" => parse_payload_filter(&value),
            p if p.starts_with('p') && p.len() == 2 => {
                let packet_index = p[1..].parse::<usize>().ok()?.saturating_sub(1);
                Some(FilterType::PacketPID(packet_index, value))
            }
            "pid" => value.parse().ok().map(FilterType::PID),
            "type" => PacketType::from_str(&value).ok().map(FilterType::Type),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_payload_filter(value: &str) -> Option<FilterType> {
    if value.starts_with('>') {
        value[1..]
            .trim()
            .parse()
            .ok()
            .map(PayloadFilter::GreaterThan)
    } else if value.starts_with('<') {
        value[1..].trim().parse().ok().map(PayloadFilter::LessThan)
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
            (PacketType::PAT, PIDTable::ProgramAssociation) => true,
            (_, PIDTable::PID(pid)) => {
                let is_pmt = ctx.pmt_pids.contains(&PIDTable::PID(*pid));
                let is_es = ctx.es_pids.contains(&PIDTable::PID(*pid));
                let is_pcr = ctx.pcr_pids.contains(&PIDTable::PID(*pid));

                match (packet_type, is_pmt, is_es, is_pcr) {
                    (PacketType::PMT, true, _, _) => true,
                    (PacketType::PCRES, _, true, true) => true,
                    (PacketType::ES, _, true, false) => true,
                    (PacketType::PCR, _, false, true) => true,
                    _ => false,
                }
            }
            _ => false,
        })
}
