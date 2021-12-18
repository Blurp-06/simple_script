extern crate pest;
#[macro_use]
extern crate pest_derive;
pub mod buildin_functions;
pub mod control_flow;
pub mod execute_code;
pub mod functions;
pub mod variables;

use std::fs;

use pest::Parser;

use crate::execute_code::CodeExecutor;

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

    // Init container.
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
