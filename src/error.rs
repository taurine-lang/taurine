//! Error types for Taurine

use std::fmt;

/// Taurine error types
#[derive(Debug, Clone)]
pub enum TaurineError {
    /// Lexer error
    Lexer {
        message: String,
        line: usize,
    },
    /// Parser error
    Parse {
        message: String,
        line: usize,
    },
    /// Runtime error
    Runtime {
        message: String,
        line: usize,
    },
    /// Variable not found
    UndefinedVariable {
        name: String,
        line: usize,
    },
    /// Cannot assign to const
    ConstAssignment {
        name: String,
        line: usize,
    },
    /// Type error
    Type {
        message: String,
        line: usize,
    },
    /// Index out of bounds
    IndexOutOfBounds {
        index: usize,
        length: usize,
        line: usize,
    },
    /// Field not found in table
    FieldNotFound {
        field: String,
        line: usize,
    },
    /// Function call error
    Call {
        message: String,
        line: usize,
    },
    /// Import error
    Import {
        path: String,
        message: String,
        line: usize,
    },
}

impl TaurineError {
    /// Create a lexer error
    pub fn lexer(message: impl Into<String>, line: usize) -> Self {
        TaurineError::Lexer {
            message: message.into(),
            line,
        }
    }

    /// Create a parse error
    pub fn parse(message: impl Into<String>, line: usize) -> Self {
        TaurineError::Parse {
            message: message.into(),
            line,
        }
    }

    /// Create a runtime error
    pub fn runtime(message: impl Into<String>, line: usize) -> Self {
        TaurineError::Runtime {
            message: message.into(),
            line,
        }
    }

    /// Create an undefined variable error
    pub fn undefined_variable(name: impl Into<String>, line: usize) -> Self {
        TaurineError::UndefinedVariable {
            name: name.into(),
            line,
        }
    }

    /// Create a const assignment error
    pub fn const_assignment(name: impl Into<String>, line: usize) -> Self {
        TaurineError::ConstAssignment {
            name: name.into(),
            line,
        }
    }

    /// Get the line number where the error occurred
    pub fn line(&self) -> usize {
        match self {
            TaurineError::Lexer { line, .. } => *line,
            TaurineError::Parse { line, .. } => *line,
            TaurineError::Runtime { line, .. } => *line,
            TaurineError::UndefinedVariable { line, .. } => *line,
            TaurineError::ConstAssignment { line, .. } => *line,
            TaurineError::Type { line, .. } => *line,
            TaurineError::IndexOutOfBounds { line, .. } => *line,
            TaurineError::FieldNotFound { line, .. } => *line,
            TaurineError::Call { line, .. } => *line,
            TaurineError::Import { line, .. } => *line,
        }
    }

    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            TaurineError::Lexer { message, .. } => message.clone(),
            TaurineError::Parse { message, .. } => message.clone(),
            TaurineError::Runtime { message, .. } => message.clone(),
            TaurineError::UndefinedVariable { name, .. } => {
                format!("Undefined variable: {}", name)
            }
            TaurineError::ConstAssignment { name, .. } => {
                format!("Cannot assign to const '{}'", name)
            }
            TaurineError::Type { message, .. } => message.clone(),
            TaurineError::IndexOutOfBounds { index, length, .. } => {
                format!("Index {} out of bounds (length: {})", index, length)
            }
            TaurineError::FieldNotFound { field, .. } => {
                format!("Field '{}' not found", field)
            }
            TaurineError::Call { message, .. } => message.clone(),
            TaurineError::Import { path, message, .. } => {
                format!("Import error for '{}': {}", path, message)
            }
        }
    }
}

impl fmt::Display for TaurineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}", self.message(), self.line())
    }
}

impl std::error::Error for TaurineError {}

// Convert from String to TaurineError for backward compatibility
impl From<String> for TaurineError {
    fn from(s: String) -> Self {
        TaurineError::Runtime {
            message: s,
            line: 0,
        }
    }
}
