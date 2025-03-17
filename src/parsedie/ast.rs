use rand::rngs::OsRng;
use rand::Rng;
use std::error;
use std::collections::HashMap;

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

/// Represents a single die roll result
#[derive(Clone, Debug, PartialEq)]
pub struct DieRoll {
    /// Number of sides on the die
    pub sides: i128,
    /// The value rolled
    pub value: i128,
}

/// Represents the complete result of a dice expression evaluation
#[derive(Clone, Debug, PartialEq)]
pub struct RollResult {
    /// Individual die rolls if any
    pub rolls: Vec<DieRoll>,
    /// The total value after all calculations
    pub total: f64,
    /// Any modifiers applied (like +2 in 2d6+2)
    pub modifier: f64,
    /// Whether this was a pure calculation without dice
    pub is_pure_calculation: bool,
    /// The formatted result expression (e.g., "2d10 [7, 2] + 5")
    pub result_expression: String,
}

impl RollResult {
    fn format_dice_rolls(&self) -> String {
        if self.is_pure_calculation {
            return self.total.to_string();
        }

        // Group rolls by dice type
        let mut rolls_by_type: HashMap<i128, Vec<i128>> = HashMap::new();
        for roll in &self.rolls {
            rolls_by_type.entry(roll.sides)
                .or_insert_with(Vec::new)
                .push(roll.value);
        }

        // Build result string
        let mut result_parts = Vec::new();
        for (sides, values) in rolls_by_type {
            result_parts.push(format!("{}d{} [{}]", 
                values.len(), 
                sides, 
                values.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Add modifier if present
        if self.modifier != 0.0 {
            format!("{} + {}", result_parts.join(" + "), self.modifier)
        } else {
            result_parts.join(" + ")
        }
    }
}

impl std::fmt::Display for RollResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.result_expression)
    }
}

pub type EvalResult = RollResult;

pub fn eval(expr: Node) -> Result<EvalResult, Box<dyn error::Error>> {
    match expr {
        Node::Number(i) => {
            let result = RollResult {
                rolls: Vec::new(),
                total: i as f64,
                modifier: 0.0,
                is_pure_calculation: true,
                result_expression: i.to_string(),
            };
            Ok(result)
        },
        Node::Add(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            let mut result = RollResult {
                rolls: Vec::new(),
                total: lhs.total + rhs.total,
                modifier: lhs.modifier + rhs.modifier,
                is_pure_calculation: lhs.is_pure_calculation && rhs.is_pure_calculation,
                result_expression: format!("{} + {}", lhs.result_expression, rhs.result_expression),
            };
            result.rolls.extend(lhs.rolls);
            result.rolls.extend(rhs.rolls);
            Ok(result)
        }
        Node::Subtract(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            let mut result = RollResult {
                rolls: Vec::new(),
                total: lhs.total - rhs.total,
                modifier: lhs.modifier - rhs.modifier,
                is_pure_calculation: lhs.is_pure_calculation && rhs.is_pure_calculation,
                result_expression: format!("{} - {}", lhs.result_expression, rhs.result_expression),
            };
            result.rolls.extend(lhs.rolls);
            result.rolls.extend(rhs.rolls);
            Ok(result)
        }
        Node::Multiply(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            if !lhs.is_pure_calculation || !rhs.is_pure_calculation {
                return Err("Cannot multiply dice results directly".into());
            }

            let result = RollResult {
                rolls: Vec::new(),
                total: lhs.total * rhs.total,
                modifier: 0.0,
                is_pure_calculation: true,
                result_expression: format!("{} * {}", lhs.result_expression, rhs.result_expression),
            };
            Ok(result)
        }
        Node::Divide(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            if !lhs.is_pure_calculation || !rhs.is_pure_calculation {
                return Err("Cannot divide dice results directly".into());
            }

            if rhs.total == 0.0 {
                return Err("Division by zero".into());
            }

            let result = RollResult {
                rolls: Vec::new(),
                total: lhs.total / rhs.total,
                modifier: 0.0,
                is_pure_calculation: true,
                result_expression: format!("{} / {}", lhs.result_expression, rhs.result_expression),
            };
            Ok(result)
        }
        Node::Negative(expr1) => {
            let value = eval(*expr1)?;

            if !value.is_pure_calculation {
                return Err("Cannot negate dice results directly".into());
            }

            let result = RollResult {
                rolls: Vec::new(),
                total: -value.total,
                modifier: -value.modifier,
                is_pure_calculation: true,
                result_expression: format!("-{}", value.result_expression),
            };
            Ok(result)
        }
        Node::Caret(expr1, expr2) => {
            let lhs = eval(*expr1)?;
            let rhs = eval(*expr2)?;

            if !lhs.is_pure_calculation || !rhs.is_pure_calculation {
                return Err("Cannot use dice results in power operation".into());
            }

            let result = RollResult {
                rolls: Vec::new(),
                total: lhs.total.powf(rhs.total),
                modifier: 0.0,
                is_pure_calculation: true,
                result_expression: format!("{}^{}", lhs.result_expression, rhs.result_expression),
            };
            Ok(result)
        }
        Node::Die(expr1, expr2) => {
            let num_rolls = eval(*expr1)?;
            let num_sides = eval(*expr2)?;
            let mut rng = OsRng;

            if !num_rolls.is_pure_calculation || !num_sides.is_pure_calculation {
                return Err("Die expressions must have numeric operands".into());
            }

            let rolls_count = num_rolls.total as i128;
            let sides = num_sides.total as i128;

            if rolls_count <= 0 || sides <= 0 {
                let result = RollResult {
                    rolls: Vec::new(),
                    total: 0.0,
                    modifier: 0.0,
                    is_pure_calculation: false,
                    result_expression: "0".to_string(),
                };
                return Ok(result);
            }

            let mut rolls = Vec::with_capacity(rolls_count as usize);
            let mut total = 0.0;

            for _ in 0..rolls_count {
                let value = rng.gen_range(1..=sides);
                total += value as f64;
                rolls.push(DieRoll { sides, value });
            }

            let mut result = RollResult {
                rolls,
                total,
                modifier: 0.0,
                is_pure_calculation: false,
                result_expression: String::new(),
            };
            result.result_expression = result.format_dice_rolls();
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a number node
    fn num(n: i128) -> Node {
        Node::Number(n)
    }

    #[test]
    fn test_basic_arithmetic() {
        // Test addition
        let add = Node::Add(Box::new(num(1)), Box::new(num(2)));
        assert_eq!(eval(add).unwrap().total, 3.0);

        // Test subtraction
        let sub = Node::Subtract(Box::new(num(5)), Box::new(num(3)));
        assert_eq!(eval(sub).unwrap().total, 2.0);

        // Test multiplication
        let mul = Node::Multiply(Box::new(num(4)), Box::new(num(2)));
        assert_eq!(eval(mul).unwrap().total, 8.0);

        // Test division
        let div = Node::Divide(Box::new(num(10)), Box::new(num(2)));
        assert_eq!(eval(div).unwrap().total, 5.0);
    }

    #[test]
    fn test_complex_expressions() {
        // Test (1 + 2) * 3
        let expr = Node::Multiply(
            Box::new(Node::Add(Box::new(num(1)), Box::new(num(2)))),
            Box::new(num(3))
        );
        assert_eq!(eval(expr).unwrap().total, 9.0);

        // Test 10 - (3 * 2)
        let expr = Node::Subtract(
            Box::new(num(10)),
            Box::new(Node::Multiply(Box::new(num(3)), Box::new(num(2))))
        );
        assert_eq!(eval(expr).unwrap().total, 4.0);
    }

    #[test]
    fn test_negative_numbers() {
        // Test -5
        let neg = Node::Negative(Box::new(num(5)));
        assert_eq!(eval(neg).unwrap().total, -5.0);

        // Test -3 + 7
        let expr = Node::Add(
            Box::new(Node::Negative(Box::new(num(3)))),
            Box::new(num(7))
        );
        assert_eq!(eval(expr).unwrap().total, 4.0);
    }

    #[test]
    fn test_power_operation() {
        // Test 2^3
        let pow = Node::Caret(Box::new(num(2)), Box::new(num(3)));
        assert_eq!(eval(pow).unwrap().total, 8.0);

        // Test 3^2
        let pow = Node::Caret(Box::new(num(3)), Box::new(num(2)));
        assert_eq!(eval(pow).unwrap().total, 9.0);
    }

    #[test]
    fn test_die_rolls() {
        // Test single die (1d6)
        let die = Node::Die(Box::new(num(1)), Box::new(num(6)));
        let result = eval(die).unwrap();
        assert!(result.total >= 1.0 && result.total <= 6.0);
        assert_eq!(result.rolls.len(), 1);
        assert_eq!(result.rolls[0].sides, 6);
        assert_eq!(result.rolls[0].value as f64, result.total);

        // Test multiple dice (3d6)
        let dice = Node::Die(Box::new(num(3)), Box::new(num(6)));
        let result = eval(dice).unwrap();
        assert_eq!(result.rolls.len(), 3);
        for roll in &result.rolls {
            assert!(roll.value >= 1 && roll.value <= 6);
        }

        // Test 0 dice
        let zero_dice = Node::Die(Box::new(num(0)), Box::new(num(6)));
        let result = eval(zero_dice).unwrap();
        assert_eq!(result.total, 0.0);
        assert!(result.rolls.is_empty());
    }

    #[test]
    fn test_error_conditions() {
        // Test division by zero
        let div_zero = Node::Divide(Box::new(num(10)), Box::new(num(0)));
        assert!(eval(div_zero).is_err());

        // Test invalid die sides
        let invalid_die = Node::Die(Box::new(num(1)), Box::new(num(0)));
        let result = eval(invalid_die).unwrap();
        assert_eq!(result.total, 0.0);
        assert!(result.rolls.is_empty());

        // Test negative dice count
        let neg_dice = Node::Die(Box::new(Node::Negative(Box::new(num(1)))), Box::new(num(6)));
        let result = eval(neg_dice).unwrap();
        assert_eq!(result.total, 0.0);
        assert!(result.rolls.is_empty());
    }
}
