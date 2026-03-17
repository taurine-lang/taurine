// examples/embedding/02_getting_values.rs
// Example: Getting values from Taurine
//
// Run with: cargo run --example 02_getting_values

use taurine::{Interpreter, Value};
use std::path::PathBuf;

fn main() -> Result<(), String> {
    println!("=== Getting Values from Taurine ===\n");

    let mut interp = Interpreter::new(PathBuf::from("."));

    // Define variables in Taurine
    interp.run(r#"
        let x = 42
        let name = "Taurine"
        let arr = [1, 2, 3, 4, 5]
    "#)?;

    // Get values from Taurine
    println!("Getting values from Taurine...\n");

    let x = interp.get("x")?;
    if let Value::Number(n) = x {
        println!("  x = {}", n);
    }

    let name = interp.get("name")?;
    if let Value::String(s) = name {
        println!("  name = {}", s);
    }

    let arr = interp.get("arr")?;
    if let Value::Array(_) = arr {
        println!("  arr = {:?}", arr);
    }

    println!("\n=== Success! ===");

    Ok(())
}
