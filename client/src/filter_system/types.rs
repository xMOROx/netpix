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

pub fn get_operator_precedence(token: &Token) -> u8 {
    match token {
        Token::Or => 1,
        Token::And => 2,
        _ => 0,
    }
}
