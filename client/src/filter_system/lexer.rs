use super::types::Token;
use std::collections::VecDeque;

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
