//! DieSir is a library for parsing and evaluating dice expressions as well as basic math expressions.
//! 
//! This crate provides functionality to parse and evaluate
//! dice rolling expressions like "2d6 + 3"

mod parsedie;

pub use parsedie::{
    ast::{self, EvalResult},
    parser::{ParseError, Parser},
};

/// Evaluates a dice expression string and returns the result
/// 
/// # Arguments
/// 
/// * `expr` - A string containing the dice expression to evaluate
/// 
/// # Examples
/// 
/// ```
/// use die_sir::evaluate;
/// 
/// let result = evaluate("2d6 + 3".to_string());
/// match result {
///     Ok(val) => println!("Result: {}", val),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn evaluate(expr: String) -> Result<EvalResult, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut die_parser = Parser::new(&expr)?;
    let ast = die_parser.parse()?;
    Ok(ast::eval(ast)?)
} 