use std::collections::HashMap;

use crate::{
    functions::{match_rule_func_call_decl, FunctionContainer},
    Rule,
};
use pest::iterators::Pair;

// All the different types a variable could be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariableTypes {
    INT,
    BOOL,
    NULL,
}

// Contains the actual variables and has some methods.
pub struct VariableContainer {
    variables: HashMap<String, VariableContent>,
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
        VariableContainer {
            variables: HashMap::new(),
        }
    }

    // Adds a variable to the hash.
    pub fn add_variable(&mut self, name: &str, value: VariableContent) {
        self.variables.insert(name.to_string(), value);
    }

    // Gets the VariableContent of a variable.
    pub fn get_variable(&self, name: &str) -> &VariableContent {
        let var = self
            .variables
            .get(name)
            .expect(format!("Couldn't get variable '{}'.", name).as_str());
        var
    }

    // Prints out the content of the hashmap for debug purposes.
    #[allow(dead_code)]
    pub fn debug_print_vars(&self) {
        for (k, v) in self.variables.iter() {
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
    let mut var_name = " ";
    let mut var_content: VariableContent = VariableContent {
        value: " ".to_string(),
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
