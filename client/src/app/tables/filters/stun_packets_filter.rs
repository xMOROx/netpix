//! STUN Packet Filtering

use crate::{
    declare_filter_type,
    filter_system::{
        CommonFilterParser, ComparisonFilter, FilterExpression, FilterParser, ParseError,
    },
};
use netpix_common::stun::StunPacket;

pub struct FilterContext<'a> {
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
    pub packet: &'a StunPacket,
}

declare_filter_type! {
    pub enum FilterType {
        Source(String),
        Destination(String),
        Type(String),
        Transaction(String),
        Length(ComparisonFilter<usize>)
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
            FilterType::Source(value) => {
                if value.is_empty() { return true; }
                ctx.source_addr.to_lowercase().contains(value)
            }
            FilterType::Destination(value) => {
                if value.is_empty() { return true; }
                ctx.destination_addr.to_lowercase().contains(value)
            }
            FilterType::Type(value) => {
                if value.is_empty() { return true; }
                ctx.packet.message_type.as_string().to_lowercase().contains(value)
            }
            FilterType::Transaction(value) => {
                if value.is_empty() { return true; }
                let tx_id = ctx.packet.transaction_id
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                tx_id.contains(value)
            }
            FilterType::Length(filter) => {
                let size = ctx.packet.message_length as usize;
                match filter {
                    ComparisonFilter::GreaterThan(val) => size > *val,
                    ComparisonFilter::GreaterOrEqualThan(val) => size >= *val,
                    ComparisonFilter::LessThan(val) => size < *val,
                    ComparisonFilter::LessOrEqualThan(val) => size <= *val,
                    ComparisonFilter::Equals(value) => size.to_string() == *value,
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
            "type" => Ok(FilterType::Type(value.to_lowercase())),
            "transaction" => Ok(FilterType::Transaction(value.to_lowercase())),
            "length" => ComparisonFilter::parse(value)
                .map(FilterType::Length)
                .ok_or_else(|| ParseError::InvalidSyntax("Invalid length filter".into())),
            _ => Err(ParseError::InvalidSyntax(format!(
                "Unknown filter: {}. Use: source, dest, type, transaction, length",
                prefix
            ))),
        }
    }
}
