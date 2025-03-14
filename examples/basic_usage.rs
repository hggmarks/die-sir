use std::io;
use die_sir::evaluate;

fn main() {
    println!("Dice Expression Evaluator");
    println!("Enter expressions like '2d6 + 3'");
    println!("Press Ctrl+C to exit");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("Result: {}\n", val),
                    Err(e) => println!("Error: {}\n", e),
                };
            }
            Err(error) => println!("Error reading input: {}\n", error),
        }
    }
} 