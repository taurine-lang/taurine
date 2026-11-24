use std::path::PathBuf;
use std::time::Instant;
use taurine::Interpreter;

fn run_code(code: &str) -> Result<(), String> {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.run(code).map_err(|e| e.message())
}

#[test]
fn test_loop_performance_no_clone() {
    let start = Instant::now();
    let result = run_code(
        r#"
let sum = 0
for i in 1..10000 {
    sum = sum + i
}
"#,
    );
    let elapsed = start.elapsed();
    assert!(result.is_ok());
    // Should complete in reasonable time without cloning
    assert!(
        elapsed.as_millis() < 5000,
        "Loop took too long: {:?}",
        elapsed
    );
}

#[test]
fn test_nested_loops_performance() {
    let start = Instant::now();
    let result = run_code(
        r#"
let sum = 0
for i in 1..100 {
    for j in 1..100 {
        sum = sum + i * j
    }
}
"#,
    );
    let elapsed = start.elapsed();
    assert!(result.is_ok());
    assert!(
        elapsed.as_millis() < 10000,
        "Nested loops took too long: {:?}",
        elapsed
    );
}

#[test]
fn test_while_loop_performance() {
    let start = Instant::now();
    let result = run_code(
        r#"
let sum = 0
let i = 0
while i < 10000 {
    sum = sum + i
    i = i + 1
}
"#,
    );
    let elapsed = start.elapsed();
    assert!(result.is_ok());
    assert!(
        elapsed.as_millis() < 5000,
        "While loop took too long: {:?}",
        elapsed
    );
}
