use crate::filter_system::{
    parse_expression, validate_filter_syntax, FilterCombinator, FilterExpression, Lexer,
    ParseError, ParseResult, Token,
};
use netpix_common::packet::{Packet, SessionProtocol};
use std::str::FromStr;

pub struct FilterContext<'a> {
    pub packet: &'a Packet,
}

pub enum FilterType {
    Source(String),
    Destination(String),
    Protocol(String),
    Length(LengthFilter),
    Type(SessionProtocol),
    And(Box<FilterType>, Box<FilterType>),
    Or(Box<FilterType>, Box<FilterType>),
    Not(Box<FilterType>),
}

pub enum LengthFilter {
    GreaterOrEqualThan(usize),
    GreaterThan(usize),
    LessOrEqualThan(usize),
    LessThan(usize),
    Equals(String),
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
                // Check both transport and session protocols
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
            FilterType::Length(length_filter) => {
                let length = ctx.packet.length;
                match length_filter {
                    LengthFilter::GreaterThan(size) => length > *size as u32,
                    LengthFilter::GreaterOrEqualThan(size) => length >= *size as u32,
                    LengthFilter::LessThan(size) => length < *size as u32,
                    LengthFilter::LessOrEqualThan(size) => length <= *size as u32,
                    LengthFilter::Equals(value) => length.to_string() == *value,
                }
            }
            FilterType::Type(protocol) => ctx.packet.session_protocol == *protocol,
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

pub fn parse_filter(filter: &str) -> Result<FilterType, ParseError> {
    validate_filter_syntax(filter)?;

    let mut lexer = Lexer::new(filter);
    parse_expression(&mut lexer, 0, parse_primary).map_err(|e| e.into())
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
                Some(Token::Filter(value)) => parse_filter_with_value(&prefix, &value),
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

fn parse_filter_with_value(prefix: &str, value: &str) -> Result<FilterType, ParseError> {
    match prefix.trim() {
        "source" => Ok(FilterType::Source(value.to_lowercase())),
        "dest" => Ok(FilterType::Destination(value.to_lowercase())),
        "proto" | "protocol" => Ok(FilterType::Protocol(value.to_lowercase())), // Allow both "proto" and "protocol"
        "length" => parse_length_filter(value).ok_or_else(|| {
            ParseError::InvalidSyntax(format!(
                "Invalid length filter format. Expected number or comparison (e.g. >100, <=1500), got '{}'",
                value
            ))
        }),
        "type" => SessionProtocol::from_str(value)
            .map(FilterType::Type)
            .map_err(|_| {
                ParseError::InvalidSyntax(format!(
                    "Invalid protocol type: {}",
                    value
                ))
            }),
        _ => Err(ParseError::InvalidSyntax(format!(
            "Unknown filter type: {}",
            prefix
        ))),
    }
}

fn parse_length_filter(value: &str) -> Option<FilterType> {
    let result = if let Some(stripped) = value.strip_prefix('>') {
        stripped.trim().parse().ok().map(LengthFilter::GreaterThan)
    } else if let Some(stripped) = value.strip_prefix(">=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(LengthFilter::GreaterOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix("<=") {
        stripped
            .trim()
            .parse()
            .ok()
            .map(LengthFilter::LessOrEqualThan)
    } else if let Some(stripped) = value.strip_prefix('<') {
        stripped.trim().parse().ok().map(LengthFilter::LessThan)
    } else {
        match value.parse::<usize>() {
            Ok(_) => Some(LengthFilter::Equals(value.to_string())),
            Err(_) => None,
        }
    };

    result.map(FilterType::Length)
}
