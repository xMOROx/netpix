use super::types::StunFilterContext;
use crate::filter_system::{FilterExpression, ParseError};

pub fn parse_filter(filter: &str) -> Result<FilterExpression<StunFilterContext>, ParseError> {
    let mut tokens = filter.split_whitespace();
    let mut expressions = Vec::new();
    let mut current_operator = None;

    while let Some(token) = tokens.next() {
        match token.to_lowercase().as_str() {
            "and" => {
                current_operator = Some("and");
            }
            "or" => {
                current_operator = Some("or");
            }
            _ => {
                let (filter_type, value) = token
                    .split_once(':')
                    .ok_or_else(|| ParseError::InvalidSyntax("Missing colon in filter".to_string()))?;

                let expr = match filter_type.to_lowercase().as_str() {
                    "source" => FilterExpression::Source(value.to_string()),
                    "dest" => FilterExpression::Destination(value.to_string()),
                    "type" => FilterExpression::Type(value.to_string()),
                    unknown => Err(ParseError::InvalidSyntax(format!(
                        "Unknown filter type: '{}'.\nAvailable filters:\n\
                         - source: Source IP filter\n\
                         - dest: Destination IP filter\n\
                         - type: STUN message type filter",
                        unknown
                    )))?,
                };

                expressions.push((current_operator.take(), expr));
            }
        }
    }

    if expressions.is_empty() {
        return Err(ParseError::InvalidSyntax("Empty filter".to_string()));
    }

    let mut result = expressions[0].1.clone();
    for (op, expr) in expressions.iter().skip(1) {
        match op {
            Some("and") => result = FilterExpression::And(Box::new(result), Box::new(expr.clone())),
            Some("or") => result = FilterExpression::Or(Box::new(result), Box::new(expr.clone())),
            _ => result = FilterExpression::And(Box::new(result), Box::new(expr.clone())),
        }
    }

    Ok(result)
} 