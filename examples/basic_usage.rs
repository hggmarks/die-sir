use die_sir::DieSir;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("🎲 DieSir - Dice Rolling Calculator 🎲");
    println!("Enter expressions like:");
    println!("  2d6 + 3     (roll two 6-sided dice and add 3)");
    println!("  1d20        (roll one 20-sided die)");
    println!("  3d8 + 1d6   (roll three 8-sided dice and one 6-sided die)");
    println!("  (2d6) * 2   (roll two 6-sided dice and multiply by 2)");
    println!("Enter 'q' to quit\n");

    // Create a single DieSir instance to reuse
    let mut dice = DieSir::new();

    loop {
        print!("> ");
        // Flush to ensure the prompt is displayed
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input!");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") {
            println!("Goodbye!");
            break;
        }

        // Skip empty lines
        if input.is_empty() {
            continue;
        }

        // Try to roll and show the detailed results
        match dice.roll(input) {
            Ok(result) => {
                println!("Result: {}", result.result_expression);
                println!("Total: {}\n", result.total);
            }
            Err(e) => println!("Error: {}\n", e),
        }
    }
}
