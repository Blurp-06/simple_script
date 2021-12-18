use core::panic;

use pest::iterators::Pairs;

use crate::control_flow::match_rule_if;
use crate::functions::{match_rule_func_call_decl, FunctionContainer};
use crate::variables::{
    match_rule_empty_var, match_rule_reassign_variable, match_rule_vardecl, VariableContainer,
};
use crate::Rule;

pub struct CodeExecutor {
    pub var_container: VariableContainer,
    pub function_container: FunctionContainer,
}

// Function to execute a pair of rules.
impl CodeExecutor {
    pub fn new() -> CodeExecutor {
        CodeExecutor {
            function_container: FunctionContainer::new(),
            var_container: VariableContainer::new(),
        }
    }

    pub fn execute_code(&mut self, lines: Pairs<Rule>) {
        // Loop through all the pairs.
        for line in lines {
            match line.as_rule() {
                // Passes the pair to a function to keep this code clean.
                Rule::var_decl_assign => {
                    match_rule_vardecl(line, &mut self.var_container, &self.function_container);
                }

                Rule::var_empty_decl => match_rule_empty_var(line, &mut self.var_container),
                Rule::var_reassign_decl => {
                    match_rule_reassign_variable(
                        line,
                        &mut self.var_container,
                        &self.function_container,
                    );
                }
                Rule::func_call_decl => {
                    match_rule_func_call_decl(
                        line,
                        &self.function_container,
                        &mut self.var_container,
                    );
                }
                Rule::control_if => match_rule_if(line, self),
                Rule::debug => {
                    // Debug rules.
                    let debug_what = line.into_inner().into_iter().next().unwrap().as_rule();
                    match debug_what {
                        Rule::debug_var => {
                            self.var_container.debug_print_vars();
                        }
                        _ => {
                            panic!("Unsupported debug rule '{:?}'.", debug_what);
                        }
                    }
                }
                _ => {
                    panic!("Rule '{:?}' not implemented.", line.as_rule());
                }
            }
        }
    }
}
