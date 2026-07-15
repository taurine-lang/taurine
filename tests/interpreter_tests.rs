use taurine::Interpreter;
use taurine::Value;
use std::path::PathBuf;

fn run(code: &str) -> Result<(), String> {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.run(code).map_err(|e| e.message())
}

fn run_and_get(code: &str, var: &str) -> Result<Value, String> {
    let mut interp = Interpreter::new(PathBuf::from("."));
    interp.run(code).map_err(|e| e.message())?;
    interp.get(var)
}

#[test]
fn test_arithmetic() {
    assert!(run("let x = 10 + 20").is_ok());
    assert!(run("let x = 100 - 50").is_ok());
    assert!(run("let x = 10 * 5").is_ok());
    assert!(run("let x = 100 / 4").is_ok());
    assert!(run("let x = 10 % 3").is_ok());
}

#[test]
fn test_string_concatenation() {
    assert!(run(r#"let s = "Hello" + " " + "World""#).is_ok());
}

#[test]
fn test_comparison() {
    assert!(run("let x = 10 > 5").is_ok());
    assert!(run("let x = 10 < 20").is_ok());
    assert!(run("let x = 10 == 10").is_ok());
    assert!(run("let x = 10 != 5").is_ok());
}

#[test]
fn test_logical_operators() {
    assert!(run("let x = true and false").is_ok());
    assert!(run("let x = true or false").is_ok());
    assert!(run("let x = not true").is_ok());
}

#[test]
fn test_variables() {
    let val = run_and_get("let x = 42", "x").unwrap();
    assert_eq!(val, Value::Number(42.0));
}

#[test]
fn test_const() {
    assert!(run("const PI = 3.14").is_ok());
}

#[test]
fn test_arrays() {
    assert!(run("let arr = [1, 2, 3, 4, 5]").is_ok());
    let val = run_and_get("let arr = [1, 2, 3]", "arr").unwrap();
    assert!(matches!(val, Value::Array(_)));
}

#[test]
fn test_tables() {
    assert!(run(r#"let obj = { name: "test", value: 42 }"#).is_ok());
    let val = run_and_get(r#"let obj = { name: "test" }"#, "obj").unwrap();
    assert!(matches!(val, Value::Table(_)));
}

#[test]
fn test_functions() {
    assert!(run(r#"
        function add(a, b) { return a + b }
        let result = add(10, 20)
    "#).is_ok());
}

#[test]
fn test_recursion() {
    let val = run_and_get(r#"
        function factorial(n) {
            if n <= 1 { return 1 }
            return n * factorial(n - 1)
        }
        let result = factorial(5)
    "#, "result").unwrap();
    assert_eq!(val, Value::Number(120.0));
}

#[test]
fn test_closures() {
    let val = run_and_get(r#"
        function makeAdder(x) {
            return function(y) { return x + y }
        }
        let add5 = makeAdder(5)
        let result = add5(10)
    "#, "result").unwrap();
    assert_eq!(val, Value::Number(15.0));
}

#[test]
fn test_loops() {
    let val = run_and_get(r#"
        let sum = 0
        for i in 1..11 {
            sum = sum + i
        }
    "#, "sum").unwrap();
    assert_eq!(val, Value::Number(55.0));
}

#[test]
fn test_while_loop() {
    let val = run_and_get(r#"
        let x = 0
        while x < 10 {
            x = x + 1
        }
    "#, "x").unwrap();
    assert_eq!(val, Value::Number(10.0));
}

#[test]
fn test_if_else() {
    let val = run_and_get(r#"
        let x = 10
        let result = 0
        if x > 5 {
            result = 1
        } else {
            result = 2
        }
    "#, "result").unwrap();
    assert_eq!(val, Value::Number(1.0));
}

#[test]
fn test_null_coalesce() {
    let val = run_and_get(r#"
        let x = nil
        let y = x ?? 42
    "#, "y").unwrap();
    assert_eq!(val, Value::Number(42.0));
}

#[test]
fn test_safe_navigation() {
    let val = run_and_get(r#"
        let obj = { name: "test" }
        let x = obj?.name
    "#, "x").unwrap();
    assert_eq!(val, Value::String("test".to_string()));
}

#[test]
fn test_safe_navigation_nil() {
    let val = run_and_get(r#"
        let obj = nil
        let x = obj?.name
    "#, "x").unwrap();
    assert_eq!(val, Value::Nil);
}

#[test]
fn test_lambda() {
    let val = run_and_get(r#"
        let add = (x, y) => x + y
        let result = add(10, 20)
    "#, "result").unwrap();
    assert_eq!(val, Value::Number(30.0));
}

#[test]
fn test_default_parameters() {
    let val = run_and_get(r#"
        function greet(name = "World") {
            return "Hello, " + name
        }
        let result = greet()
    "#, "result").unwrap();
    assert_eq!(val, Value::String("Hello, World".to_string()));
}

#[test]
fn test_array_operations() {
    assert!(run(r#"
        let arr = [1, 2, 3]
        io_arraypush(arr, 4)
        let len = io_arraylen(arr)
    "#).is_ok());
}

#[test]
fn test_table_operations() {
    assert!(run(r#"
        let obj = {}
        obj.name = "test"
        obj.value = 42
    "#).is_ok());
}

#[test]
fn test_string_operations() {
    assert!(run(r#"
        let s = "Hello, World!"
        let upper = io_strupper(s)
        let lower = io_strlower(s)
    "#).is_ok());
}

#[test]
fn test_json_parse() {
    let val = run_and_get(r#"
        let data = json_parse('{"name": "test", "value": 42}')
    "#, "data").unwrap();
    assert!(matches!(val, Value::Table(_)));
}

#[test]
fn test_json_stringify() {
    assert!(run(r#"
        let data = { name: "test", value: 42 }
        let json = json_stringify(data)
    "#).is_ok());
}

#[test]
fn test_try_catch() {
    assert!(run(r#"
        try {
            throw "error"
        } catch (e) {
            print(e)
        }
    "#).is_ok());
}

#[test]
fn test_break() {
    let val = run_and_get(r#"
        let sum = 0
        for i in 1..100 {
            if i > 5 { break }
            sum = sum + i
        }
    "#, "sum").unwrap();
    assert_eq!(val, Value::Number(15.0));
}

#[test]
fn test_continue() {
    let val = run_and_get(r#"
        let sum = 0
        for i in 1..11 {
            if i % 2 == 0 { continue }
            sum = sum + i
        }
    "#, "sum").unwrap();
    assert_eq!(val, Value::Number(25.0)); // 1+3+5+7+9
}

#[test]
fn test_range() {
    assert!(run(r#"
        let r = 1..10
        for i in r {
            print(i)
        }
    "#).is_ok());
}

#[test]
fn test_length_operator() {
    let val = run_and_get(r#"
        let arr = [1, 2, 3, 4, 5]
        let len = #arr
    "#, "len").unwrap();
    assert_eq!(val, Value::Number(5.0));
}

#[test]
fn test_fstring() {
    let val = run_and_get(r#"
        let name = "Taurine"
        let msg = f"Hello, {name}!"
    "#, "msg").unwrap();
    assert_eq!(val, Value::String("Hello, Taurine!".to_string()));
}

#[test]
fn test_class() {
    assert!(run(r#"
        class Dog {
            function bark() {
                return "Woof!"
            }
        }
        let dog = new Dog()
        let sound = dog:bark()
    "#).is_ok());
}

#[test]
fn test_class_inheritance() {
    assert!(run(r#"
        class Animal {
            function speak() {
                return "Some sound"
            }
        }
        class Dog extends Animal {
            function speak() {
                return "Woof!"
            }
        }
        let dog = new Dog()
    "#).is_ok());
}

#[test]
fn test_match_expression() {
    let val = run_and_get(r#"
        let x = 42
        let result = match x {
            0 => "zero",
            1 => "one",
            n if n > 0 => "positive",
            _ => "negative"
        }
    "#, "result").unwrap();
    assert_eq!(val, Value::String("positive".to_string()));
}

#[test]
fn test_destructuring() {
    assert!(run(r#"
        let arr = [1, 2, 3]
        let [a, b, c] = arr
    "#).is_ok());
}

#[test]
fn test_spread_operator() {
    assert!(run(r#"
        let arr1 = [1, 2, 3]
        let arr2 = [...arr1, 4, 5, 6]
    "#).is_ok());
}

#[test]
fn test_multi_return() {
    let val = run_and_get(r#"
        function divmod(a, b) {
            return a / b, a % b
        }
        let [q, r] = divmod(10, 3)
    "#, "q").unwrap();
    assert_eq!(val, Value::Number(3.3333333333333335));
}

#[test]
fn test_async_function() {
    assert!(run(r#"
        async function fetchData() {
            return "data"
        }
        let result = fetchData()
    "#).is_ok());
}

#[test]
fn test_generator() {
    assert!(run(r#"
        generator count() {
            yield 1
            yield 2
            yield 3
        }
        let gen = count()
    "#).is_ok());
}

#[test]
fn test_import() {
    // Import should work without errors
    assert!(run(r#"
        import "std/math.tau" as math
    "#).is_ok());
}

#[test]
fn test_error_handling() {
    let result = run("undefined_var + 1");
    assert!(result.is_err());
}

#[test]
fn test_division_by_zero() {
    let result = run("let x = 10 / 0");
    assert!(result.is_err());
}

#[test]
fn test_type_error() {
    let result = run(r#"let x = "hello" + 5"#);
    assert!(result.is_err());
}

#[test]
fn test_stack_overflow() {
    let result = run(r#"
        function infinite() {
            return infinite()
        }
        infinite()
    "#);
    assert!(result.is_err());
}

#[test]
fn test_complex_program() {
    assert!(run(r#"
        // Fibonacci
        function fib(n) {
            if n <= 1 { return n }
            return fib(n - 1) + fib(n - 2)
        }
        
        // Array operations
        let arr = []
        for i in 1..11 {
            io_arraypush(arr, fib(i))
        }
        
        // Table operations
        let results = {}
        for i in 1..11 {
            results["fib_" + tostring(i)] = fib(i)
        }
        
        // String operations
        let msg = f"Fibonacci sequence: {arr}"
        
        print(msg)
    "#).is_ok());
}