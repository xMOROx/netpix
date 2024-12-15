//! RTP Stream Filtering
//!
//! This module provides a filtering system for RTP streams with support for complex queries.
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
//! - `alias:value` - Matches stream alias containing the value
//! - `cname:value` - Matches CNAME containing the value
//! - `mean_bitrate:comparison` - Matches mean bitrate using comparison operators
//! - `mean_rtp_bitrate:comparison` - Matches mean RTP bitrate using comparison operators
//! - `packet_count:comparison` - Matches packet count using comparison operators
//!
//! # Examples
//!
//! Simple filters:
//! - `source:192.168` - Matches streams from addresses containing "192.168"
//! - `alias:test` - Matches streams with alias containing "test"
//! - `mean_bitrate:>1000` - Matches streams with mean bitrate greater than 1000
//! - `packet_count:<=100` - Matches streams with packet count less or equal to 100
//!
//! Complex filters:
//! - `source:10.0.0 AND NOT alias:test` - Streams from specific network without test alias
//! - `dest:192.168 OR dest:10.0.0` - Streams going to specific networks
//! - `(mean_bitrate:>1000 OR packet_count:>1000) AND NOT source:192.168` - Large streams excluding specific network

use crate::app::rtp_streams_table::RtpStreamFilterContext;
use crate::filter_system::{
    CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
};
use crate::{declare_filter_type, filter_system};

const KILO: f64 = 1000.0;

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Alias(String),
        Cname(String),
        MeanBitrate(ComparisonFilter<f64>),
        MeanRtpBitrate(ComparisonFilter<f64>),
        PacketCount(ComparisonFilter<u32>),
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
    type Context = RtpStreamFilterContext<'a>;

    fn matches(&self, ctx: &Self::Context) -> bool {
        match self {
            FilterType::Source(value) => ctx.source_addr.to_lowercase().contains(value),
            FilterType::Destination(value) => ctx.destination_addr.to_lowercase().contains(value),
            FilterType::Alias(value) => ctx.alias.to_lowercase().contains(value),
            FilterType::Cname(value) => ctx
                .stream
                .clone()
                .cname
                .filter(|cname| cname.to_lowercase().contains(value))
                .is_some(),
            FilterType::MeanBitrate(filter) => match filter {
                ComparisonFilter::Equals(value) => ctx.stream.get_mean_bitrate() / KILO == (*value).parse().unwrap_or(0.0),
                ComparisonFilter::GreaterThan(value) => ctx.stream.get_mean_bitrate() / KILO > *value,
                ComparisonFilter::GreaterOrEqualThan(value) => ctx.stream.get_mean_bitrate() / KILO >= *value,
                ComparisonFilter::LessThan(value) => ctx.stream.get_mean_bitrate() / KILO < *value,
                ComparisonFilter::LessOrEqualThan(value) => ctx.stream.get_mean_bitrate() / KILO <= *value,
            },
            FilterType::MeanRtpBitrate(filter) => match filter {
                ComparisonFilter::Equals(value) => ctx.stream.get_mean_rtp_bitrate() / KILO == (*value).parse().unwrap_or(0.0),
                ComparisonFilter::GreaterThan(value) => ctx.stream.get_mean_rtp_bitrate() / KILO > *value,
                ComparisonFilter::GreaterOrEqualThan(value) => ctx.stream.get_mean_rtp_bitrate() / KILO >= *value,
                ComparisonFilter::LessThan(value) => ctx.stream.get_mean_rtp_bitrate() / KILO < *value,
                ComparisonFilter::LessOrEqualThan(value) => ctx.stream.get_mean_rtp_bitrate() / KILO <= *value,

            },
            FilterType::PacketCount(filter) => match filter {
                ComparisonFilter::Equals(value) => ctx.stream.rtp_packets.len() == (*value).parse().unwrap_or(0),
                ComparisonFilter::GreaterThan(value) => ctx.stream.rtp_packets.len() > *value as usize,
                ComparisonFilter::GreaterOrEqualThan(value) => ctx.stream.rtp_packets.len() >= *value as usize,
                ComparisonFilter::LessThan(value) => ctx.stream.rtp_packets.len() < *value as usize,
                ComparisonFilter::LessOrEqualThan(value) => ctx.stream.rtp_packets.len() <= *value as usize
                
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

            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Alias filter cannot be empty (e.g. alias:stream_a)".into(),
                    ))
                }),

            "cname" => (!value.is_empty())
                .then_some(Ok(FilterType::Cname(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "CNAME filter cannot be empty (e.g. cname:stream_a)".into(),
                    ))
                }),

            "mean_bitrate" => ComparisonFilter::parse(value)
                .map(FilterType::MeanBitrate)
                .ok_or(ParseError::InvalidSyntax(
                    "Invalid mean bitrate filter (e.g. mean_bitrate:>1000)".into(),
                )),

            "mean_rtp_bitrate" => ComparisonFilter::parse(value)
                .map(FilterType::MeanRtpBitrate)
                .ok_or(ParseError::InvalidSyntax(
                    "Invalid mean RTP bitrate filter (e.g. mean_rtp_bitrate:>1000)".into(),
                )),

            "packet_count" => ComparisonFilter::parse(value)
                .map(FilterType::PacketCount)
                .ok_or(ParseError::InvalidSyntax(
                    "Invalid packet count filter (e.g. packet_count:>1000)".into(),
                )),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - source: Source IP filter (e.g. source:192.168.1.1)\n\
                 - dest: Destination IP filter (e.g. dest:192.168.1.1)\n\
                 - alias: Stream alias filter (e.g. alias:test_stream)\n\
                 - cname: CNAME filter (e.g. cname:test_stream)\n\
                 - mean_bitrate: Mean bitrate filter (e.g. mean_bitrate:>1000)\n\
                 - mean_rtp_bitrate: Mean RTP bitrate filter (e.g. mean_rtp_bitrate:>1000)\n\
                 - packet_count: Packet count filter (e.g. packet_count:>1000)\n",
                unknown
            ))),
        }
    }
}
