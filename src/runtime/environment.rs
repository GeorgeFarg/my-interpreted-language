use std::collections::HashMap;

use super::RuntimeValue;

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn declare_var(&mut self, var_name: &str, value: RuntimeValue) -> RuntimeValue {
        if self.variables.contains_key(var_name) {
            panic!(
                "Cannot declare variable {}. As it already is defined",
                var_name
            );
        }
        self.variables.insert(var_name.to_string(), value.clone());
        value
    }

    #[allow(unused)]
    pub fn assign_variable(&mut self, var_name: &str, value: RuntimeValue) -> RuntimeValue {
        let env = self.resolve(var_name);
        if let Some(inserted_value) = env.variables.insert(var_name.to_string(), value) {
            inserted_value
        } else {
            panic!("Couldn't insert the value")
        }
    }

    // used to search the variable in the entire program and that includes the parents
    pub fn resolve(&mut self, var_name: &str) -> &mut Self {
        if self.variables.contains_key(var_name) {
            return self;
        }

        if let Some(ref mut parent) = self.parent {
            return parent.resolve(var_name);
        }

        panic!("Cannot resolve '{}' as it does not exist.", var_name)
    }

    pub fn lookup_var(&mut self, var_name: &str) -> &RuntimeValue {
        let env = self.resolve(var_name);

        if let Some(value) = env.variables.get(var_name) {
            value
        } else {
            panic!("Couldn't find the variable")
        }
    }

    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
        }
    }
}
