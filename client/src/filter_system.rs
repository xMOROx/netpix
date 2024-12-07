use std::collections::VecDeque;

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

pub struct Lexer {
    tokens: VecDeque<Token>,
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
        self.tokens.front()
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnmatchedParenthesis,
    MissingOperand,
    InvalidToken,
    UnexpectedToken,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait FilterExpression<'a> {
    type Context;
    fn matches(&self, ctx: &Self::Context) -> bool;
}

pub fn parse_expression<'a, F, C>(
    lexer: &mut Lexer,
    precedence: u8,
    parse_primary: impl Fn(&mut Lexer) -> ParseResult<F>,
) -> ParseResult<F>
where
    F: FilterExpression<'a, Context = C> + FilterCombinator<'a>,
{
    let mut left = parse_primary(lexer)?;

    while let Some(token) = lexer.peek_token() {
        let current_precedence = get_operator_precedence(token);
        if current_precedence < precedence {
            break;
        }

        match token {
            Token::And | Token::Or => {
                let op = lexer.next_token().ok_or(ParseError::UnexpectedToken)?;
                let right = parse_primary(lexer).map_err(|_| ParseError::MissingOperand)?;
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

pub trait FilterCombinator<'a>: Sized {
    fn and(left: Self, right: Self) -> Self;
    fn or(left: Self, right: Self) -> Self;
}

pub fn get_operator_precedence(token: &Token) -> u8 {
    match token {
        Token::Or => 1,
        Token::And => 2,
        _ => 0,
    }
}
