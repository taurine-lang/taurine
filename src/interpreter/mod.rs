//! Taurine Interpreter Module
//! This module provides the core interpreter for executing Taurine code.
//! It has been refactored into submodules for better maintainability:

pub mod native_functions;
pub mod execute;

// Re-export from execute module
pub use execute::Interpreter;

// Re-export types that were in the original file
pub use execute::StackFrame;
