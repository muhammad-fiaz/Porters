use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    pub variables: HashMap<String, String>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
}