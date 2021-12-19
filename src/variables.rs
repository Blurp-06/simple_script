use std::collections::HashMap;

use crate::{
    functions::{match_rule_func_call_decl, FunctionContainer},
    type_string::make_string,
    Rule,
};
use pest::iterators::Pair;

// All the different types a variable could be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariableTypes {
    INT,
    STRING,
    FLOAT,
    BOOL,
    NULL,
}

// Contains the actual variables and has some methods.
pub struct VariableContainer {
    variables: Vec<HashMap<String, VariableContent>>,
}

// Stores information about the variable that is stored, like the value and type.
#[derive(Debug, Clone)]
pub struct VariableContent {
    pub value: String,
    pub data_type: VariableTypes,
}

impl VariableContainer {
    // Creates a new and empty hashmap for the variables to live in.
    pub fn new() -> VariableContainer {
        let mut ret = VariableContainer {
            variables: Vec::new(),
        };
        ret.variables.push(HashMap::new());
        ret
    }

    // Moves into the next scope.
    pub fn scope_in(&mut self) {
        self.variables.push(HashMap::new());
    }

    // Moves out of the scope.
    pub fn scope_out(&mut self) {
        self.variables.pop();
    }

    // Adds a variable to the hash.
    pub fn add_variable(&mut self, name: &str, value: VariableContent) {
        self.variables
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value);
    }

    // Sets a variable in the first scope it exists in.
    pub fn set_variable(&mut self, name: &str, var_value: VariableContent) {
        let var_iter = self.variables.iter_mut().rev();

        for scope in var_iter {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), var_value.clone());
                return;
            }
        }

        panic!("Couldn't set variable {} as it doesn't exist.", name);
    }

    // Gets the VariableContent of a variable.
    pub fn get_variable(&self, name: &str) -> &VariableContent {
        let var_iter = self.variables.iter().rev();

        for scope in var_iter {
            if scope.contains_key(name) {
                return scope.get(name).unwrap();
            }
        }

        panic!("Couldn't get variable {}.", name);
    }

    // Prints out the content of the hashmap for debug purposes.
    #[allow(dead_code)]
    pub fn debug_print_vars(&self) {
        for (k, v) in self.variables.last().unwrap().iter() {
            println!("{}: {:?}", k, v);
        }
    }
}

pub fn match_rule_vardecl(
    pair: Pair<Rule>,
    var_container: &mut VariableContainer,
    function_container: &FunctionContainer,
) {
    // Moves into the useful info.
    let inner = pair.into_inner();

    // Pre-allocates memory for the add_variable call later.
    let mut var_name = "";
    let mut var_content: VariableContent = VariableContent {
        value: "".to_string(),
        data_type: VariableTypes::NULL,
    };

    // Gets the name and content info.
    for info in inner {
        match info.as_rule() {
            Rule::var_name => {
                var_name = info.as_str();
            }

            Rule::var_types => {
                // Don't know why, but it works.
                let type_info = info
                    .into_inner()
                    .into_iter()
                    .next()
                    .expect("Error parsing type.");

                // Assigns the types and value for the variables.
                match type_info.as_rule() {
                    Rule::type_int => {
                        var_content.data_type = VariableTypes::INT;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_bool => {
                        var_content.data_type = VariableTypes::BOOL;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_float => {
                        var_content.data_type = VariableTypes::FLOAT;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_string => {
                        var_content.data_type = VariableTypes::STRING;
                        var_content.value = make_string(type_info.as_str());
                    }
                    Rule::var_name => {
                        let result = var_container.get_variable(type_info.as_str());
                        var_content.data_type = result.data_type;
                        var_content.value = result.value.clone();
                    }
                    Rule::func_call_decl => {
                        let result =
                            match_rule_func_call_decl(type_info, function_container, var_container);
                        var_content.data_type = result.data_type;
                        var_content.value = result.value.clone();
                    }
                    _ => panic!("Type not implemented: {:?}", type_info.as_rule()),
                }
            }
            _ => {
                panic!("Not implemented: {}", info);
            }
        }
    }

    // Finally adds the variable.
    var_container.add_variable(var_name, var_content);
}

// Only sets a variable but doesn't init one.
pub fn match_rule_reassign_variable(
    pair: Pair<Rule>,
    var_container: &mut VariableContainer,
    function_container: &FunctionContainer,
) {
    // Moves into the useful info.
    let inner = pair.into_inner();

    // Pre-allocates memory for the add_variable call later.
    let mut var_name = "";
    let mut var_content: VariableContent = VariableContent {
        value: "".to_string(),
        data_type: VariableTypes::NULL,
    };

    // Gets the name and content info.
    for info in inner {
        match info.as_rule() {
            Rule::var_name => {
                var_name = info.as_str();
            }

            Rule::var_types => {
                // Don't know why, but it works.
                let type_info = info
                    .into_inner()
                    .into_iter()
                    .next()
                    .expect("Error parsing type.");

                // Assigns the types and value for the variables.
                match type_info.as_rule() {
                    Rule::type_int => {
                        var_content.data_type = VariableTypes::INT;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_float => {
                        var_content.data_type = VariableTypes::FLOAT;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_bool => {
                        var_content.data_type = VariableTypes::BOOL;
                        var_content.value = type_info.as_str().to_string();
                    }
                    Rule::type_string => {
                        var_content.data_type = VariableTypes::STRING;
                        var_content.value = make_string(type_info.as_str());
                    }
                    Rule::var_name => {
                        let result = var_container.get_variable(type_info.as_str());
                        var_content.data_type = result.data_type;
                        var_content.value = result.value.clone();
                    }
                    Rule::func_call_decl => {
                        let result =
                            match_rule_func_call_decl(type_info, function_container, var_container);
                        var_content.data_type = result.data_type;
                        var_content.value = result.value.clone();
                    }
                    _ => panic!("Type not implemented: {:?}", type_info.as_rule()),
                }
            }
            _ => {
                panic!("Not implemented: {}", info);
            }
        }
    }

    // Finally adds the variable.
    var_container.set_variable(var_name, var_content);
}

// Match case for an empty init.
pub fn match_rule_empty_var(pair: Pair<Rule>, var_container: &mut VariableContainer) {
    let var_name = pair.into_inner().into_iter().next().unwrap().as_str();
    var_container.add_variable(
        var_name,
        VariableContent {
            data_type: VariableTypes::NULL,
            value: "".to_string(),
        },
    );
}
