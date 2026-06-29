//! Environment management

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::string_intern::InternedString;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Rc<RefCell<HashMap<usize, Value>>>,
    parent: Option<Rc<RefCell<Environment>>>,
    constants: Rc<RefCell<HashMap<usize, ()>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
            constants: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(parent),
            constants: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn define(&mut self, name: InternedString, value: Value) {
        self.values.borrow_mut().insert(name.id(), value);
    }

    pub fn define_with_name(&mut self, name: &str, value: Value) {
        // For FFI - store value with a hash-based ID
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish() as usize;
        
        self.values.borrow_mut().insert(id, value);
    }

    pub fn define_const(&mut self, name: InternedString, value: Value) {
        self.values.borrow_mut().insert(name.id(), value);
        self.constants.borrow_mut().insert(name.id(), ());
    }

    pub fn is_const(&self, name_id: usize) -> bool {
        if self.constants.borrow().contains_key(&name_id) {
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().is_const(name_id);
        }
        false
    }

    pub fn get(&self, name: &InternedString) -> Result<Value, String> {
        if let Some(value) = self.values.borrow().get(&name.id()) {
            return Ok(value.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        Err(format!("Undefined variable: {}", name.id()))
    }

    pub fn get_by_name(&self, name: &str) -> Result<Value, String> {
        // For FFI - lookup by string name using hash
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish() as usize;
        
        if let Some(value) = self.values.borrow().get(&id) {
            return Ok(value.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get_by_name(name);
        }
        Err(format!("Undefined variable: {name}"))
    }

    pub fn assign(&mut self, name: &InternedString, value: Value) -> Result<(), String> {
        if self.values.borrow_mut().insert(name.id(), value.clone()).is_some() {
            return Ok(());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign(name, value);
        }
        Err(format!("Undefined variable: {}", name.id()))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
