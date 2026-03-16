//! Taurine Programming Language
//!
//! Taurine is a fast, embeddable scripting language implemented in Rust.
//! It combines the simplicity of Lua with the performance of compiled languages.
//!
//! # Features
//!
//! - Dynamic typing with tables and arrays
//! - First-class functions with closures
//! - JSON parsing and stringification
//! - HTTP client (GET, POST, PUT, DELETE)
//! - Regular expressions
//! - Crypto functions (MD5, SHA256, Base64, UUID)
//! - Date/time formatting
//! - Range-based for loops
//! - Nil-safe operators
//! - break/continue in loops
//! - String interpolation (f-strings)
//! - Multi-return values with destructuring
//! - let/const variable declarations
//! - JSON-style tables {key: val}
//!
//! # Usage
//!
//! ```bash
//! # Run a script
//! taurine script.tau
//!
//! # Start REPL
//! taurine --repl
//!
//! # With optimizations
//! taurine --optimize script.tau
//! ```
//!
//! # Example
//!
//! ```taurine
//! // Variables
//! let x = 10
//! const PI = 3.14
//!
//! // Functions with multi-return
//! function divmod(a, b) {
//!     return a / b, a % b
//! }
//! let {q, r} = divmod(10, 3)
//!
//! // f-strings
//! print(f"Result: {q} remainder {r}")
//!
//! // JSON-style tables
//! let obj = { name: "Taurine", version: "1.0" }
//! print(obj.name)
//! ```

mod lexer;
mod ast;
mod parser;
mod value;
mod environment;
mod interpreter;
mod optimizer;
mod formatter;

use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use std::time::Instant;
use clap::Parser;
use lexer::tokenize;
use parser::Parser as TaurineParser;
use interpreter::Interpreter;
use optimizer::Optimizer;
use formatter::Formatter;

#[derive(Parser, Debug)]
#[command(name = "taurine")]
#[command(about = "Taurine Programming Language v1.0", long_about = None)]
struct Args {
    file: Option<String>,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    #[arg(long, default_value_t = false)]
    repl: bool,
    #[arg(short, long, default_value_t = false)]
    optimize: bool,
    #[arg(long, default_value_t = false)]
    format: bool,
}

fn main() {
    let args = Args::parse();

    if args.format {
        format_file(args);
        return;
    }

    if args.repl {
        run_repl();
        return;
    }

    if args.file.is_none() {
        run_demo();
        return;
    }

    run_file(args);
}

fn run_file(args: Args) {
    let filepath = args.file.unwrap();
    if !filepath.ends_with(".tau") {
        eprintln!("❌ Error: expected file with .tau extension");
        std::process::exit(1);
    }

    let source = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read file '{filepath}': {e}");
            std::process::exit(1);
        }
    };

    let base_path = PathBuf::from(&filepath)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    run_source(&source, &filepath, base_path, args.debug, args.optimize);
}

fn run_repl() {
    println!("🐂 Taurine v1.0 REPL");
    println!("Type 'exit' or 'quit' to exit\n");

    let mut interpreter = Interpreter::new(PathBuf::from("."));
    let mut input = String::new();

    loop {
        print!("taurine> ");
        io::stdout().flush().unwrap();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input == "exit" || input == "quit" {
            println!("👋 Goodbye!");
            break;
        }

        if input == "clear" {
            #[cfg(target_os = "windows")]
            std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();
            #[cfg(not(target_os = "windows"))]
            std::process::Command::new("clear").status().unwrap();
            continue;
        }

        if input.is_empty() {
            continue;
        }

        let tokens = tokenize(input);
        let mut parser = TaurineParser::new(tokens);

        match parser.parse() {
            Ok(program) => {
                if let Err(e) = interpreter.interpret(program) {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("❌ Parse error: {e}"),
        }
    }
}

fn run_demo() {
    println!("=== 🐂 Taurine v1.0 ===\n");
    println!("Usage:");
    println!("  taurine <file.tau>     Run a script");
    println!("  taurine --repl         Start REPL");
    println!("  taurine --optimize     Run with optimizations");
    println!("  taurine --format       Format a file");
    println!("  taurine --help         Show help\n");
    println!("Running demo:\n");

    let source = r#"
print("Hello from Taurine!")
loc nums = [1, 2, 3, 4, 5]
print("Array:", nums)
for n in nums { print("Number:", n) }
"#;
    run_source(source, "<demo>", PathBuf::from("."), false, false);
}

fn run_source(source: &str, filename: &str, base_path: PathBuf, debug: bool, optimize: bool) {
    println!("📄 File: {filename}\n");

    let lex_start = Instant::now();
    let tokens = tokenize(source);
    let lex_time = lex_start.elapsed();

    if debug {
        println!("🔍 Tokens:");
        for t in &tokens {
            println!("  [{:3}] {:?} -> '{}'", t.line, t.kind, t.lexeme);
        }
        println!();
    }

    let parse_start = Instant::now();
    let mut parser = TaurineParser::new(tokens);
    let mut program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Parse error: {e}");
            std::process::exit(1);
        }
    };
    let parse_time = parse_start.elapsed();

    if debug {
        println!("🌳 AST (before optimization):\n{program:#?}\n");
    }

    let opt_time;
    if optimize {
        let opt_start = Instant::now();
        let mut optimizer = Optimizer::new();
        program = optimizer.optimize(program);
        opt_time = opt_start.elapsed();
        println!("⚡ Optimization: {:.4} ms", opt_time.as_secs_f64() * 1000.0);

        if debug {
            println!("🌳 AST (after optimization):\n{program:#?}\n");
        }
    }

    println!("⏱  Lexing: {:.4} ms", lex_time.as_secs_f64() * 1000.0);
    println!("⏱  Parsing: {:.4} ms", parse_time.as_secs_f64() * 1000.0);
    println!("▶️  Execution:\n---");

    let exec_start = Instant::now();
    let mut interpreter = Interpreter::new(base_path);
    if optimize {
        interpreter.optimize();
    }

    match interpreter.interpret(program) {
        Ok(_) => {
            let exec_time = exec_start.elapsed();
            println!("---\n✅ Success!");
            println!("⏱  Execution time: {:.4} seconds ({:.2} ms)",
                     exec_time.as_secs_f64(),
                     exec_time.as_secs_f64() * 1000.0);
        }
        Err(e) => {
            let exec_time = exec_start.elapsed();
            eprintln!("---\n{e}");
            eprintln!("⏱  Time until error: {:.4} seconds", exec_time.as_secs_f64());
            std::process::exit(1);
        }
    }
}

fn format_file(args: Args) {
    let filepath = args.file.expect("Expected file to format");
    
    let source = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to read file '{}': {}", filepath, e);
            std::process::exit(1);
        }
    };
    
    let tokens = tokenize(&source);
    let mut parser = TaurineParser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Parse error: {}", e);
            std::process::exit(1);
        }
    };
    
    let mut formatter = Formatter::new();
    let formatted = formatter.format(&program);
    
    print!("{formatted}");
}
