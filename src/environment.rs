use crate::ast::Literal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum EnvironmentError {
    UndefinedVariable(String),
}

#[derive(Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    /// Create a new root environment
    pub fn new() -> Self {
        Environment {
            parent: None,
            values: HashMap::new(),
        }
    }

    /// Create a new child environment pointing to `parent`
    pub fn new_child(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<Literal, EnvironmentError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            Ok(value)
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(EnvironmentError::UndefinedVariable(name.to_string()))
        }
    }

    pub fn get(&self, name: &str) -> Result<Literal, EnvironmentError> {
        if let Some(val) = self.values.get(name) {
            Ok(val.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            Err(EnvironmentError::UndefinedVariable(name.to_string()))
        }
    }
}
