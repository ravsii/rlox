use std::collections::HashMap;

use crate::ast::Literal;

#[derive(Default)]
struct Enviroment {
    values: HashMap<String, Literal>,
}

impl Enviroment {
    pub fn new() -> Self {
        Enviroment::default()
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.values.get(name)
    }
}
