use crate::error::RuntimeError;
use crate::runtime::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Binding {
    pub value: Value,
    pub is_constant: bool,
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<HashMap<String, Binding>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    pub fn define_variable(
        &mut self,
        name: String,
        value: Value,
        is_constant: bool,
    ) -> Result<(), RuntimeError> {
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name) {
            return Err(RuntimeError::DuplicateVariable(name));
        }
        scope.insert(name, Binding { value, is_constant });
        Ok(())
    }
    pub fn get_variable(&self, name: &str) -> Result<Value, RuntimeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(binding) = scope.get(name) {
                return Ok(binding.value.clone());
            }
        }
        Err(RuntimeError::UndefinedVariable(name.to_string()))
    }
    pub fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(binding) = scope.get_mut(name) {
                if binding.is_constant {
                    return Err(RuntimeError::ConstantReassignment(name.to_string()));
                }
                binding.value = value;
                return Ok(());
            }
        }
        Err(RuntimeError::UndefinedVariable(name.to_string()))
    }
}
