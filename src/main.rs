extern crate pest;
#[macro_use]
extern crate pest_derive;

use execute_code::CodeExecutor;
use pest::Parser;
use std::fs;

mod control_flow;
mod execute_code;
mod functions;
mod variables;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Relative to src.
struct SimpleParser;

fn main() {
    // Getting the source code of the user.
    let mut file_content = fs::read_to_string("main.smpl").expect("Couldn't read the file.");

    // Preventing nasty error where parser can't reach EOI because there is no newline; now there is a newline.
    file_content.push('\n');

    // Parse the file.
    let parsed = SimpleParser::parse(Rule::ast, &file_content).expect("Unable to parse file.");

    // Remove comments.
    let mut to_remove_slices: Vec<String> = Vec::new();
    for pair in parsed {
        match pair.as_rule() {
            Rule::line => {
                let inners = pair.into_inner();
                for inner in inners {
                    match inner.as_rule() {
                        Rule::comment_decl => {
                            to_remove_slices.push(inner.as_str().to_string());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // Actually removing them.
    for i in to_remove_slices {
        file_content = file_content.replace(i.as_str(), "");
    }

    // Parse the file again.
    let parsed = SimpleParser::parse(Rule::ast, &file_content).expect("Unable to parse file.");

    // Init containers.
    // let mut variable_container = VariableContainer::new();
    // let mut function_container = FunctionContainer::new();
    let mut code_executor = CodeExecutor::new();

    // Loop through the declerations.
    for pair in parsed {
        match pair.as_rule() {
            Rule::EOI => {}
            Rule::line => code_executor.execute_code(pair.into_inner()),
            _ => panic!("Unimplemented rule '{:?}'.", pair.as_rule()),
        }
    }

    // Prints out the variables at the end of a program.
    // code_executor.var_container.debug_print_vars();
}
