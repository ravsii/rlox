use std::collections::HashMap;

use crate::ast::Literal;

pub enum EnvironmentError {
    UndefinedVariable(String),
}

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<Literal, EnvironmentError> {
        match self.values.get_mut(name) {
            Some(v) => {
                *v = value.clone();
                Ok(value)
            }
            None => Err(EnvironmentError::UndefinedVariable(format!(
                "Undefined variable '{}'",
                name,
            ))),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.values.get(name)
    }
}
