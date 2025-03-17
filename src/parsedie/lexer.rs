use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::Chars;

use super::tokens::Token;

#[derive(Debug)]
pub enum LexerError {
    ParseIntError(ParseIntError),
    InvalidCharacter(char),
}

pub struct Lexer {
    current_expr: String,
    position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            current_expr: String::new(),
            position: 0,
        }
    }

    pub fn set_expression(&mut self, new_expr: &str) {
        self.current_expr = new_expr.to_string();
        self.position = 0;
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.current_expr.len() {
            return Some(Ok(Token::EOF));
        }

        let chars: Vec<char> = self.current_expr.chars().collect();
        let next_char = chars[self.position];
        self.position += 1;

        match next_char {
            '0'..='9' => {
                let mut number = next_char.to_string();
                while self.position < chars.len() {
                    let peek_char = chars[self.position];
                    if peek_char.is_numeric() {
                        number.push(peek_char);
                        self.position += 1;
                    } else if peek_char == '(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(number.parse::<i128>()
                    .map(Token::Num)
                    .map_err(LexerError::ParseIntError))
            },
            '+' => Some(Ok(Token::Add)),
            '-' => Some(Ok(Token::Subtract)),
            '*' => Some(Ok(Token::Multiply)),
            '/' => Some(Ok(Token::Divide)),
            '^' => Some(Ok(Token::Caret)),
            '(' => Some(Ok(Token::LeftParen)),
            ')' => Some(Ok(Token::RightParen)),
            'd' => Some(Ok(Token::Die)),
            c => Some(Err(LexerError::InvalidCharacter(c))),
        }
    }
}
