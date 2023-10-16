use std::iter::Peekable;
use std::str::Chars;use

use super::tokens::Token;

pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Lexer { expr: new_expr.chars().peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
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
                Some(Token::Num(number.parse::<i128>().unwrap()))
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('d') => Some(Token::Die),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}
