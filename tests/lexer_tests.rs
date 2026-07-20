use taurine::lexer::{tokenize, tokenize_with_interner, TokenKind};
use taurine::string_intern::StringInterner;

#[test]
fn test_keywords() {
    let tokens = tokenize("let const function if else return");
    assert_eq!(tokens[0].kind, TokenKind::Let);
    assert_eq!(tokens[1].kind, TokenKind::Const);
    assert_eq!(tokens[2].kind, TokenKind::Function);
    assert_eq!(tokens[3].kind, TokenKind::If);
    assert_eq!(tokens[4].kind, TokenKind::Else);
    assert_eq!(tokens[5].kind, TokenKind::Return);
}

#[test]
fn test_operators() {
    let tokens = tokenize("+ - * / == != < > <= >=");
    assert_eq!(tokens[0].kind, TokenKind::Plus);
    assert_eq!(tokens[1].kind, TokenKind::Minus);
    assert_eq!(tokens[2].kind, TokenKind::Star);
    assert_eq!(tokens[3].kind, TokenKind::Slash);
    assert_eq!(tokens[4].kind, TokenKind::EqualEqual);
    assert_eq!(tokens[5].kind, TokenKind::NotEqual);
}

#[test]
fn test_modern_operators() {
    let tokens = tokenize("?. ?? ??= => ...");
    assert_eq!(tokens[0].kind, TokenKind::QuestionDot);
    assert_eq!(tokens[1].kind, TokenKind::NullCoalesce);
    assert_eq!(tokens[2].kind, TokenKind::NullCoalesceAssign);
    assert_eq!(tokens[3].kind, TokenKind::FatArrow);
    assert_eq!(tokens[4].kind, TokenKind::DotDotDot);
}

#[test]
fn test_numbers() {
    let tokens = tokenize("42 3.14 0 100 0.5");
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].lexeme, "42");
    assert_eq!(tokens[1].kind, TokenKind::Number);
    assert_eq!(tokens[1].lexeme, "3.14");
}

#[test]
fn test_strings() {
    let tokens = tokenize(r#""hello" "world""#);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[1].kind, TokenKind::String);
}

#[test]
fn test_fstrings() {
    let tokens = tokenize(r#"f"hello {name}" f"value: {x + y}""#);
    assert_eq!(tokens[0].kind, TokenKind::FString);
    assert_eq!(tokens[1].kind, TokenKind::FString);
}

#[test]
fn test_identifiers() {
    let tokens = tokenize("x foo my_var _private camelCase");
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].lexeme, "x");
    assert_eq!(tokens[1].lexeme, "foo");
    assert_eq!(tokens[2].lexeme, "my_var");
}

#[test]
fn test_comments() {
    let tokens = tokenize("let x = 10 // comment\nlet y = 20");
    assert_eq!(tokens.iter().filter(|t| t.kind == TokenKind::Let).count(), 2);
}

#[test]
fn test_line_numbers() {
    let tokens = tokenize("let x = 10\nlet y = 20\nlet z = 30");
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[4].line, 2);
    assert_eq!(tokens[8].line, 3);
}

#[test]
fn test_string_interning() {
    let mut interner = StringInterner::new();
    let _tokens = tokenize_with_interner("let x = 10 let y = x", &mut interner);
    assert!(interner.contains("x"));
    assert!(interner.contains("y"));
}

#[test]
fn test_async_keywords() {
    let tokens = tokenize("async await yield");
    assert_eq!(tokens[0].kind, TokenKind::Async);
    assert_eq!(tokens[1].kind, TokenKind::Await);
    assert_eq!(tokens[2].kind, TokenKind::Yield);
}

#[test]
fn test_class_keywords() {
    let tokens = tokenize("class extends this super new");
    assert_eq!(tokens[0].kind, TokenKind::Class);
    assert_eq!(tokens[1].kind, TokenKind::Extends);
    assert_eq!(tokens[2].kind, TokenKind::This);
    assert_eq!(tokens[3].kind, TokenKind::Super);
    assert_eq!(tokens[4].kind, TokenKind::New);
}

#[test]
fn test_empty_input() {
    let tokens = tokenize("");
    assert!(tokens.is_empty());
}

#[test]
fn test_whitespace_only() {
    let tokens = tokenize("  \n\t  ");
    assert!(tokens.is_empty());
}