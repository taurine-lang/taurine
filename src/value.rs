//! Value types

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use smallvec::SmallVec;

use crate::environment::Environment;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Table(Rc<RefCell<HashMap<usize, Value>>>),
    Array(Rc<RefCell<SmallVec<[Value; 4]>>>),
    Range {
        start: f64,
        end: f64,
    },
    Function {
        name: usize,
        params: Vec<usize>,
        default_params: Vec<crate::ast::Expr>,
        body: Vec<crate::ast::Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    NativeFunction(fn(&[Value]) -> Result<Value, String>),
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
            Value::Number(n) => write!(f, "{n}"),
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
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Bool(false) => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    pub fn new_table() -> Self {
        Value::Table(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn new_array() -> Self {
        Value::Array(Rc::new(RefCell::new(SmallVec::new())))
    }
}
