//! Benchmark Suite for Taurine
//!
//! This module provides comprehensive benchmarks for:
//! - Interpreter performance
//! - GC performance across strategies
//! - Comparison with baseline operations

use std::time::Instant;

/// Benchmark result
#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time_ms: f64,
    pub avg_time_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub ops_per_second: f64,
}

impl BenchmarkResult {
    pub fn print(&self) {
        println!("{:<40} {:>10} iterations", self.name, self.iterations);
        println!("  Total:  {:.2} ms", self.total_time_ms);
        println!("  Avg:    {:.4} ms", self.avg_time_ms);
        println!("  Min:    {:.4} ms", self.min_time_ms);
        println!("  Max:    {:.4} ms", self.max_time_ms);
        println!("  Ops/s:  {:.0}", self.ops_per_second);
        println!();
    }
}

/// Run a benchmark
pub fn benchmark<F>(name: &str, iterations: usize, mut f: F) -> BenchmarkResult
where
    F: FnMut(),
{
    let mut times = Vec::with_capacity(iterations);
    
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        times.push(start.elapsed().as_secs_f64() * 1000.0);
    }

    let total: f64 = times.iter().sum();
    let avg = total / iterations as f64;
    let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time_ms: total,
        avg_time_ms: avg,
        min_time_ms: min,
        max_time_ms: max,
        ops_per_second: iterations as f64 / (total / 1000.0),
    }
}

// Micro Benchmarks

/// Benchmark variable allocation
pub fn bench_variable_allocation(iterations: usize) -> BenchmarkResult {
    benchmark("Variable Allocation", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run("let x = 42");
    })
}

/// Benchmark function calls
pub fn bench_function_calls(iterations: usize) -> BenchmarkResult {
    benchmark("Function Calls", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            function add(a, b) { return a + b }
            add(10, 20)
        "#);
    })
}

/// Benchmark loop performance
pub fn bench_loop_performance(iterations: usize) -> BenchmarkResult {
    benchmark("Loop Performance (1000 iter)", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let sum = 0
            for i in 1..1001 { sum = sum + i }
        "#);
    })
}

/// Benchmark array operations
pub fn bench_array_operations(iterations: usize) -> BenchmarkResult {
    benchmark("Array Operations", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let arr = [1, 2, 3, 4, 5]
            io_arraypush(arr, 6)
            io_arraylen(arr)
        "#);
    })
}

/// Benchmark table operations
pub fn bench_table_operations(iterations: usize) -> BenchmarkResult {
    benchmark("Table Operations", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let obj = { name: "test", value: 42 }
            obj.name
        "#);
    })
}

/// Benchmark string operations
pub fn bench_string_operations(iterations: usize) -> BenchmarkResult {
    benchmark("String Operations", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let s = "Hello, World!"
            io_strupper(s)
            io_strlower(s)
            io_strtrim(s)
        "#);
    })
}

/// Benchmark math operations
pub fn bench_math_operations(iterations: usize) -> BenchmarkResult {
    benchmark("Math Operations", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let a = 100
            let b = 23
            a + b
            a - b
            a * b
            a / b
            a % b
        "#);
    })
}

/// Benchmark closure creation
pub fn bench_closure_creation(iterations: usize) -> BenchmarkResult {
    benchmark("Closure Creation", iterations, || {
        let mut interp = crate::interpreter::Interpreter::new(std::path::PathBuf::from("."));
        let _ = interp.run(r#"
            let make_adder = fn(x) { fn(y) { x + y } }
            let add5 = make_adder(5)
        "#);
    })
}

// GC Benchmarks

/// Benchmark GC allocation rate
pub fn bench_gc_allocation(iterations: usize) -> BenchmarkResult {
    benchmark("GC Allocation Rate", iterations, || {
        let config = crate::gc::GcConfig::builder()
            .strategy(crate::gc::GcStrategy::ReferenceCounting)
            .build();
        let mut gc = crate::gc::GarbageCollector::new(config);
        
        for _ in 0..100 {
            gc.allocate(64);
        }
    })
}

/// Benchmark GC mark-and-sweep
pub fn bench_gc_mark_sweep(iterations: usize) -> BenchmarkResult {
    benchmark("GC Mark-and-Sweep", iterations, || {
        let config = crate::gc::GcConfig::builder()
            .strategy(crate::gc::GcStrategy::MarkAndSweep)
            .build();
        let mut gc = crate::gc::GarbageCollector::new(config);
        
        let root = gc.allocate(100);
        gc.add_root(root);
        
        for _ in 0..50 {
            gc.allocate(50);
        }
        
        gc.collect();
    })
}

/// Benchmark GC generational
pub fn bench_gc_generational(iterations: usize) -> BenchmarkResult {
    benchmark("GC Generational", iterations, || {
        let mut gc = crate::gc::GarbageCollector::for_server();
        
        for _ in 0..100 {
            gc.allocate(1000);
        }
        
        gc.collect();
    })
}

/// Benchmark GC arena
pub fn bench_gc_arena(iterations: usize) -> BenchmarkResult {
    benchmark("GC Arena", iterations, || {
        let mut gc = crate::gc::GarbageCollector::for_cli();
        
        for _ in 0..100 {
            gc.allocate(100);
        }
        
        gc.reset_arena();
    })
}

/// Benchmark GC cycle detection
pub fn bench_gc_cycle_detection(iterations: usize) -> BenchmarkResult {
    benchmark("GC Cycle Detection", iterations, || {
        let config = crate::gc::GcConfig::builder()
            .strategy(crate::gc::GcStrategy::ReferenceCounting)
            .enable_cycle_detection(true)
            .build();
        let mut gc = crate::gc::GarbageCollector::new(config);
        
        let a = gc.allocate(50);
        let b = gc.allocate(50);
        
        gc.add_child(a, b);
        gc.add_child(b, a);
        
        gc.collect_full();
    })
}

// Security Benchmarks

/// Benchmark security validation
pub fn bench_security_validation(iterations: usize) -> BenchmarkResult {
    benchmark("Security Validation", iterations, || {
        let ctx = crate::safety::SecurityContext::new();
        let _ = ctx.validate_path("/safe/path");
        let _ = crate::safety::InputValidator::validate_string("safe input", 100);
    })
}

// Full Benchmark Suite

/// Run all benchmarks
pub fn run_all_benchmarks() {
    println!("Taurine Benchmark Suite");

    println!("Interpreter Benchmarks\n");
    bench_variable_allocation(100).print();
    bench_function_calls(100).print();
    bench_loop_performance(50).print();
    bench_array_operations(100).print();
    bench_table_operations(100).print();
    bench_string_operations(100).print();
    bench_math_operations(100).print();
    bench_closure_creation(100).print();

    println!("GC Benchmarks\n");
    bench_gc_allocation(100).print();
    bench_gc_mark_sweep(100).print();
    bench_gc_generational(100).print();
    bench_gc_arena(100).print();
    bench_gc_cycle_detection(100).print();

    println!("Security Benchmarks\n");
    bench_security_validation(1000).print();
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result() {
        let result = benchmark("Test", 10, || {
            let _ = 1 + 1;
        });
        
        assert_eq!(result.iterations, 10);
        assert!(result.total_time_ms > 0.0);
        assert!(result.avg_time_ms > 0.0);
        assert!(result.ops_per_second > 0.0);
    }

    #[test]
    fn test_bench_variable_allocation() {
        let result = bench_variable_allocation(10);
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_bench_function_calls() {
        let result = bench_function_calls(10);
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_bench_gc_allocation() {
        let result = bench_gc_allocation(10);
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_bench_gc_mark_sweep() {
        let result = bench_gc_mark_sweep(10);
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_bench_security_validation() {
        let result = bench_security_validation(100);
        assert!(result.total_time_ms > 0.0);
    }

    #[test]
    fn test_benchmark_print() {
        let result = benchmark("Test Print", 5, || {});
        let output = format!("{:?}", result);
        assert!(output.contains("Test Print"));
        assert!(output.contains("iterations"));
    }
}
