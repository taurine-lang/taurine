// examples/embedding/03_rust_functions.rs
// Example: Using Taurine with pre-defined functions
//
// Run with: cargo run --example 03_rust_functions

use taurine::Interpreter;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    println!("Using Taurine from Rust\n");

    let mut interp = Interpreter::new(PathBuf::from("."));

    // Run Taurine code with various features
    interp.run(r#"
        print("Using Taurine features:")
        print("")

        // Variables
        let x = 10
        let y = 20
        print(f"x = {x}, y = {y}")

        // Functions
        function add(a, b) {
            return a + b
        }
        print(f"add(5, 3) = {add(5, 3)}")

        // Arrays
        let arr = [1, 2, 3, 4, 5]
        print(f"arr = {arr}")
        print(f"arr[2] = {arr[2]}")

        // Tables
        let obj = { name: "Taurine", version: "1.0" }
        print(f"obj.name = {obj.name}")

        // Loops
        print("")
        print("Counting from 1 to 5:")
        for i in 1..6 {
            print(f"  {i}")
        }
    "#)?;

    println!("\nSuccess!");

    Ok(())
}
