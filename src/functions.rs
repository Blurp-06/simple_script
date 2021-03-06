use core::panic;
use std::collections::HashMap;
use std::io::{self, Write};

use pest::iterators::Pair;

use crate::buildin_functions::math_functions::{simple_add, simple_sub};
use crate::type_string::make_string;
use crate::variables::{VariableContainer, VariableContent, VariableTypes};
use crate::Rule;

type SimpleFunction = fn(Vec<VariableContent>) -> VariableContent;

pub struct FunctionContainer {
    functions: HashMap<String, SimpleFunction>,
}

impl FunctionContainer {
    pub fn new() -> FunctionContainer {
        let mut loaded_func: HashMap<String, SimpleFunction> = HashMap::new();
        loaded_func.insert("print".to_string(), simple_print);
        loaded_func.insert("eq".to_string(), simple_eq);
        loaded_func.insert("add".to_string(), simple_add);
        loaded_func.insert("sub".to_string(), simple_sub);
        FunctionContainer {
            functions: loaded_func,
        }
    }

    pub fn call_function(&self, func_name: &str, args: Vec<VariableContent>) -> VariableContent {
        let function = *self
            .functions
            .get(func_name)
            .expect(format!("Unknown function '{}'.", func_name).as_str());
        function(args)
    }
}

// Matches the functions.
pub fn match_rule_func_call_decl(
    pair: Pair<Rule>,
    function_container: &FunctionContainer,
    variable_container: &VariableContainer,
) -> VariableContent {
    let mut args: Vec<VariableContent> = Vec::new();
    let mut func = "".to_string();
    let pair_inner = pair.into_inner();

    // Gets function and arguments.
    for p in pair_inner {
        match p.as_rule() {
            Rule::func_call_name => func = p.as_str().to_string(),
            Rule::func_call_args => {
                for px in p.into_inner() {
                    match px.as_rule() {
                        Rule::type_int => {
                            args.push(VariableContent {
                                value: px.as_str().to_string(),
                                data_type: VariableTypes::INT,
                            });
                        }
                        Rule::type_bool => {
                            args.push(VariableContent {
                                value: px.as_str().to_string(),
                                data_type: VariableTypes::BOOL,
                            });
                        }
                        Rule::type_string => args.push(VariableContent {
                            value: make_string(px.as_str()),
                            data_type: VariableTypes::STRING,
                        }),
                        Rule::type_float => args.push(VariableContent {
                            value: px.as_str().to_string(),
                            data_type: VariableTypes::FLOAT,
                        }),
                        Rule::var_name => {
                            // Gets the variable then copies it because it is a shared reference.
                            let var = variable_container.get_variable(px.as_str());
                            let var_to_push = VariableContent {
                                data_type: var.data_type,
                                value: var.value.clone(),
                            };
                            args.push(var_to_push);
                        }
                        Rule::func_call_decl => {
                            let r = match_rule_func_call_decl(
                                px,
                                function_container,
                                variable_container,
                            );
                            let to_push = VariableContent {
                                data_type: r.data_type,
                                value: r.value.clone(),
                            };
                            args.push(to_push);
                        }
                        _ => panic!("Rule {:?} not implemented for functions.", px.as_rule()),
                    }
                }
            }
            _ => {}
        }
    }

    // Calls the function.
    function_container.call_function(func.as_str(), args)
}

// Functions for in the simple script source code.
fn simple_print(args: Vec<VariableContent>) -> VariableContent {
    let mut final_message = String::new();
    for arg in args {
        final_message.push_str(arg.value.as_str());
    }
    println!("{}", final_message);
    io::stdout().flush().unwrap();
    VariableContent {
        data_type: VariableTypes::NULL,
        value: "".to_string(),
    }
}

// Compares all variables to check if type and value are the same.
fn simple_eq(args: Vec<VariableContent>) -> VariableContent {
    if args.len() < 2 {
        panic!(
            "Length of args was {}; there should atleast be 2 arguments.",
            args.len()
        );
    }

    // Gets value and type of the first argument.
    let mut arg_iter = args.into_iter();
    let eq_first = arg_iter.next().unwrap();
    let eq_type = eq_first.data_type;
    let eq_value = eq_first.value.clone();

    // Loops through all arguments to check if the value and type are equal to the first.
    for arg in arg_iter {
        if (arg.data_type != eq_type) || (arg.value != eq_value) {
            return VariableContent {
                value: "false".to_string(),
                data_type: VariableTypes::BOOL,
            };
        }
    }

    VariableContent {
        value: "true".to_string(),
        data_type: VariableTypes::BOOL,
    }
}
