use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use taurine::formatter::Formatter;
use taurine::interpreter::create_default_interner;
use taurine::interpreter::Interpreter;
use taurine::lexer::tokenize_with_interner;
use taurine::optimizer::Optimizer;
use taurine::parser::Parser as TaurineParser;

#[derive(Parser, Debug)]
#[command(name = "taurine")]
#[command(about = "Taurine Programming Language v2.14.0", long_about = None)]
#[command(version = "2.14.0")]
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
        eprintln!("Error: expected file with .tau extension");
        std::process::exit(1);
    }

    let source = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read file '{filepath}': {e}");
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
    println!("Taurine v2.14 REPL");
    println!("Type 'exit' or 'quit' to exit\n");

    let mut interner = create_default_interner();
    let mut interpreter = Interpreter::with_interner(PathBuf::from("."), interner.clone());
    let mut input = String::new();

    loop {
        print!("taurine> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input_trimmed = input.trim();
        if input_trimmed == "exit" || input_trimmed == "quit" {
            println!("Goodbye!");
            break;
        }
        if input_trimmed == "clear" {
            #[cfg(target_os = "windows")]
            std::process::Command::new("cmd")
                .args(["/C", "cls"])
                .status()
                .unwrap();
            #[cfg(not(target_os = "windows"))]
            std::process::Command::new("clear").status().unwrap();
            continue;
        }
        if input_trimmed.is_empty() {
            continue;
        }

        let tokens = tokenize_with_interner(input_trimmed, &mut interner);
        let mut parser = TaurineParser::with_interner(tokens, interner.clone());

        match parser.parse() {
            Ok(program) => {
                if let Some(updated_interner) = parser.take_interner() {
                    interner = updated_interner;
                }

                if let Err(e) = interpreter.interpret(program) {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("Error: Parse error: {e}"),
        }
    }
}

fn run_demo() {
    println!("Taurine v2.14.0\n");
    println!("Usage:");
    println!("  taurine <file.tau>     Run a script");
    println!("  taurine --repl         Start REPL");
    println!("  taurine --optimize     Run with optimizations");
    println!("  taurine --format       Format a file");
    println!("  taurine --help         Show help\n");
    println!("Running demo:\n");

    let source = r#"
print("Hello from Taurine!")
let nums = [1, 2, 3, 4, 5]
print("Array:", nums)
for n in nums { print("Number:", n) }
"#;
    run_source(source, "<demo>", PathBuf::from("."), false, false);
}

fn run_source(source: &str, filename: &str, base_path: PathBuf, debug: bool, optimize: bool) {
    if debug {
        println!("File: {filename}\n");
    }

    let lex_start = Instant::now();
    let mut interner = create_default_interner();
    let tokens = tokenize_with_interner(source, &mut interner);
    let lex_time = lex_start.elapsed();

    if debug {
        println!("Tokens:");
        for t in &tokens {
            println!("  [{:3}] {:?} -> '{}'", t.line, t.kind, t.lexeme);
        }
        println!();
    }

    let parse_start = Instant::now();
    let mut parser = TaurineParser::with_interner(tokens, interner.clone());
    let mut program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(1);
        }
    };
    let parse_time = parse_start.elapsed();
    if debug {
        println!("AST (before optimization):\n{program:#?}\n");
    }

    let opt_time;
    if optimize {
        let opt_start = Instant::now();
        let mut optimizer = Optimizer::new();
        program = optimizer.optimize(program);
        opt_time = opt_start.elapsed();
        println!("Optimization: {:.4} ms", opt_time.as_secs_f64() * 1000.0);

        if debug {
            println!("AST (after optimization):\n{program:#?}\n");
        }
    }

    if debug {
        println!("Time:  Lexing: {:.4} ms", lex_time.as_secs_f64() * 1000.0);
        println!(
            "Time:  Parsing: {:.4} ms",
            parse_time.as_secs_f64() * 1000.0
        );
        println!("Execution:\n---");
    }

    let exec_start = Instant::now();
    let interner = parser
        .take_interner()
        .unwrap_or_else(create_default_interner);
    let mut interpreter = Interpreter::with_interner(base_path, interner);
    if optimize {
        interpreter.optimize();
    }

    match interpreter.interpret(program) {
        Ok(_) => {
            let exec_time = exec_start.elapsed();
            if debug {
                println!("Success!");
                println!(
                    "Execution time: {:.4} seconds ({:.2} ms)",
                    exec_time.as_secs_f64(),
                    exec_time.as_secs_f64() * 1000.0
                );
            }
        }
        Err(e) => {
            if e.message().starts_with("__EXIT__:") {
                let code: i32 = e
                    .message()
                    .trim_start_matches("__EXIT__:")
                    .parse()
                    .unwrap_or(0);
                std::process::exit(code);
            }
            eprintln!("error: {}", e.message());
            if e.line() > 0 {
                eprintln!("line {}", e.line());
            }
            std::process::exit(1);
        }
    }
}

fn format_file(args: Args) {
    let filepath = args.file.expect("Expected file to format");

    let source = match fs::read_to_string(&filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read file '{}': {}", filepath, e);
            std::process::exit(1);
        }
    };

    let mut interner = create_default_interner();
    let tokens = tokenize_with_interner(&source, &mut interner);
    let mut parser = TaurineParser::with_interner(tokens, interner);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let mut formatter = Formatter::new();
    let formatted = formatter.format(&program);

    print!("{formatted}");
}
