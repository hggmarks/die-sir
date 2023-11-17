use rand::Rng;
use rand::rngs::OsRng;
use std::error;


#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Die(Box<Node>, Box<Node>),
    Number(i128),
}


#[derive(Clone, Debug, PartialEq)]
pub enum EvalResult {
    Number(f64),
    DieResult(Vec<i128>),
}


impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            self::EvalResult::Number(e) => write!(f, "{}", e),
            self::EvalResult::DieResult(e) => write!(f, "{:?}", e),
        } 
    }
}


pub fn eval(expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
    match expr {
        Node::Number(i) => Ok(EvalResult::Number(i as f64)),
        Node::Add(expr1, expr2) => {

            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs, rhs) {
                (EvalResult::Number(num1), EvalResult::Number(num2)) => {
                    Ok(EvalResult::Number(num1 + num2))
                }
                _ => Err("Cannot Add DieResult to Number".into()),

            }
        }
        Node::Subtract(expr1, expr2) => {

            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs, rhs) {
                (EvalResult::Number(num1), EvalResult::Number(num2)) => {
                    Ok(EvalResult::Number(num1 - num2))
                }
                _ => Err("Cannot Subtract DieResult to Number".into()),
            }
        }
        Node::Multiply(expr1, expr2) => {
            
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs, rhs) {
                (EvalResult::Number(num1), EvalResult::Number(num2)) => {
                    Ok(EvalResult::Number(num1 * num2))
                }
                _ => Err("Cannot Multiply DieResult by Number".into()),
            }
        }
        Node::Divide(expr1, expr2) => {
            
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            match (lhs, rhs) {
                (EvalResult::Number(num1), EvalResult::Number(num2)) => {
                    Ok(EvalResult::Number(num1 / num2))
                }
                _ => Err("Cannot Divide DieResult by Number".into())
            }
        },
        Node::Negative(expr1) => {
            let value = eval(*expr1)?;

            match value {
                EvalResult::Number(val) => {
                    Ok(EvalResult::Number(-val))
                }
                _ => Err("DieResult cannot be negative".into())
            }
        }
        Node::Caret(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            match (lhs, rhs) {
                (EvalResult::Number(num1), EvalResult::Number(num2)) => {
                    Ok(EvalResult::Number(num1.powf(num2)))
                }
                _ => Err("Cannot use DieResult in power operation".into())
            }
        }
        Node::Die(expr1, expr2) => {
            let num_rows = eval(*expr1)?;
            let num_sides = eval(*expr2)?;
            let mut results: Vec<i128> = Vec::new();
            let mut rng = OsRng;

            if let (EvalResult::Number(num_rows), EvalResult::Number(num_sides)) = (num_rows, num_sides) {
                if num_rows == 0.0 || num_sides == 0.0 {
                    return Ok(EvalResult::DieResult(vec![0]));
                }

                for _ in 0..(num_rows as i128) {
                    results.push(rng.gen_range(1..=(num_sides as i128)));
                }

                Ok(EvalResult::DieResult(results))
            } else {
                Err("Die expressions must have numeric operands".into())
            }
            
        }   
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsedie::parser::Parser;


        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, EvalResult::Number(0.0))
    }
    #[test]
    fn test_expr2() {
        use crate::parsedie::parser::Parser;


        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, EvalResult::Number(3.75))
    }
}
