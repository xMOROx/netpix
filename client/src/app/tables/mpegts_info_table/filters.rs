//! MPEG-TS Information Table Filtering
//!
//! This module provides filtering capabilities for the MPEG-TS information table.
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
//! - `alias:value` - Filter by stream alias containing the value
//! - `pid:number` - Filter by PID value
//! - `type:value` - Filter by packet type (PAT, PMT)

use super::types::{MpegTsInfo, RowKey};
use crate::{
    declare_filter_type,
    filter_system::{self, *},
};
use netpix_common::mpegts::header::PIDTable;
use std::str::FromStr;

pub struct FilterContext<'a> {
    pub key: &'a RowKey,
    pub info: &'a MpegTsInfo,
}

#[derive(Debug)]
pub enum PacketType {
    Pat,
    Pmt,
}

impl FromStr for PacketType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PAT" => Ok(PacketType::Pat),
            "PMT" => Ok(PacketType::Pmt),
            _ => Err(()),
        }
    }
}

declare_filter_type! {
    pub enum FilterType {
        Alias(String),
        Pid(u16),
        Type(PacketType)
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
            FilterType::Alias(value) => ctx.key.alias.to_lowercase().contains(value),
            FilterType::Pid(value) => match ctx.key.pid {
                PIDTable::PID(pid) => pid == *value,
                PIDTable::ProgramAssociation => *value == 0,
                _ => false,
            },
            FilterType::Type(packet_type) => match packet_type {
                PacketType::Pat => matches!(ctx.key.pid, PIDTable::ProgramAssociation),
                PacketType::Pmt => matches!(ctx.key.pid, PIDTable::PID(_)),
            },
            FilterType::And(left, right) => left.matches(ctx) && right.matches(ctx),
            FilterType::Or(left, right) => left.matches(ctx) || right.matches(ctx),
            FilterType::Not(filter) => !filter.matches(ctx),
        }
    }
}

impl FilterParser for FilterType {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError> {
        match prefix.trim() {
            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Alias filter cannot be empty (e.g. alias:stream_a)".into(),
                    ))
                }),

            "pid" => value
                .parse()
                .map(FilterType::Pid)
                .map_err(|_| ParseError::InvalidSyntax("Invalid PID number".into())),

            "type" => PacketType::from_str(value)
                .map(FilterType::Type)
                .map_err(|_| {
                    ParseError::InvalidSyntax(
                        "Invalid packet type. Must be one of:\n\
                         - type:PAT (Program Association Table)\n\
                         - type:PMT (Program Map Table)"
                            .into(),
                    )
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - alias: Stream alias filter\n\
                 - pid: PID value filter\n\
                 - type: Packet type filter (PAT, PMT)",
                unknown
            ))),
        }
    }
}
