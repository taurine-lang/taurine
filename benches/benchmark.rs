//! Benchmarks for Taurine
//!
//! This module provides comprehensive benchmarks for measuring
//! the performance of the Taurine interpreter and its optimizations.

use std::time::Instant;
use taurine::{Interpreter, Optimizer, Compiler, SafetyLimits, AstArena, StringInterner};
use taurine::parser::Parser;
use taurine::lexer::{tokenize, tokenize_with_interner};
use std::path::PathBuf;

fn main() {
    println!("=== Taurine Benchmarks ===\n");

    // Benchmark 1: Simple arithmetic
    benchmark_arithmetic();

    // Benchmark 2: Loop performance
    benchmark_loop();

    // Benchmark 3: Function calls
    benchmark_function_calls();

    // Benchmark 4: Array operations
    benchmark_array();

    // Benchmark 5: Table operations
    benchmark_table();

    // Benchmark 6: String operations
    benchmark_string();

    // Benchmark 7: Optimizer comparison
    benchmark_optimizer();

    // Benchmark 8: String interning
    benchmark_string_interning();

    // Benchmark 9: Arena allocation
    benchmark_arena_allocation();

    // Benchmark 10: Lexer performance
    benchmark_lexer();

    // Benchmark 11: Parser performance
    benchmark_parser();

    // Benchmark 12: Interpreter vs Bytecode
    benchmark_interpreter_vs_bytecode();

    println!("\n=== All benchmarks completed ===");
}

fn benchmark_arithmetic() {
    println!("\n[1] Arithmetic Operations");
    let code = r#"
        let a = 10
        let b = 20
        let c = a + b
        let d = c * 2
        let e = d / 4
        let f = e - 5
        print(f"Arithmetic result: {f}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    1000 iterations: {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
}

fn benchmark_loop() {
    println!("\n[2] Loop Performance");
    let code = r#"
        let sum = 0
        for i in 1..1000 {
            sum = sum + 1
        }
        print(f"Loop sum: {sum}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..100 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    100 iterations (1000 loop): {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_function_calls() {
    println!("\n[3] Function Calls");
    let code = r#"
        function add(a, b) {
            return a + b
        }
        let result = 0
        for i in 1..100 {
            result = add(i, i + 1)
        }
        print(f"Function result: {result}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..100 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    100 iterations (100 calls): {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_array() {
    println!("\n[4] Array Operations");
    let code = r#"
        let arr = []
        for i in 1..100 {
            io_arraypush(arr, i)
        }
        let len = #arr
        print(f"Array length: {len}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..100 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    100 iterations (100 push): {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_table() {
    println!("\n[5] Table Operations");
    let code = r#"
        let table = {}
        for i in 1..50 {
            table["key_" + tostring(i)] = i * 2
        }
        let val = table["key_25"]
        print(f"Table value: {val}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..100 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    100 iterations (50 inserts): {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_string() {
    println!("\n[6] String Operations");
    let code = r#"
        let str = "hello"
        for i in 1..50 {
            str = str + " world"
        }
        print(f"String length: {#str}")
    "#;
    
    let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
    let start = Instant::now();
    for _ in 0..100 {
        let _ = interp.run(code);
    }
    let elapsed = start.elapsed();
    println!("    100 iterations (50 concat): {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_optimizer() {
    println!("\n[7] Optimizer Comparison");
    
    let code = r#"
        let x = 10
        let y = 20
        let z = x + y
        let result = z * 2
        if true {
            print(f"Optimized: {result}")
        } else {
            print("unreachable")
        }
    "#;
    
    // Without optimization
    let tokens = taurine::tokenize(code);
    let mut parser = taurine::Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let start = Instant::now();
    for _ in 0..100 {
        let mut interp = Interpreter::new(PathBuf::from("."));
        let _ = interp.interpret(program.clone());
    }
    let elapsed_no_opt = start.elapsed();
    
    // With optimization
    let tokens = taurine::tokenize(code);
    let mut parser = taurine::Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut optimizer = Optimizer::new();
    let optimized = optimizer.optimize(program);
    
    let start = Instant::now();
    for _ in 0..100 {
        let mut interp = Interpreter::new(PathBuf::from("."));
        let _ = interp.interpret(optimized.clone());
    }
    let elapsed_opt = start.elapsed();
    
    println!("    Without optimization: {:.2?} ({:.2} µs/iter)", elapsed_no_opt, elapsed_no_opt.as_micros() as f64 / 100.0);
    println!("    With optimization:    {:.2?} ({:.2} µs/iter)", elapsed_opt, elapsed_opt.as_micros() as f64 / 100.0);
    
    if elapsed_opt < elapsed_no_opt {
        let speedup = elapsed_no_opt.as_secs_f64() / elapsed_opt.as_secs_f64();
        println!("    Speedup: {:.2}x faster with optimizations", speedup);
    }
}

// ============================================================================
// New Benchmarks: String Interning, Arena Allocation, Lexer, Parser
// ============================================================================

fn benchmark_string_interning() {
    println!("\n[8] String Interning");
    
    let code = r#"
        let variable_name = 10
        let another_variable = 20
        let variable_name_copy = variable_name
        let result = variable_name + another_variable
        print(f"Result: {result}")
    "#;
    
    // Without interning
    let start = Instant::now();
    for _ in 0..1000 {
        let _tokens = tokenize(code);
    }
    let elapsed_no_intern = start.elapsed();
    
    // With interning
    let start = Instant::now();
    for _ in 0..1000 {
        let mut interner = StringInterner::with_capacity(64);
        let _tokens = tokenize_with_interner(code, &mut interner);
    }
    let elapsed_with_intern = start.elapsed();
    
    println!("    Without interning: {:.2?} ({:.2} µs/iter)", elapsed_no_intern, elapsed_no_intern.as_micros() as f64 / 1000.0);
    println!("    With interning:    {:.2?} ({:.2} µs/iter)", elapsed_with_intern, elapsed_with_intern.as_micros() as f64 / 1000.0);
    
    // Memory comparison
    let mut interner = StringInterner::with_capacity(64);
    let _tokens = tokenize_with_interner(code, &mut interner);
    println!("    Interner memory: {} bytes", interner.memory_usage());
}

fn benchmark_arena_allocation() {
    println!("\n[9] Arena Allocation");
    
    let code = r#"
        let a = 1
        let b = 2
        let c = 3
        let d = a + b
        let e = c * d
        let f = e - a
        function test(x, y) {
            return x + y
        }
        let result = test(d, e)
    "#;
    
    // Without arena
    let start = Instant::now();
    for _ in 0..1000 {
        let tokens = tokenize(code);
        let mut parser = Parser::new(tokens);
        let _ = parser.parse();
    }
    let elapsed_no_arena = start.elapsed();
    
    // With arena
    let start = Instant::now();
    for _ in 0..1000 {
        let tokens = tokenize(code);
        let arena = AstArena::with_capacity(128, 64);
        let mut parser = Parser::with_arena(tokens, arena);
        let _ = parser.parse();
    }
    let elapsed_with_arena = start.elapsed();
    
    println!("    Without arena: {:.2?} ({:.2} µs/iter)", elapsed_no_arena, elapsed_no_arena.as_micros() as f64 / 1000.0);
    println!("    With arena:    {:.2?} ({:.2} µs/iter)", elapsed_with_arena, elapsed_with_arena.as_micros() as f64 / 1000.0);
    
    if elapsed_with_arena < elapsed_no_arena {
        let speedup = elapsed_no_arena.as_secs_f64() / elapsed_with_arena.as_secs_f64();
        println!("    Speedup: {:.2}x faster with arena", speedup);
    }
}

fn benchmark_lexer() {
    println!("\n[10] Lexer Performance");
    
    // Small program
    let small_code = r#"let x = 10 let y = 20 print(x + y)"#;
    
    // Medium program
    let medium_code = r#"
        function fibonacci(n) {
            if (n <= 1) { return n }
            return fibonacci(n - 1) + fibonacci(n - 2)
        }
        let result = fibonacci(10)
        print(f"Fibonacci(10) = {result}")
    "#;
    
    // Large program
    let large_code = r#"
        class Calculator {
            function add(a, b) { return a + b }
            function sub(a, b) { return a - b }
            function mul(a, b) { return a * b }
            function div(a, b) { return a / b }
        }
        let calc = new Calculator()
        let result = calc.add(10, 20)
        result = calc.mul(result, 2)
        print(f"Result: {result}")
    "#;
    
    println!("    Small program ({} chars):", small_code.len());
    let start = Instant::now();
    for _ in 0..10000 {
        let _tokens = tokenize(small_code);
    }
    let elapsed = start.elapsed();
    println!("      {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 10000.0);
    
    println!("    Medium program ({} chars):", medium_code.len());
    let start = Instant::now();
    for _ in 0..1000 {
        let _tokens = tokenize(medium_code);
    }
    let elapsed = start.elapsed();
    println!("      {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
    
    println!("    Large program ({} chars):", large_code.len());
    let start = Instant::now();
    for _ in 0..100 {
        let _tokens = tokenize(large_code);
    }
    let elapsed = start.elapsed();
    println!("      {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 100.0);
}

fn benchmark_parser() {
    println!("\n[11] Parser Performance");
    
    let code = r#"
        function factorial(n) {
            if (n <= 1) { return 1 }
            return n * factorial(n - 1)
        }
        
        let result = factorial(5)
        print(f"Factorial(5) = {result}")
    "#;
    
    // Without optimizations
    let start = Instant::now();
    for _ in 0..1000 {
        let tokens = tokenize(code);
        let mut parser = Parser::new(tokens);
        let _ = parser.parse();
    }
    let elapsed = start.elapsed();
    println!("    Standard parser: {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
    
    // With arena
    let start = Instant::now();
    for _ in 0..1000 {
        let tokens = tokenize(code);
        let arena = AstArena::with_capacity(256, 128);
        let mut parser = Parser::with_arena(tokens, arena);
        let _ = parser.parse();
    }
    let elapsed = start.elapsed();
    println!("    With arena:      {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
    
    // With interner
    let start = Instant::now();
    for _ in 0..1000 {
        let mut interner = StringInterner::with_capacity(64);
        let tokens = tokenize_with_interner(code, &mut interner);
        let mut parser = Parser::with_interner(tokens, interner);
        let _ = parser.parse();
    }
    let elapsed = start.elapsed();
    println!("    With interner:   {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
    
    // With both
    let start = Instant::now();
    for _ in 0..1000 {
        let mut interner = StringInterner::with_capacity(64);
        let tokens = tokenize_with_interner(code, &mut interner);
        let arena = AstArena::with_capacity(256, 128);
        let mut parser = Parser::with_arena_and_interner(tokens, arena, interner);
        let _ = parser.parse();
    }
    let elapsed = start.elapsed();
    println!("    With both:       {:.2?} ({:.2} µs/iter)", elapsed, elapsed.as_micros() as f64 / 1000.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_runs() {
        let code = r#"
            let x = 10
            let y = 20
            print(x + y)
        "#;
        
        let mut interp = Interpreter::with_limits(PathBuf::from("."), SafetyLimits::new().without_timeout());
        let result = interp.run(code);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_optimizer() {
        use taurine::{tokenize, Parser};
        
        let code = r#"
            let x = 10 + 20
            print(x)
        "#;
        
        let tokens = tokenize(code);
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        let mut optimizer = Optimizer::new();
        let _ = optimizer.optimize(program);
    }
    
    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("hello");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_arena() {
        use taurine::ast::Expr;

        let arena = AstArena::new();
        let expr = arena.alloc_expr(Expr::Number(42.0));
        assert!(matches!(expr, Expr::Number(42.0)));
    }
}

// ============================================================================
// Interpreter vs Bytecode Benchmark
// ============================================================================

fn benchmark_interpreter_vs_bytecode() {
    println!("Benchmark 12: Interpreter vs Bytecode");
    println!("--------------------------------------");

    let source = r#"
loc sum = 0
for (loc i = 0; i < 1000; i = i + 1) {
    sum = sum + i
}
print(sum)
"#;

    // Interpreter
    let interp_start = Instant::now();
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.run(source).unwrap();
    let interp_time = interp_start.elapsed();

    // Bytecode - skip for now (bytecode VM has limited support)
    println!("  Interpreter: {:.4} ms", interp_time.as_secs_f64() * 1000.0);
    println!("  Bytecode:    (skipped - limited support)");
    println!();
}
