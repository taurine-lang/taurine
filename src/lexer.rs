use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = ())]
pub enum TokenKind {
    // Ключевые слова
    #[token("loc")]
    Loc,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
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

    // Литералы и идентификаторы
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    #[regex(r#"f"([^"\\]|\\.)*""#)]
    FString,

    // Операторы
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

    // Скобки и разделители
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

    // Комментарии
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

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