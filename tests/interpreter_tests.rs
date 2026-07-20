use taurine::lexer::tokenize;
use taurine::parser::Parser;
use taurine::ast::{Stmt, Expr};

fn parse(code: &str) -> Result<Vec<Stmt>, String> {
    let tokens = tokenize(code);
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    Ok(program.statements)
}

#[test]
fn test_variable_declaration() {
    let stmts = parse("let x = 42").unwrap();
    assert_eq!(stmts.len(), 1);
    assert!(matches!(&stmts[0], Stmt::Declaration { .. }));
}

#[test]
fn test_const_declaration() {
    let stmts = parse("const PI = 3.14").unwrap();
    match &stmts[0] {
        Stmt::Declaration { is_const, .. } => assert!(is_const),
        _ => panic!("Expected Declaration"),
    }
}

#[test]
fn test_function_declaration() {
    let stmts = parse("function add(a, b) { return a + b }").unwrap();
    match &stmts[0] {
        Stmt::Function { params, body, .. } => {
            assert_eq!(params.len(), 2);
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected Function"),
    }
}

#[test]
fn test_if_statement() {
    let stmts = parse("if x > 5 { print(x) }").unwrap();
    assert!(matches!(&stmts[0], Stmt::If { .. }));
}

#[test]
fn test_while_loop() {
    let stmts = parse("while x < 10 { x = x + 1 }").unwrap();
    assert!(matches!(&stmts[0], Stmt::While { .. }));
}

#[test]
fn test_for_in_loop() {
    let stmts = parse("for i in 1..10 { print(i) }").unwrap();
    assert!(matches!(&stmts[0], Stmt::ForIn { .. }));
}

#[test]
fn test_array_literal() {
    let stmts = parse("let arr = [1, 2, 3]").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Array { items, .. }), .. } => {
            assert_eq!(items.len(), 3);
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn test_table_literal() {
    let stmts = parse(r#"let obj = { name: "test", value: 42 }"#).unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Table { entries, .. }), .. } => {
            assert_eq!(entries.len(), 2);
        }
        _ => panic!("Expected Table"),
    }
}

#[test]
fn test_lambda() {
    let stmts = parse("let add = (x, y) => x + y").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Lambda { params, .. }), .. } => {
            assert_eq!(params.len(), 2);
        }
        _ => panic!("Expected Lambda"),
    }
}

#[test]
fn test_class_declaration() {
    let stmts = parse("class Dog { function bark() { print(\"Woof\") } }").unwrap();
    assert!(matches!(&stmts[0], Stmt::Class { .. }));
}

#[test]
fn test_try_catch() {
    let stmts = parse("try { risky() } catch (e) { print(e) }").unwrap();
    assert!(matches!(&stmts[0], Stmt::Try { .. }));
}

#[test]
fn test_import() {
    let stmts = parse(r#"import "module.tau""#).unwrap();
    assert!(matches!(&stmts[0], Stmt::Import { .. }));
}

#[test]
fn test_match_expression() {
    let stmts = parse(r#"let result = match x { 0 => "zero", _ => "other" }"#).unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Match { arms, .. }), .. } => {
            assert_eq!(arms.len(), 2);
        }
        _ => panic!("Expected Match"),
    }
}

#[test]
fn test_async_function() {
    let stmts = parse("async function fetchData() { return data }").unwrap();
    assert!(matches!(&stmts[0], Stmt::AsyncFunction { .. }));
}

#[test]
fn test_null_coalesce() {
    let stmts = parse("let x = a ?? b").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::NullCoalesce { .. }), .. } => {}
        _ => panic!("Expected NullCoalesce"),
    }
}

#[test]
fn test_safe_navigation() {
    let stmts = parse("let x = obj?.prop").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::SafeGet { .. }), .. } => {}
        _ => panic!("Expected SafeGet"),
    }
}

#[test]
fn test_spread_operator() {
    let stmts = parse("let arr = [...other, 4, 5]").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Array { items, .. }), .. } => {
            assert!(matches!(&items[0], Expr::Spread { .. }));
        }
        _ => panic!("Expected Array with Spread"),
    }
}

#[test]
fn test_return_multi() {
    let stmts = parse("function f() { return 1, 2, 3 }").unwrap();
    match &stmts[0] {
        Stmt::Function { body, .. } => {
            assert!(matches!(&body[0], Stmt::ReturnMulti(_)));
        }
        _ => panic!("Expected Function"),
    }
}

#[test]
fn test_fstring() {
    let stmts = parse(r#"let s = f"Hello {name}""#).unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::FString { .. }), .. } => {}
        _ => panic!("Expected FString"),
    }
}

#[test]
fn test_range() {
    let stmts = parse("let r = 1..10").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Range { .. }), .. } => {}
        _ => panic!("Expected Range"),
    }
}

#[test]
fn test_length_operator() {
    let stmts = parse("let len = #arr").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Length { .. }), .. } => {}
        _ => panic!("Expected Length"),
    }
}

#[test]
fn test_empty_program() {
    let stmts = parse("").unwrap();
    assert_eq!(stmts.len(), 0);
}

#[test]
fn test_multiple_statements() {
    let stmts = parse("let x = 1\nlet y = 2\nlet z = 3").unwrap();
    assert_eq!(stmts.len(), 3);
}

#[test]
fn test_default_parameters() {
    let stmts = parse("function f(x = 10) { return x }").unwrap();
    match &stmts[0] {
        Stmt::Function { params, .. } => {
            assert!(params[0].1.is_some());
        }
        _ => panic!("Expected Function with default param"),
    }
}

#[test]
fn test_await() {
    let stmts = parse("let x = await promise").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::Await { .. }), .. } => {}
        _ => panic!("Expected Await"),
    }
}

#[test]
fn test_new_instance() {
    let stmts = parse("let obj = new MyClass()").unwrap();
    match &stmts[0] {
        Stmt::Declaration { initializer: Some(Expr::NewInstance { .. }), .. } => {}
        _ => panic!("Expected NewInstance"),
    }
}

#[test]
fn test_this() {
    let stmts = parse("function f() { return this }").unwrap();
    match &stmts[0] {
        Stmt::Function { body, .. } => {
            match &body[0] {
                Stmt::Return(Some(Expr::This { .. })) => {}
                _ => panic!("Expected This"),
            }
        }
        _ => panic!("Expected Function"),
    }
}