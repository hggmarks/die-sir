use std::fmt;

use super::{tokens::{Token, OperPrec}, lexer::{Lexer, LexerError}, ast::Node};

pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
}

impl Parser {
    pub fn new() -> Self {
        let mut lexer = Lexer::new();
        Parser { 
            lexer,
            curr_token: Token::EOF, // Initial token
        }
    }

    /// Parse a new expression using the same Parser instance
    pub fn parse(&mut self, expr: &str) -> Result<Node, ParseError> {
        // Reset lexer with new expression
        self.lexer.set_expression(expr);
        
        // Get first token
        self.curr_token = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(ParseError::from(e)),
            None => return Err(ParseError::InvalidOperator("Invalid Character".into())),
        };

        // Parse the expression
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(ParseError::from(e)),
            None => return Err(ParseError::InvalidOperator(
                "Invalid character".into())),
        };
        self.curr_token = next_token;
        Ok(())
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.curr_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.curr_token
            )))
        }
    }

    fn parse_number (&mut self) -> Result<Node, ParseError> {
        let token = self.curr_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }

            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }

            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.curr_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.curr_token.get_oper_prec() {
            if self.curr_token == Token::EOF {
                break;
            }

            let right_expr = self.convert_token_to_node(left_expr.clone())?;

            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.curr_token {
            Token::Add => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Subtract => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::AddSub)?;

                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;

                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Divide => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Caret => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::Power)?;

                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }

            Token::Die => {
                self.get_next_token()?;

                let right_expr = self.generate_ast(OperPrec::DieRoll)?;

                Ok(Node::Die(Box::new(left_expr), Box::new(right_expr)))
            }

            _ => Err(ParseError::InvalidOperator(format!("Please, enter a valid operator {:?}", self.curr_token))),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
    NumberOverflow(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnableToParse(msg) => write!(f, "Unable to parse: {}", msg),
            ParseError::InvalidOperator(msg) => write!(f, "Invalid operator: {}", msg),
            ParseError::NumberOverflow(msg) => write!(f, "Number overflow: {}", msg),
        }
    }
}

impl From<LexerError> for ParseError {
    fn from(error: LexerError) -> Self {
        match error {
            LexerError::ParseIntError(e) => ParseError::NumberOverflow(format!("Number is too large: {}", e)),
            LexerError::InvalidCharacter(c) => ParseError::InvalidOperator(format!("Invalid character: {}", c)),
        }
    }
}

impl From<Box<dyn std::error::Error>> for ParseError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse(error.to_string())
    }
}

impl std::error::Error for ParseError {}
