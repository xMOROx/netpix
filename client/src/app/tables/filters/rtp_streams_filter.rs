//! RTP Streams Filtering

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
    streams::rtpStream::RtpStream,
};

pub struct FilterContext<'a> {
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
    pub alias: &'a str,
    pub stream: &'a RtpStream,
}

const KILO: f64 = 1000.0;

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Alias(String),
        Cname(String),
        MeanBitrate(ComparisonFilter<f64>),
        PacketCount(ComparisonFilter<u32>)
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
            FilterType::Source(value) => ctx.source_addr.to_lowercase().contains(value),
            FilterType::Destination(value) => ctx.destination_addr.to_lowercase().contains(value),
            FilterType::Alias(value) => ctx.alias.to_lowercase().contains(value),
            FilterType::Cname(value) => ctx.stream.cname
                .as_ref()
                .map(|c| c.to_lowercase().contains(value))
                .unwrap_or(false),
            FilterType::MeanBitrate(filter) => {
                let bitrate = ctx.stream.get_mean_bitrate() / KILO;
                match filter {
                    ComparisonFilter::GreaterThan(val) => bitrate > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => bitrate >= *val,
                    ComparisonFilter::LessThan(val) => bitrate < *val,
                    ComparisonFilter::LessOrEqualThan(val) => bitrate <= *val,
                    ComparisonFilter::Equals(value) => bitrate.to_string() == *value,
                }
            }
            FilterType::PacketCount(filter) => {
                let count = ctx.stream.rtp_packets.len() as u32;
                match filter {
                    ComparisonFilter::GreaterThan(val) => count > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => count >= *val,
                    ComparisonFilter::LessThan(val) => count < *val,
                    ComparisonFilter::LessOrEqualThan(val) => count <= *val,
                    ComparisonFilter::Equals(value) => count.to_string() == *value,
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
            "source" => Ok(FilterType::Source(value.to_lowercase())),
            "dest" => Ok(FilterType::Destination(value.to_lowercase())),
            "alias" => Ok(FilterType::Alias(value.to_lowercase())),
            "cname" => Ok(FilterType::Cname(value.to_lowercase())),
            "bitrate" => ComparisonFilter::parse(value)
                .map(FilterType::MeanBitrate)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid bitrate filter".into())),
            "packets" => ComparisonFilter::parse(value)
                .map(FilterType::PacketCount)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid packet count filter".into())),
            _ => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter: {}. Use: source, dest, alias, cname, bitrate, packets",
                prefix
            ))),
        }
    }
}
