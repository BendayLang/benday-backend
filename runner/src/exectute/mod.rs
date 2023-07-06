use crate::math;
use std::collections::HashMap;

mod execute;
mod tests;
use execute::execute_node;
use models::{ast::ASTNode, error::ErrorMessage, return_value::ReturnValue};

mod user_prefs {
    pub const MAX_ITERATION: usize = 100;
}

pub type AstResult = Result<ReturnValue, Vec<ErrorMessage>>;

// La cle contiens le nom de la variable et l'id de le sequence (scope) dans laquelle elle a ete declaree
// Ca va permettre de gerer les variables locales et globales (recursivement)
pub type VariableMap = HashMap<(String, models::ast::Id), ReturnValue>;

pub fn runner(ast: &ASTNode) -> (AstResult, Vec<String>, VariableMap) {
    let mut variables: VariableMap = HashMap::new();
    let mut stdout = Vec::new();
    let return_value = execute_node(ast, &mut variables, &mut Vec::new(), &mut stdout);
    return (return_value, stdout, variables);
}

pub fn linter(ast: &ASTNode) -> AstResult {
    todo!("Implement linter")
}
