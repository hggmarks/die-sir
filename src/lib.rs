//! DieSir is a library for rolling dice and calculating expressions
//! commonly used in tabletop RPGs and board games.
//! 
//! # Examples
//! 
//! ```
//! use die_sir::DieSir;
//! 
//! let mut dice = DieSir::new();
//! 
//! // Simple dice roll with modifier
//! let result = dice.roll("2d6 + 3").unwrap();
//! println!("Roll result: {}", result);
//! 
//! // Complex roll with multiple dice
//! let result = dice.roll("2d8 + 1d6 + 4").unwrap();
//! println!("Rolls: {:?}", result.rolls);
//! println!("Total with modifier: {}", result.total);
//! ```

mod parsedie;

use parsedie::{
    ast,
    parser::Parser,
};

// Re-export types for users
pub use parsedie::ast::{DieRoll, RollResult};
pub use parsedie::parser::ParseError;

/// The main struct for rolling dice and calculating expressions.
pub struct DieSir {
    parser: Parser,
}

impl DieSir {
    /// Create a new DieSir instance.
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    /// Roll dice and calculate the result of an expression.
    /// 
    /// # Arguments
    /// 
    /// * `expression` - A string like "2d6 + 3" where:
    ///   * `2d6` means "roll two 6-sided dice"
    ///   * `+3` adds 3 to the result
    /// 
    /// # Returns
    /// 
    /// Returns a `RollResult` containing:
    /// * `rolls` - Vector of individual die rolls with their sides
    /// * `total` - The final result after all calculations
    /// * `modifier` - Any constant modifiers (like the +3 in 2d6+3)
    /// * `is_pure_calculation` - Whether this was just math without dice
    /// 
    /// # Examples
    /// 
    /// ```
    /// use die_sir::DieSir;
    /// 
    /// let mut dice = DieSir::new();
    /// 
    /// // Roll 2 six-sided dice and add 3
    /// let result = dice.roll("2d6 + 3").unwrap();
    /// println!("Rolls: {:?}", result.rolls);
    /// println!("Total with modifier: {}", result.total);
    /// 
    /// // Roll for damage with different dice
    /// let result = dice.roll("1d8 + 2d6").unwrap();
    /// for roll in &result.rolls {
    ///     println!("Rolled {} on d{}", roll.value, roll.sides);
    /// }
    /// 
    /// // Complex expression
    /// let result = dice.roll("(2d8 + 1d6 + 4) * 2").unwrap();
    /// println!("Critical hit! {}", result);
    /// ```
    pub fn roll(&mut self, expression: &str) -> Result<RollResult, ParseError> {
        let expr = expression.split_whitespace().collect::<String>();
        let ast = self.parser.parse(&expr)?;
        Ok(ast::eval(ast)?)
    }
}

// Implement Default to allow creating DieSir with default values
impl Default for DieSir {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for one-off dice rolls.
/// For multiple rolls, create a `DieSir` instance instead.
pub fn roll(expression: &str) -> Result<RollResult, ParseError> {
    DieSir::new().roll(expression)
}
