use std::collections::HashMap;
use crate::error::RuntimeError;
use crate::runtime::Value;

#[derive(Debug, Clone)]
pub struct Binding {
    pub value: Value,
    pub is_constant: bool,
}

#[derive(Debug, Default)]
pub struct Environment {
    globals: HashMap<String, Binding>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_variable(
        &mut self,
        name: String,
        value: Value,
        is_constant: bool,
    ) -> Result<(), RuntimeError> {
        if self.globals.contains_key(&name) {
            return Err(RuntimeError::DuplicateVariable(name));
        }

        self.globals.insert(name, Binding { value, is_constant });

        Ok(())
    }

    pub fn get_variable(&self, name: &str) -> Result<Value, RuntimeError> {
        self.globals
            .get(name)
            .map(|binding| binding.value.clone())
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        let binding: &mut Binding = self
            .globals
            .get_mut(name)
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.to_string()))?;

        if binding.is_constant {
            return Err(RuntimeError::ConstantReassignment(name.to_string()));
        }

        binding.value = value;
        Ok(())
    }
}
