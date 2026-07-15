use taurine::Interpreter;
use std::path::PathBuf;

fn run_code(code: &str) -> Result<(), String> {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.run(code).map_err(|e| e.message())
}

#[test]
fn test_hello_world() {
    assert!(run_code(r#"print("Hello, World!")"#).is_ok());
}

#[test]
fn test_variables() {
    assert!(run_code(r#"
        let x = 10
        let y = 20
        let sum = x + y
        print(sum)
    "#).is_ok());
}

#[test]
fn test_arrays() {
    assert!(run_code(r#"
        let arr = [1, 2, 3, 4, 5]
        print(arr)
        print(#arr)
    "#).is_ok());
}

#[test]
fn test_tables() {
    assert!(run_code(r#"
        let obj = { name: "test", value: 42 }
        print(obj.name)
    "#).is_ok());
}

#[test]
fn test_functions() {
    assert!(run_code(r#"
        function add(a, b) {
            return a + b
        }
        let result = add(10, 20)
        print(result)
    "#).is_ok());
}

#[test]
fn test_loops() {
    assert!(run_code(r#"
        let sum = 0
        for i in 1..11 {
            sum = sum + i
        }
        print(sum)
    "#).is_ok());
}

#[test]
fn test_json() {
    assert!(run_code(r#"
        let data = json_parse('{"name": "test", "value": 42}')
        print(data.name)
    "#).is_ok());
}

#[test]
fn test_crypto() {
    assert!(run_code(r#"
        let hash = crypto_md5("hello")
        print(hash)
    "#).is_ok());
}

#[test]
fn test_date() {
    assert!(run_code(r#"
        let now = date_now()
        print(now)
    "#).is_ok());
}

#[test]
fn test_regex() {
    assert!(run_code(r#"
        let matches = regex_match("\\d+", "abc123")
        print(matches)
    "#).is_ok());
}

#[test]
fn test_fibonacci() {
    assert!(run_code(r#"
        function fib(n) {
            if n <= 1 { return n }
            return fib(n - 1) + fib(n - 2)
        }
        let result = fib(10)
        print(result)
    "#).is_ok());
}