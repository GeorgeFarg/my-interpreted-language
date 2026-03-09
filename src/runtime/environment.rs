use std::collections::{HashMap, HashSet};

use super::RuntimeValue;

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn declare_var(
        &mut self,
        var_name: &str,
        value: RuntimeValue,
        is_constant: bool,
    ) -> RuntimeValue {
        if self.variables.contains_key(var_name) {
            panic!(
                "Cannot declare variable {}. As it already is defined",
                var_name
            );
        }
        self.variables.insert(var_name.to_string(), value.clone());

        if is_constant {
            self.constants.insert(var_name.to_string());
        }

        value
    }

    #[allow(unused)]
    pub fn assign_variable(&mut self, var_name: &str, value: RuntimeValue) -> RuntimeValue {
        let env = self.resolve(var_name);

        // cannot assign to constant
        if env.constants.contains(var_name) {
            panic!(
                "Cannot reassign to a variable {} as it was declared constant",
                var_name
            )
        }

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
            constants: HashSet::new(),
        }
    }
}
