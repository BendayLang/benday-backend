use crate::math;
use models::{ASTNode, ReturnValue};
use std::collections::HashMap;

mod execute;
mod tests;
use execute::exec_ast;

mod user_prefs {
    pub const MAX_ITERATION: usize = 100;
}

pub type AstResult = Result<ReturnValue, ()>;

pub fn execute(ast: &ASTNode) -> (AstResult, Vec<String>, HashMap<String, ReturnValue>) {
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let return_value = exec_ast(ast, &mut variables, &mut Vec::new(), &mut stdout);
    return (return_value, stdout, variables);
}
