// examples/embedding/01_basic_rust.rs
// Basic example: Running Taurine code from Rust
//
// Run with: cargo run --example 01_basic_rust

use taurine::Interpreter;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    println!("Basic Rust Embedding Example\n");

    // Create interpreter
    let mut interp = Interpreter::new(PathBuf::from("."));

    // Run simple Taurine code
    println!("Running Taurine code...\n");
    interp.run(r#"
        let x = 10
        let y = 20
        let sum = x + y
        print(f"x = {x}")
        print(f"y = {y}")
        print(f"x + y = {sum}")
    "#)?;

    println!("\nSuccess!");

    Ok(())
}
