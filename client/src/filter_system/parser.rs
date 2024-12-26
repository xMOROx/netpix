use super::{
    error::ParseError,
    lexer::Lexer,
    traits::{CommonFilterParser, FilterCombinator, FilterExpression},
    types::{get_operator_precedence, Token},
};

pub type ParseResult<T> = Result<T, ParseError>;

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
    F: CommonFilterParser + FilterCombinator<'static> + FilterExpression<'static>,
{
    validate_filter_syntax(filter)?;
    let mut lexer = Lexer::new(filter);
    parse_expression(&mut lexer, 0, primary_parser::<F>)
}

fn primary_parser<F>(lexer: &mut Lexer) -> ParseResult<F>
where
    F: CommonFilterParser + FilterCombinator<'static> + FilterExpression<'static>,
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
