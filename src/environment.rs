use std::collections::HashMap;

use crate::ast::Literal;

pub enum EnvironmentError {
    UndefinedVariable(String),
}

#[derive(Default)]
pub struct Environment {
    /// parent
    parent: Option<Box<Environment>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn new_child(parent: Environment) -> Self {
        Environment {
            parent: Some(Box::new(parent)),
            ..Environment::default()
        }
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<Literal, EnvironmentError> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value.clone();
                Ok(value)
            }
            None => {
                if let Some(parent) = &mut self.parent {
                    parent.assign(name, value)
                } else {
                    Err(EnvironmentError::UndefinedVariable(format!(
                        "Undefined variable '{}'",
                        name,
                    )))
                }
            }
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<Literal, EnvironmentError> {
        match self.values.get(name) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    parent.get(name)
                } else {
                    Err(EnvironmentError::UndefinedVariable(format!(
                        "Undefined variable '{}'",
                        name,
                    )))
                }
            }
        }
    }
}
