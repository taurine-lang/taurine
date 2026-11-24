pub mod arena;
pub mod ast;
#[cfg(feature = "async")]
pub mod async_rt;
pub mod benchmark;
pub mod bytecode;
pub mod environment;
pub mod error;
pub mod ffi;
pub mod formatter;
pub mod gc;
pub mod interpreter;
pub mod lexer;
pub mod optimizer;
pub mod parser;
pub mod safety;
pub mod string_intern;
pub mod value;

pub use arena::AstArena;
pub use ast::{Expr, Program, Stmt};
#[cfg(feature = "async")]
pub use async_rt::{AsyncRuntime, FutureValue};
pub use benchmark::{run_all_benchmarks, BenchmarkResult};
pub use bytecode::{
    BytecodeFunction, BytecodeProgram, Compiler, Instruction, OpCode, VirtualMachine,
};
pub use environment::Environment;
pub use error::TaurineError;
pub use formatter::Formatter;
pub use gc::{GarbageCollector, GcConfig, GcPtr, GcStats, GcStrategy};
pub use interpreter::execute::ControlFlow;
pub use interpreter::execute::StackFrame;
pub use interpreter::Interpreter;
pub use lexer::{tokenize, tokenize_with_interner, Token, TokenKind};
pub use optimizer::Optimizer;
pub use parser::Parser;
pub use safety::{
    InputValidator, Permissions, ResourceTracker, SafetyContext, SafetyLimits, SecurityError,
    SecurityLevel,
};
pub use string_intern::{InternedString, StringInterner};
pub use value::Value;
