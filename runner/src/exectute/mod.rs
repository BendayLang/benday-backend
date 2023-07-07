mod execute;
#[cfg(test)]
mod tests;

use crate::math;
use execute::execute_node;
use models::{
    ast::Node,
    error::ErrorMessage,
    return_value::ReturnValue,
    runner::{RunnerResult, VariableMap},
};
use std::collections::HashMap;

mod user_prefs {
    pub const MAX_ITERATION: usize = 100;
}

pub fn runner(ast: &Node) -> (RunnerResult, Vec<String>, VariableMap) {
    match &ast.data {
        models::ast::NodeData::Sequence(_) => (),
        _ => {
            return (
                Err(vec![ErrorMessage::new(
                    vec![],
                    models::error::ErrorType::RootIsNotSequence,
                    None,
                )]),
                Vec::new(),
                HashMap::new(),
            )
        }
    }
    let mut variables: VariableMap = HashMap::new();
    let mut stdout = Vec::new();
    let return_value: RunnerResult =
        execute_node(ast, &mut variables, &mut Vec::new(), &mut stdout);
    return (return_value, stdout, variables);
}

pub fn linter(_ast: &Node) -> RunnerResult {
    todo!("Implement linter")
}
