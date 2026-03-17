//! Environment management for the Taurine programming language.
//!
//! This module provides the `Environment` struct, which manages variable scopes
//! with support for parent environments (lexical scoping).

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;

/// Represents a variable environment/scope.
///
/// Environments can have parent environments, forming a chain for variable lookup.
#[derive(Clone, Debug)]
pub struct Environment {
    /// The variables stored in this environment.
    pub values: Rc<RefCell<HashMap<String, Value>>>,
    /// The parent environment (if any).
    parent: Option<Rc<RefCell<Environment>>>,
    /// Set of constant variable names.
    constants: Rc<RefCell<HashMap<String, bool>>>,
}

impl Environment {
    /// Creates a new root environment with no parent.
    pub fn new() -> Self {
        Environment {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
            constants: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Creates a new environment with the given parent.
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(parent),
            constants: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Defines a new variable in this environment.
    pub fn define(&self, name: String, value: Value) {
        self.values.borrow_mut().insert(name.clone(), value);
    }

    /// Defines a new constant in this environment.
    pub fn define_const(&self, name: String, value: Value) {
        self.values.borrow_mut().insert(name.clone(), value);
        self.constants.borrow_mut().insert(name, true);
    }

    /// Checks if a variable is a constant.
    pub fn is_const(&self, name: &str) -> bool {
        if self.constants.borrow().contains_key(name) {
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().is_const(name);
        }
        false
    }

    /// Gets a variable's value, searching in this environment and parent environments.
    ///
    /// Returns an error if the variable is not found.
    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(val) = self.values.borrow().get(name) {
            return Ok(val.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        Err(format!("Undefined variable: {name}"))
    }

    /// Assigns a new value to an existing variable.
    ///
    /// Searches in this environment and parent environments.
    /// Returns an error if the variable is not found or is a constant.
    pub fn assign(&self, name: &str, value: Value) -> Result<(), String> {
        if self.constants.borrow().contains_key(name) {
            return Err(format!("Cannot assign to const '{name}'"));
        }
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().assign(name, value);
        }
        Err(format!("Undefined variable: {name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_environment() {
        let env = Environment::new();
        assert!(env.values.borrow().is_empty());
        assert!(env.parent.is_none());
    }

    #[test]
    fn test_define_and_get() {
        let env = Environment::new();
        env.define("x".to_string(), Value::Number(42.0));

        let result = env.get("x").unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_get_undefined() {
        let env = Environment::new();
        let result = env.get("undefined");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_assign_existing() {
        let env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0));
        env.assign("x", Value::Number(2.0)).unwrap();

        let result = env.get("x").unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_assign_undefined() {
        let env = Environment::new();
        let result = env.assign("x", Value::Number(1.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_parent_environment() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent.borrow_mut().define("x".to_string(), Value::Number(100.0));

        let child = Environment::with_parent(parent.clone());
        child.define("y".to_string(), Value::Number(50.0));

        // Child can access parent's variables
        let result = child.get("x").unwrap();
        assert_eq!(result, Value::Number(100.0));

        // Child can access own variables
        let result = child.get("y").unwrap();
        assert_eq!(result, Value::Number(50.0));
    }

    #[test]
    fn test_assign_in_parent() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent.borrow_mut().define("x".to_string(), Value::Number(1.0));

        let child = Environment::with_parent(parent.clone());
        child.assign("x", Value::Number(999.0)).unwrap();

        let result = parent.borrow().get("x").unwrap();
        assert_eq!(result, Value::Number(999.0));
    }

    #[test]
    fn test_shadowing() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent.borrow_mut().define("x".to_string(), Value::Number(1.0));

        let child = Environment::with_parent(parent.clone());
        child.define("x".to_string(), Value::Number(2.0));

        // Child sees its own x
        let result = child.get("x").unwrap();
        assert_eq!(result, Value::Number(2.0));

        // Parent still sees its own x
        let result = parent.borrow().get("x").unwrap();
        assert_eq!(result, Value::Number(1.0));
    }
}