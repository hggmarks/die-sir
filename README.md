# Die-Sir

Die-Sir (Dicer) is a dice parser for random dice rolling with support for modifiers through basic math operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
die-sir = "0.1.0"
```

Basic usage example:

```rust
use die_sir::evaluate;

fn main() {
    let result = evaluate("2d6 + 3".to_string());
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
```

## Features

- Parse dice expressions (e.g., "2d6", "1d20+5")
- Support for basic arithmetic operations (+, -, *, /, ^)
- Error handling for invalid expressions and number overflow

## License

This project is licensed under the MIT License - see the LICENSE file for details