#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnmatchedParenthesis,
    MissingOperand,
    InvalidToken(String),
    UnexpectedToken,
    InvalidSyntax(String),
    EmptyExpression,
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

impl std::error::Error for ParseError {}
