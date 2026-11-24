//! Taurine Interpreter Module
//! This module provides the core interpreter for executing Taurine code.
//! It has been refactored into submodules for better maintainability:

pub mod interner_setup;
pub mod native_functions;

pub mod execute;

pub use execute::{ControlFlow, Interpreter, StackFrame};

pub use interner_setup::create_default_interner;
