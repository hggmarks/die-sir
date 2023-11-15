use std::io;

use parsedie::{ast::EvalResult, parser::{ParseError, Parser}};

use crate::parsedie::ast;

mod parsedie;


fn evaluate(expr: String) -> Result<EvalResult, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();

    let mut die_parser = Parser::new(&expr)?;
    let ast = die_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)

}


fn main() {
    println!("Hello, world!");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => {
                        println!("Error in evaluating expression. Please enter a valid expression\n");
                    }
                };
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}






