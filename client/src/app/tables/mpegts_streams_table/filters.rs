//! MPEG-TS Streams Filtering
//!
//! This module provides a filtering system for MPEG-TS streams with support for complex queries.
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
//! - `alias:value` - Matches stream alias containing the value
//! - `program:number` - Matches program number
//! - `source:value` - Matches source IP address containing the value
//! - `dest:value` - Matches destination IP address containing the value
//!
//! ## Numeric Filters
//! - `fragments:number` - Filter by number of fragments
//! - `duration:number` - Filter by stream duration (in seconds)
//! - `bitrate:number` - Filter by mean bitrate (in kbps)
//! - `fragmentrate:number` - Filter by fragment rate (fragments/sec)

use crate::{
    declare_filter_type,
    filter_system::{self, *},
    streams::{mpegts_stream::substream::MpegtsSubStream, stream_statistics::StreamStatistics},
};

pub struct FilterContext<'a> {
    pub stream: &'a MpegtsSubStream,
}

declare_filter_type! {
    pub enum FilterType {
        Alias(String),
        Program(u16),
        Source(String),
        Destination(String),
        Fragments(ComparisonFilter<usize>),
        Duration(ComparisonFilter<f64>),
        Bitrate(ComparisonFilter<f64>),
        FragmentRate(ComparisonFilter<f64>)
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
            FilterType::Alias(value) => ctx
                .stream
                .aliases
                .stream_alias
                .to_lowercase()
                .contains(value),
            FilterType::Program(value) => ctx.stream.program_number == *value,
            FilterType::Source(value) => ctx
                .stream
                .packet_association_table
                .source_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Destination(value) => ctx
                .stream
                .packet_association_table
                .destination_addr
                .to_string()
                .to_lowercase()
                .contains(value),
            FilterType::Fragments(filter) => {
                let count = ctx.stream.packets.len();
                match filter {
                    ComparisonFilter::GreaterThan(size) => count > *size,
                    ComparisonFilter::GreaterOrEqualThan(size) => count >= *size,
                    ComparisonFilter::LessThan(size) => count < *size,
                    ComparisonFilter::LessOrEqualThan(size) => count <= *size,
                    ComparisonFilter::Equals(value) => count.to_string() == *value,
                }
            }
            FilterType::Duration(filter) => {
                let duration = ctx.stream.get_duration().as_secs_f64();
                match filter {
                    ComparisonFilter::GreaterThan(val) => duration > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => duration >= *val,
                    ComparisonFilter::LessThan(val) => duration < *val,
                    ComparisonFilter::LessOrEqualThan(val) => duration <= *val,
                    ComparisonFilter::Equals(value) => duration.to_string() == *value,
                }
            }
            FilterType::Bitrate(filter) => {
                let bitrate = ctx.stream.get_mean_frame_bitrate() / 1000.0;
                match filter {
                    ComparisonFilter::GreaterThan(val) => bitrate > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => bitrate >= *val,
                    ComparisonFilter::LessThan(val) => bitrate < *val,
                    ComparisonFilter::LessOrEqualThan(val) => bitrate <= *val,
                    ComparisonFilter::Equals(value) => bitrate.to_string() == *value,
                }
            }
            FilterType::FragmentRate(filter) => {
                let rate = ctx.stream.get_mean_packet_rate();
                match filter {
                    ComparisonFilter::GreaterThan(val) => rate > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => rate >= *val,
                    ComparisonFilter::LessThan(val) => rate < *val,
                    ComparisonFilter::LessOrEqualThan(val) => rate <= *val,
                    ComparisonFilter::Equals(value) => rate.to_string() == *value,
                }
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
            "alias" => (!value.is_empty())
                .then_some(Ok(FilterType::Alias(value.to_lowercase())))
                .unwrap_or_else(|| {
                    Err(ParseError::InvalidSyntax(
                        "Alias filter cannot be empty (e.g. alias:stream_a)".into(),
                    ))
                }),

            "program" => value
                .parse()
                .map(FilterType::Program)
                .map_err(|_| ParseError::InvalidSyntax("Invalid program number".into())),

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

            "fragments" => ComparisonFilter::parse(value)
                .map(FilterType::Fragments)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid fragments filter format (e.g. fragments:>100)".into(),
                    )
                }),

            "duration" => ComparisonFilter::parse(value)
                .map(FilterType::Duration)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid duration filter format (e.g. duration:>10.5)".into(),
                    )
                }),

            "bitrate" => ComparisonFilter::parse(value)
                .map(FilterType::Bitrate)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid bitrate filter format (e.g. bitrate:>1000)".into(),
                    )
                }),

            "fragmentrate" => ComparisonFilter::parse(value)
                .map(FilterType::FragmentRate)
                .ok_or_else(|| {
                    ParseError::InvalidSyntax(
                        "Invalid fragment rate filter format (e.g. fragmentrate:>30)".into(),
                    )
                }),

            unknown => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter type: '{}'.\nAvailable filters:\n\
                 - alias: Stream alias filter\n\
                 - program: Program number filter\n\
                 - source: Source IP filter\n\
                 - dest: Destination IP filter\n\
                 - fragments: Number of fragments filter\n\
                 - duration: Stream duration filter (seconds)\n\
                 - bitrate: Mean bitrate filter (kbps)\n\
                 - fragmentrate: Fragment rate filter (per second)",
                unknown
            ))),
        }
    }
}
