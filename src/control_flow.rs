use crate::{
    execute_code::CodeExecutor,
    functions::match_rule_func_call_decl,
    variables::{VariableContent, VariableTypes},
    Rule,
};
use pest::iterators::Pair;

// Matches if statements.
pub fn match_rule_if(if_statement: Pair<Rule>, executor: &mut CodeExecutor) {
    // Gets the condition and makes an iterable.
    let mut if_iter = if_statement.into_inner().into_iter();
    let condition = if_iter.next().unwrap();

    // Temp value.
    let mut condition_value: VariableContent = VariableContent {
        value: "".to_string(),
        data_type: VariableTypes::NULL,
    };

    // Check for types of conditions.
    match condition.as_rule() {
        Rule::type_bool => {
            condition_value.value = condition.as_str().clone().to_string();
            condition_value.data_type = VariableTypes::BOOL;
        }
        Rule::var_name => {
            let r = executor.var_container.get_variable(condition.as_str());
            condition_value.data_type = r.data_type;
            condition_value.value = r.value.clone();
        }
        Rule::func_call_decl => {
            let r = match_rule_func_call_decl(
                condition,
                &executor.function_container,
                &executor.var_container,
            );
            condition_value.data_type = r.data_type;
            condition_value.value = r.value.clone();
        }
        _ => {
            panic!(
                "Not implemented condition rule for if statement: '{:?}'.",
                condition.as_rule()
            );
        }
    }

    // Execute the code.
    match condition_value.data_type {
        VariableTypes::BOOL => {
            if condition_value.value == "true" {
                executor.var_container.scope_in();
                for pair in if_iter {
                    executor.execute_code(pair.into_inner());
                }
                executor.var_container.scope_out();
                return;
            }
        }
        _ => {
            panic!(
                "Data type '{:?}' not implemented for if condition.",
                condition_value.data_type
            )
        }
    }
}
