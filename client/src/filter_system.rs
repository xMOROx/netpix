use std::collections::VecDeque;
use crate::filter_system;

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

/// Trait for parsing filter values
pub trait FilterParser: Sized {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError>;
}

/// Result type for parsing filter expressions
pub trait FilterExpression<'a> {
    type Context;
    fn matches(&self, ctx: &Self::Context) -> bool;
}

/// Trait for combining filters
pub trait FilterCombinator<'a>: Sized {
    fn and(left: Self, right: Self) -> Self;
    fn or(left: Self, right: Self) -> Self;
}

/// Common trait for implementing filter parsing functionality
pub trait CommonFilterParser: FilterParser + Sized {
    /// Create a NOT filter
    fn not(expr: Self) -> Self;
}

/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    And,
    Or,
    Not,
    Colon,
    Filter(String),
}

/// Error types for the parser
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnmatchedParenthesis,
    MissingOperand,
    InvalidToken(String),
    UnexpectedToken,
    InvalidSyntax(String),
    EmptyExpression,
}

/// Common comparison operators for numeric filters
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonFilter<T> {
    GreaterThan(T),
    GreaterOrEqualThan(T),
    LessThan(T),
    LessOrEqualThan(T),
    Equals(String),
}

/// Lexer for filter expressions
pub struct Lexer {
    tokens: VecDeque<Token>,
}

pub fn parse_expression<'a, F, C>(
    lexer: &mut Lexer,
    precedence: u8,
    parse_primary: impl Fn(&mut Lexer) -> ParseResult<F>,
) -> ParseResult<F>
where
    F: FilterExpression<'a, Context = C> + FilterCombinator<'a>,
{
    if lexer.is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut left = parse_primary(lexer)?;

    while let Some(token) = lexer.peek_token() {
        let current_precedence = get_operator_precedence(token);
        if current_precedence < precedence {
            break;
        }

        match token {
            Token::And | Token::Or => {
                let op = lexer.next_token().ok_or(ParseError::UnexpectedToken)?;
                let right = parse_primary(lexer).map_err(|_e| ParseError::MissingOperand)?;

                left = match op {
                    Token::And => F::and(left, right),
                    Token::Or => F::or(left, right),
                    _ => unreachable!(),
                };
            }
            _ => break,
        }
    }

    Ok(left)
}

pub fn get_operator_precedence(token: &Token) -> u8 {
    match token {
        Token::Or => 1,
        Token::And => 2,
        _ => 0,
    }
}

pub fn validate_filter_syntax(input: &str) -> ParseResult<()> {
    if input.trim().is_empty() {
        return Err(ParseError::EmptyExpression);
    }

    let mut paren_count = 0;
    for c in input.chars() {
        match c {
            '(' => paren_count += 1,
            ')' => {
                paren_count -= 1;
                if paren_count < 0 {
                    return Err(ParseError::UnmatchedParenthesis);
                }
            }
            _ => {}
        }
    }
    if paren_count != 0 {
        return Err(ParseError::UnmatchedParenthesis);
    }

    if input.to_lowercase().contains("and and") || input.to_lowercase().contains("or or") {
        return Err(ParseError::InvalidSyntax("Consecutive operators".into()));
    }

    Ok(())
}

pub fn parse_filter<F>(filter: &str) -> Result<F, ParseError>
where
    F: CommonFilterParser + FilterCombinator<'static> + FilterExpression<'static>
{
    validate_filter_syntax(filter)?;
    let mut lexer = Lexer::new(filter);
    parse_expression(&mut lexer, 0, primary_parser::<F>).map_err(|e| e.into())
}

fn primary_parser<F>(lexer: &mut Lexer) -> ParseResult<F>
where
    F:  CommonFilterParser + FilterCombinator<'static> + FilterExpression<'static>
{
    let token = lexer
        .next_token()
        .ok_or(ParseError::InvalidToken("Empty token".into()))?;

    match token {
        Token::OpenParen => {
            let expr = parse_expression(lexer, 0, primary_parser::<F>)?;
            match lexer.next_token() {
                Some(Token::CloseParen) => Ok(expr),
                Some(_other) => Err(ParseError::UnmatchedParenthesis),
                None => Err(ParseError::UnmatchedParenthesis),
            }
        }
        Token::Not => {
            let expr = primary_parser::<F>(lexer)?;
            Ok(F::not(expr))
        }
        Token::Filter(prefix) => {
            if lexer.next_token() != Some(Token::Colon) {
                return Err(ParseError::InvalidSyntax(format!(
                    "Missing colon after '{}' filter",
                    prefix
                )));
            }

            match lexer.next_token() {
                Some(Token::Filter(value)) => F::parse_filter_value(&prefix, &value),
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

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut tokens = VecDeque::new();
        let mut current = String::new();

        for c in input.chars() {
            match c {
                '(' => {
                    Self::push_if_not_empty(&mut tokens, &mut current);
                    tokens.push_back(Token::OpenParen);
                }
                ')' => {
                    Self::push_if_not_empty(&mut tokens, &mut current);
                    tokens.push_back(Token::CloseParen);
                }
                ':' => {
                    Self::push_if_not_empty(&mut tokens, &mut current);
                    tokens.push_back(Token::Colon);
                }
                ' ' => {
                    if !current.is_empty() {
                        match current.to_lowercase().as_str() {
                            "and" => tokens.push_back(Token::And),
                            "or" => tokens.push_back(Token::Or),
                            "not" => tokens.push_back(Token::Not),
                            _ => tokens.push_back(Token::Filter(current.clone())),
                        }
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
        Self::push_if_not_empty(&mut tokens, &mut current);
        Self { tokens }
    }

    fn push_if_not_empty(tokens: &mut VecDeque<Token>, current: &mut String) {
        if !current.is_empty() {
            tokens.push_back(Token::Filter(current.clone()));
            current.clear();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if let Some(token) = self.tokens.front() {
            Some(token)
        } else {
            None
        }
    }

    pub fn remaining_tokens(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnmatchedParenthesis => write!(f, "Unmatched parenthesis"),
            ParseError::MissingOperand => write!(f, "Missing operand"),
            ParseError::InvalidToken(t) => write!(f, "{}", t),
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::InvalidSyntax(s) => write!(f, "{}", s),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
        }
    }
}

impl<T: std::str::FromStr> ComparisonFilter<T> {
    pub fn parse(value: &str) -> Option<Self> {
        if let Some(stripped) = value.strip_prefix('>') {
            if let Some(stripped) = stripped.strip_prefix('=') {
                stripped.trim().parse().ok().map(Self::GreaterOrEqualThan)
            } else {
                stripped.trim().parse().ok().map(Self::GreaterThan)
            }
        } else if let Some(stripped) = value.strip_prefix("<=") {
            stripped.trim().parse().ok().map(Self::LessOrEqualThan)
        } else if let Some(stripped) = value.strip_prefix('<') {
            stripped.trim().parse().ok().map(Self::LessThan)
        } else {
            Some(Self::Equals(value.to_string()))
        }
    }
}

#[macro_export]
macro_rules! declare_filter_type {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident$(($($type:ty),+))?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant$(($($type),+))?,
            )*
            And(Box<$name>, Box<$name>),
            Or(Box<$name>, Box<$name>),
            Not(Box<$name>),
        }

        impl<'a> $crate::filter_system::FilterCombinator<'a> for $name {
            fn and(left: Self, right: Self) -> Self {
                Self::And(Box::new(left), Box::new(right))
            }

            fn or(left: Self, right: Self) -> Self {
                Self::Or(Box::new(left), Box::new(right))
            }
        }
    };
}
