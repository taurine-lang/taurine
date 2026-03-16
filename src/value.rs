//! Value types for the Taurine programming language.
//!
//! This module defines the `Value` enum, which represents all possible values
//! that can be used in Taurine, including numbers, strings, tables, arrays, etc.

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::environment::Environment;

/// Represents any value in the Taurine language.
#[derive(Clone, Debug)]
pub enum Value {
    /// A 64-bit floating point number.
    Number(f64),
    /// A UTF-8 string.
    String(String),
    /// A boolean value.
    Bool(bool),
    /// The nil value (represents absence of a value).
    Nil,
    /// A table (hash map) of string keys to values.
    Table(Rc<RefCell<HashMap<String, Value>>>),
    /// An array (vector) of values. Uses SmallVec for small arrays (up to 4 elements inline).
    Array(Rc<RefCell<Vec<Value>>>),
    /// A range of numbers (used in for-in loops).
    Range {
        /// Start of the range.
        start: f64,
        /// End of the range.
        end: f64,
    },
    /// A user-defined function.
    Function {
        /// Function name.
        name: String,
        /// Parameter names.
        params: Vec<String>,
        /// Default parameter values.
        default_params: Vec<crate::ast::Expr>,
        /// Function body statements.
        body: Vec<crate::ast::Stmt>,
        /// Closure environment.
        closure: Rc<RefCell<Environment>>,
    },
    /// A built-in native function.
    NativeFunction(fn(&[Value]) -> Result<Value, String>),
    /// An error value.
    Error(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Error(a), Value::Error(b)) => a == b,
            (Value::Range { start: s1, end: e1 }, Value::Range { start: s2, end: e2 }) => {
                s1 == s2 && e1 == e2
            }
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{n}")
                } else {
                    write!(f, "{n}")
                }
            }
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
            Value::Table(t) => {
                let table = t.borrow();
                if table.is_empty() {
                    write!(f, "{{}}")
                } else {
                    let items: Vec<String> = table.iter().map(|(k, v)| format!("{k}: {v}")).collect();
                    write!(f, "{{{}}}", items.join(", "))
                }
            }
            Value::Array(arr) => {
                let array = arr.borrow();
                let items: Vec<String> = array.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Range { start, end } => write!(f, "{}..{}", *start as i64, *end as i64),
            Value::Function { name, .. } => write!(f, "<function {name}>"),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
            Value::Error(msg) => write!(f, "error: {msg}"),
        }
    }
}

impl Value {
    /// Returns `true` if the value is truthy, `false` otherwise.
    ///
    /// The following values are falsy:
    /// - `Value::Nil`
    /// - `Value::Bool(false)`
    /// - `Value::Error(_)`
    ///
    /// All other values are truthy.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false) | Value::Error(_))
    }

    /// Creates a new empty table.
    pub fn new_table() -> Value {
        Value::Table(Rc::new(RefCell::new(HashMap::new())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_display_integer() {
        let v = Value::Number(42.0);
        assert_eq!(v.to_string(), "42");
    }

    #[test]
    fn test_number_display_float() {
        let v = Value::Number(3.14);
        assert_eq!(v.to_string(), "3.14");
    }

    #[test]
    fn test_string_display() {
        let v = Value::String("hello".to_string());
        assert_eq!(v.to_string(), "hello");
    }

    #[test]
    fn test_bool_display() {
        let v_true = Value::Bool(true);
        let v_false = Value::Bool(false);
        assert_eq!(v_true.to_string(), "true");
        assert_eq!(v_false.to_string(), "false");
    }

    #[test]
    fn test_nil_display() {
        let v = Value::Nil;
        assert_eq!(v.to_string(), "nil");
    }

    #[test]
    fn test_empty_table_display() {
        let v = Value::new_table();
        assert_eq!(v.to_string(), "{}");
    }

    #[test]
    fn test_table_display() {
        let v = Value::new_table();
        if let Value::Table(t) = &v {
            t.borrow_mut().insert("name".to_string(), Value::String("test".to_string()));
            t.borrow_mut().insert("value".to_string(), Value::Number(42.0));
        }
        let s = v.to_string();
        assert!(s.contains("name: test"));
        assert!(s.contains("value: 42"));
    }

    #[test]
    fn test_empty_array_display() {
        let v = Value::Array(Rc::new(RefCell::new(Vec::new())));
        assert_eq!(v.to_string(), "[]");
    }

    #[test]
    fn test_array_display() {
        let arr = Rc::new(RefCell::new(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]));
        let v = Value::Array(arr);
        assert_eq!(v.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_range_display() {
        let v = Value::Range { start: 1.0, end: 10.0 };
        assert_eq!(v.to_string(), "1..10");
    }

    #[test]
    fn test_function_display() {
        let v = Value::Function {
            name: "test".to_string(),
            params: vec![],
            default_params: vec![],
            body: vec![],
            closure: Rc::new(RefCell::new(Environment::new())),
        };
        assert_eq!(v.to_string(), "<function test>");
    }

    #[test]
    fn test_native_function_display() {
        let v = Value::NativeFunction(|_| Ok(Value::Nil));
        assert_eq!(v.to_string(), "<native fn>");
    }

    #[test]
    fn test_error_display() {
        let v = Value::Error("something went wrong".to_string());
        assert_eq!(v.to_string(), "error: something went wrong");
    }

    #[test]
    fn test_is_truthy() {
        assert!(!Value::Nil.is_truthy());
        assert!(!Value::Bool(false).is_truthy());
        assert!(!Value::Error("err".to_string()).is_truthy());
        
        assert!(Value::Bool(true).is_truthy());
        assert!(Value::Number(0.0).is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(Value::String("".to_string()).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(Value::new_table().is_truthy());
    }
}