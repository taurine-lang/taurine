//! Taurine - Fast, embeddable scripting language
//!
//! This library provides the core Taurine interpreter and API for embedding
//! Taurine in Rust applications.
//!
//! # Example
//!
//! ```rust,no_run
//! use taurine::Interpreter;
//!
//! fn main() -> Result<(), String> {
//!     let mut interp = Interpreter::new();
//!     interp.run(r#"
//!         let x = 10
//!         let y = 20
//!         print(f"x + y = {x + y}")
//!     "#)?;
//!     Ok(())
//! }
//! ```

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod value;
pub mod environment;
pub mod interpreter;
pub mod optimizer;
pub mod formatter;
pub mod ffi;
pub mod error;

pub use interpreter::Interpreter;
pub use value::Value;
pub use environment::Environment;
pub use ast::{Expr, Stmt, Program};
pub use parser::Parser;
pub use lexer::{tokenize, Token, TokenKind};
pub use optimizer::Optimizer;
pub use formatter::Formatter;
pub use error::TaurineError;
