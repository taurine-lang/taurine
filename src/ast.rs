use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    FString {
        parts: Vec<(String, Option<Box<Expr>>)>,
        line: usize,
    },
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr>,
        line: usize,
    },
    Unary {
        op: TokenKind,
        expr: Box<Expr>,
        line: usize,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        line: usize,
    },
    LiteralTrue,
    LiteralFalse,
    LiteralNil,
    Table {
        entries: Vec<(String, Expr)>,
        line: usize,
    },
    Array {
        items: Vec<Expr>,
        line: usize,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        line: usize,
    },
    SetIndex {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
        line: usize,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        line: usize,
    },
    Length {
        expr: Box<Expr>,
        line: usize,
    },
    Get {
        object: Box<Expr>,
        name: String,
        line: usize,
    },
    SafeGet {
        object: Box<Expr>,
        name: String,
        line: usize,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
        line: usize,
    },
    Throw {
        expr: Box<Expr>,
        line: usize,
    },
    FunctionLiteral {
        params: Vec<(String, Option<Expr>)>,
        body: Vec<Stmt>,
        line: usize,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Declaration {
        name: String,
        initializer: Option<Expr>,
        line: usize,
        is_const: bool,
    },
    Destructure {
        names: Vec<String>,
        initializer: Expr,
        line: usize,
    },
    Assignment {
        name: String,
        value: Expr,
        line: usize,
        is_const_assign: bool,
    },
    Expression(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        line: usize,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    For {
        initializer: Option<Box<Expr>>,
        condition: Box<Expr>,
        increment: Option<Box<Expr>>,
        body: Vec<Stmt>,
        line: usize,
    },
    ForIn {
        variable: String,
        iterable: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    Function {
        name: String,
        params: Vec<(String, Option<Expr>)>,
        body: Vec<Stmt>,
        line: usize,
    },
    Return(Option<Expr>),
    ReturnMulti(Vec<Expr>),
    Block(Vec<Stmt>),
    Import {
        path: String,
        alias: Option<String>,
        line: usize,
    },
    Try {
        body: Vec<Stmt>,
        catch_var: Option<String>,
        catch_body: Vec<Stmt>,
        line: usize,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}