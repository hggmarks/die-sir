use std::iter::Peekable;
use std::str::Chars;
use std::num::ParseIntError;

use super::tokens::Token;

#[derive(Debug)]
pub enum LexerError {
    ParseIntError(ParseIntError),
    InvalidCharacter(char),
}

pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Lexer { expr: new_expr.chars().peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Result<Token, LexerError>> {
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(number.parse::<i128>()
                    .map(Token::Num)
                    .map_err(LexerError::ParseIntError))
            },
            Some('+') => Some(Ok(Token::Add)),
            Some('-') => Some(Ok(Token::Subtract)),
            Some('*') => Some(Ok(Token::Multiply)),
            Some('/') => Some(Ok(Token::Divide)),
            Some('^') => Some(Ok(Token::Caret)),
            Some('(') => Some(Ok(Token::LeftParen)),
            Some(')') => Some(Ok(Token::RightParen)),
            Some('d') => Some(Ok(Token::Die)),
            None => Some(Ok(Token::EOF)),
            Some(c) => Some(Err(LexerError::InvalidCharacter(c))),
        }
    }
}
