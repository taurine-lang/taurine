use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = ())]
pub enum TokenKind {
    // Keywords
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("loc")]
    Loc,  // Deprecated: use 'let' instead
    #[token("function")]
    Function,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("throw")]
    Throw,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    
    // Classes and OOP
    #[token("class")]
    Class,
    #[token("extends")]
    Extends,
    #[token("this")]
    This,
    #[token("super")]
    Super,
    #[token("new")]
    New,
    
    // Modules
    #[token("export")]
    Export,
    #[token("require")]
    Require,
    
    // Pattern matching
    #[token("match")]
    Match,

    // Async/Await
    #[token("async")]
    Async,
    #[token("await")]
    Await,

    // Generators
    #[token("yield")]
    Yield,

    // Literals and identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 3)]
    Identifier,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    #[regex(r#"f"([^"\\]|\\.)*""#)]
    FString,

    // Operators
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("#")]
    Hash,
    #[token(":")]
    Colon,
    #[token("?.")]
    QuestionDot,
    #[token("??")]
    NullCoalesce,
    #[token("??=")]
    NullCoalesceAssign,
    #[token("=>")]
    FatArrow,
    #[token("...")]
    DotDotDot,

   // Brackets and separators
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,

    #[token("_")]
    Underscore,

    // Comments
    #[regex(r"//[^\n]*")]
    Comment,
    #[regex(r"[ \t\n\f\r]+")]
    Whitespace,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

/// Tokenize source code into a vector of tokens
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(source);
    let mut tokens = Vec::new();
    let mut line = 1;

    while let Some(result) = lexer.next() {
        if let Ok(kind) = result {
            if matches!(kind, TokenKind::Whitespace) {
                if lexer.slice().contains('\n') {
                    line += lexer.slice().matches('\n').count();
                }
                continue;
            }
            if matches!(kind, TokenKind::Comment) {
                continue;
            }
            tokens.push(Token {
                lexeme: lexer.slice().to_string(),
                kind,
                line,
            });
        }
    }
    tokens
}

/// Tokenize source code with string interning support
///
/// This function interns all identifier strings during tokenization,
/// which can improve performance for programs with many repeated identifiers.
pub fn tokenize_with_interner(
    source: &str,
    interner: &mut crate::string_intern::StringInterner,
) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(source);
    let mut tokens = Vec::new();
    let mut line = 1;

    while let Some(result) = lexer.next() {
        if let Ok(kind) = result {
            if matches!(kind, TokenKind::Whitespace) {
                if lexer.slice().contains('\n') {
                    line += lexer.slice().matches('\n').count();
                }
                continue;
            }
            if matches!(kind, TokenKind::Comment) {
                continue;
            }

            let lexeme = lexer.slice();
            
            // Intern identifiers for faster comparison
            if matches!(kind, TokenKind::Identifier) {
                let _id = interner.intern(lexeme);
            }
            
            tokens.push(Token {
                lexeme: lexeme.to_string(),
                kind,
                line,
            });
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        let tokens = tokenize("let const loc function if else return true false nil");
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Const);
        assert_eq!(tokens[2].kind, TokenKind::Loc);
        assert_eq!(tokens[3].kind, TokenKind::Function);
        assert_eq!(tokens[4].kind, TokenKind::If);
        assert_eq!(tokens[5].kind, TokenKind::Else);
        assert_eq!(tokens[6].kind, TokenKind::Return);
        assert_eq!(tokens[7].kind, TokenKind::True);
        assert_eq!(tokens[8].kind, TokenKind::False);
        assert_eq!(tokens[9].kind, TokenKind::Nil);
    }

    #[test]
    fn test_tokenize_new_keywords() {
        let tokens = tokenize("class extends this super new match export require");
        assert_eq!(tokens[0].kind, TokenKind::Class);
        assert_eq!(tokens[1].kind, TokenKind::Extends);
        assert_eq!(tokens[2].kind, TokenKind::This);
        assert_eq!(tokens[3].kind, TokenKind::Super);
        assert_eq!(tokens[4].kind, TokenKind::New);
        assert_eq!(tokens[5].kind, TokenKind::Match);
        assert_eq!(tokens[6].kind, TokenKind::Export);
        assert_eq!(tokens[7].kind, TokenKind::Require);
    }

    #[test]
    fn test_tokenize_control_flow() {
        let tokens = tokenize("while for in try catch throw break continue");
        assert_eq!(tokens[0].kind, TokenKind::While);
        assert_eq!(tokens[1].kind, TokenKind::For);
        assert_eq!(tokens[2].kind, TokenKind::In);
        assert_eq!(tokens[3].kind, TokenKind::Try);
        assert_eq!(tokens[4].kind, TokenKind::Catch);
        assert_eq!(tokens[5].kind, TokenKind::Throw);
        assert_eq!(tokens[6].kind, TokenKind::Break);
        assert_eq!(tokens[7].kind, TokenKind::Continue);
    }

    #[test]
    fn test_tokenize_operators() {
        let tokens = tokenize("= + - * / == != < > <= >= and or not");
        assert_eq!(tokens[0].kind, TokenKind::Equal);
        assert_eq!(tokens[1].kind, TokenKind::Plus);
        assert_eq!(tokens[2].kind, TokenKind::Minus);
        assert_eq!(tokens[3].kind, TokenKind::Star);
        assert_eq!(tokens[4].kind, TokenKind::Slash);
        assert_eq!(tokens[5].kind, TokenKind::EqualEqual);
        assert_eq!(tokens[6].kind, TokenKind::NotEqual);
        assert_eq!(tokens[7].kind, TokenKind::Less);
        assert_eq!(tokens[8].kind, TokenKind::Greater);
        assert_eq!(tokens[9].kind, TokenKind::LessEqual);
        assert_eq!(tokens[10].kind, TokenKind::GreaterEqual);
        assert_eq!(tokens[11].kind, TokenKind::And);
        assert_eq!(tokens[12].kind, TokenKind::Or);
        assert_eq!(tokens[13].kind, TokenKind::Not);
    }

    #[test]
    fn test_tokenize_new_operators() {
        let tokens = tokenize("?? ??= => ...");
        assert_eq!(tokens[0].kind, TokenKind::NullCoalesce);
        assert_eq!(tokens[1].kind, TokenKind::NullCoalesceAssign);
        assert_eq!(tokens[2].kind, TokenKind::FatArrow);
        assert_eq!(tokens[3].kind, TokenKind::DotDotDot);
    }

    #[test]
    fn test_tokenize_punctuation() {
        let tokens = tokenize("( ) { } [ ] , ; . .. : #");
        assert_eq!(tokens[0].kind, TokenKind::LParen);
        assert_eq!(tokens[1].kind, TokenKind::RParen);
        assert_eq!(tokens[2].kind, TokenKind::LBrace);
        assert_eq!(tokens[3].kind, TokenKind::RBrace);
        assert_eq!(tokens[4].kind, TokenKind::LBracket);
        assert_eq!(tokens[5].kind, TokenKind::RBracket);
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        assert_eq!(tokens[7].kind, TokenKind::Semicolon);
        assert_eq!(tokens[8].kind, TokenKind::Dot);
        assert_eq!(tokens[9].kind, TokenKind::DotDot);
        assert_eq!(tokens[10].kind, TokenKind::Colon);
        assert_eq!(tokens[11].kind, TokenKind::Hash);
    }

    #[test]
    fn test_tokenize_identifiers() {
        let tokens = tokenize("x foo my_var _private camelCase PascalCase");
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "x");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "foo");
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "my_var");
    }

    #[test]
    fn test_tokenize_numbers() {
        let tokens = tokenize("42 3.14 0 100 0.5");
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[0].lexeme, "42");
        assert_eq!(tokens[1].kind, TokenKind::Number);
        assert_eq!(tokens[1].lexeme, "3.14");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].lexeme, "0");
    }

    #[test]
    fn test_tokenize_strings() {
        let tokens = tokenize(r#""hello" "world" "test\nstring""#);
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].lexeme, "\"hello\"");
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[1].lexeme, "\"world\"");
    }

    #[test]
    fn test_tokenize_fstrings() {
        let tokens = tokenize(r#"f"hello {name}" f"value: {x + y}""#);
        assert_eq!(tokens[0].kind, TokenKind::FString);
        assert_eq!(tokens[1].kind, TokenKind::FString);
    }

    #[test]
    fn test_tokenize_comments() {
        let tokens = tokenize("let x = 10 // this is a comment\nlet y = 20");
        // Comments should be skipped
        assert_eq!(tokens.iter().filter(|t| t.kind == TokenKind::Let).count(), 2);
        assert_eq!(tokens.iter().filter(|t| t.kind == TokenKind::Identifier).count(), 2);
    }

    #[test]
    fn test_tokenize_whitespace() {
        let tokens = tokenize("  let   x  =  10  \n  let y = 20  ");
        // Whitespace should be skipped
        assert!(!tokens.iter().any(|t| t.kind == TokenKind::Whitespace));
    }

    #[test]
    fn test_tokenize_line_numbers() {
        let tokens = tokenize("let x = 10\nlet y = 20\nlet z = 30");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[4].line, 2);  // Second 'let' is on line 2
        assert_eq!(tokens[8].line, 3);  // Third 'let' is on line 3
    }

    #[test]
    fn test_tokenize_nil_safe_operators() {
        let tokens = tokenize("?.");
        assert_eq!(tokens[0].kind, TokenKind::QuestionDot);
    }

    #[test]
    fn test_tokenize_import_as() {
        let tokens = tokenize("import \"module.tau\" as mod");
        assert_eq!(tokens[0].kind, TokenKind::Import);
        assert_eq!(tokens[1].kind, TokenKind::String);
        assert_eq!(tokens[2].kind, TokenKind::As);
        assert_eq!(tokens[3].kind, TokenKind::Identifier);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let tokens = tokenize("let result = (a + b) * c ?? default");
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[3].kind, TokenKind::LParen);
        assert_eq!(tokens[4].kind, TokenKind::Identifier);
        assert_eq!(tokens[5].kind, TokenKind::Plus);
        assert_eq!(tokens[6].kind, TokenKind::Identifier);
        assert_eq!(tokens[7].kind, TokenKind::RParen);
        assert_eq!(tokens[8].kind, TokenKind::Star);
        assert_eq!(tokens[9].kind, TokenKind::Identifier);
        assert_eq!(tokens[10].kind, TokenKind::NullCoalesce);
        assert_eq!(tokens[11].kind, TokenKind::Identifier);
    }

    #[test]
    fn test_tokenize_lambda() {
        let tokens = tokenize("let add = (x, y) => x + y");
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[2].kind, TokenKind::Equal);
        assert_eq!(tokens[3].kind, TokenKind::LParen);
        assert_eq!(tokens[8].kind, TokenKind::FatArrow);
    }

    #[test]
    fn test_tokenize_class_definition() {
        let tokens = tokenize("class Dog extends Animal { function bark() { } }");
        assert_eq!(tokens[0].kind, TokenKind::Class);
        assert_eq!(tokens[2].kind, TokenKind::Extends);
        assert_eq!(tokens[5].kind, TokenKind::Function);
    }

    #[test]
    fn test_tokenize_match_expression() {
        let tokens = tokenize("match x { 0 => \"zero\", _ => \"other\" }");
        assert_eq!(tokens[0].kind, TokenKind::Match);
        assert_eq!(tokens[2].kind, TokenKind::LBrace);
        assert_eq!(tokens[4].kind, TokenKind::FatArrow);
        assert_eq!(tokens[8].kind, TokenKind::FatArrow);
    }

    #[test]
    fn test_tokenize_empty_input() {
        let tokens = tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_only_whitespace() {
        let tokens = tokenize("   \n\t  \n  ");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_spread_operator() {
        let tokens = tokenize("...args");
        assert_eq!(tokens[0].kind, TokenKind::DotDotDot);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
    }

    #[test]
    fn test_tokenize_async_keywords() {
        let tokens = tokenize("async await yield");
        assert_eq!(tokens[0].kind, TokenKind::Async);
        assert_eq!(tokens[1].kind, TokenKind::Await);
        assert_eq!(tokens[2].kind, TokenKind::Yield);
    }

    #[test]
    fn test_tokenize_async_function() {
        let tokens = tokenize("async function fetch() { await promise }");
        assert_eq!(tokens[0].kind, TokenKind::Async);
        assert_eq!(tokens[1].kind, TokenKind::Function);
        assert_eq!(tokens[6].kind, TokenKind::Await);
    }
}