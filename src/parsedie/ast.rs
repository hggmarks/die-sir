use rand::Rng;
use rand::rngs::OsRng;
use std::{error, i128};


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

#[derive(Debug)]
pub struct EvalResult {
    result: f64,
    dice: Vec<i128>,
}

impl EvalResult {
    pub fn new_by_dice (rolls: Vec<i128>) -> Self {
        let sum: i128 = rolls.iter().sum();
        EvalResult { result: (sum as f64), dice: (rolls) }
    }

    pub fn new_by_number (value: f64) -> Self {
        EvalResult { result: (value), dice: (vec![0]) }
    }
}

impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Result: {}", self.result)?;

        if !self.dice.is_empty() && self.dice[0] != 0 {
            writeln!(f, "Dice rolls: {:?}", self.dice)
        } else {
            Ok(())
        }
    }
}

pub fn eval(expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
    match expr {
        Node::Number(i) => Ok(EvalResult::new_by_number(i as f64)),
        Node::Add(expr1, expr2) => {

            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs.result,rhs.result) {
                (num1, num2) => {
                    Ok(EvalResult::new_by_number(num1 + num2))
                }
            }
        }
        Node::Subtract(expr1, expr2) => {

            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs.result, rhs.result) {
                (num1, num2) => {
                    Ok(EvalResult::new_by_number(num1 - num2))
                }
            }
        }
        Node::Multiply(expr1, expr2) => {
            
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;
            
            match (lhs.result, rhs.result) {
                (num1, num2) => {
                    Ok(EvalResult::new_by_number(num1 * num2))
                }
            }
        }
        Node::Divide(expr1, expr2) => {
            
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            match (lhs.result, rhs.result) {
                (num1, num2) => {
                    Ok(EvalResult::new_by_number(num1 / num2))
                }
            }
        },
        Node::Negative(expr1) => {
            let value = eval(*expr1)?;

            match value.result {
                val => {
                    Ok(EvalResult::new_by_number(-val))
                }
            }
        }
        Node::Caret(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            match (lhs.result, rhs.result) {
                (num1, num2) => {
                    Ok(EvalResult::new_by_number(num1.powf(num2)))
                }
            }
        }
        Node::Die(expr1, expr2) => {
            let num_rows = eval(*expr1)?;
            let num_sides = eval(*expr2)?;
            let mut results: Vec<i128> = Vec::new();
            let mut rng = OsRng;

            if num_rows.result == 0.0 || num_sides.result == 0.0 {
                return Ok(EvalResult::new_by_dice(vec![0]));
            }

            for _ in 0..(num_rows.result as i128) {
                results.push(rng.gen_range(1..=(num_sides.result as i128)));
            }

            Ok(EvalResult::new_by_dice(results))

        }   
    }
} 

