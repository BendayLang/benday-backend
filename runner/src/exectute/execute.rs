use super::{math, user_prefs, AstResult};
use crate::variables_expansion::expand_variables;
use models::{ast::*, return_value::ReturnValue, *};
use std::collections::HashMap;

pub fn exec_ast(
    ast: &ASTNode,
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    // The if statement is to prevent adding the same id twice when looping (e.g. while loops)
    if id_path.last() != Some(&ast.id) {
        id_path.push(ast.id);
    }
    match &ast.data {
        ASTNodeData::Sequence(sequence) => handle_sequence(sequence, variables, id_path, stdout),
        ASTNodeData::While(while_node) => handle_while(while_node, variables, id_path, stdout),
        ASTNodeData::IfElse(ifelse) => handle_if_else(ifelse, variables, id_path, stdout),
        ASTNodeData::Input(value) => handle_input(value, variables),
        ASTNodeData::VariableAssignment(variable_assignment) => {
            handle_variable_assignment(variable_assignment, variables, id_path, stdout)
        }
        ASTNodeData::FunctionCall(function_call) => {
            handle_function_call(function_call, variables, id_path, stdout)
        }
        ASTNodeData::FunctionDeclaration(_function_declaration) => {
            // variables.insert(
            //     function_declaration.name.to_string(),
            //     ReturnValue::Function(function_declaration.clone()),
            // );
            Ok(ReturnValue::None)
        }
    }
}

fn handle_while(
    while_node: &While,
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    if while_node.is_do {
        todo!("Implement at the end of the project");
    }
    let mut iteration = 0;
    while iteration != user_prefs::MAX_ITERATION
        && get_bool(exec_ast(&while_node.condition, variables, id_path, stdout)?)
    {
        let return_value = handle_sequence(&while_node.sequence, variables, id_path, stdout)?;
        if return_value != ReturnValue::None {
            return Ok(return_value);
        }
        iteration += 1;
    }
    if iteration == user_prefs::MAX_ITERATION {
        todo!("break on max iteration ({})", user_prefs::MAX_ITERATION);
    }
    return Ok(ReturnValue::None);
}

fn handle_if_else(
    ifelse: &IfElse,
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    if get_bool(exec_ast(&ifelse.if_.condition, variables, id_path, stdout)?) {
        return handle_sequence(&ifelse.if_.sequence, variables, id_path, stdout);
    }
    if let Some(elifs) = &ifelse.elif {
        for elif in elifs {
            if get_bool(exec_ast(&elif.condition, variables, id_path, stdout)?) {
                return handle_sequence(&elif.sequence, variables, id_path, stdout);
            }
        }
    }
    if let Some(else_) = &ifelse.else_ {
        return handle_sequence(&else_, variables, id_path, stdout);
    }
    Ok(ReturnValue::None)
}

fn handle_input(value: &str, variables: &mut HashMap<String, ReturnValue>) -> AstResult {
    let value: String = if value.contains("{") {
        match expand_variables(value, variables) {
            Ok(v) => v,
            Err(()) => todo!("erreur expand_variables, comment reagir ?"),
        }
    } else {
        value.to_string()
    };
    match math::get_math_parsibility(&value) {
        math::MathParsability::Unparsable => Ok(ReturnValue::String_(value)),
        _ => math::math_expression(&value),
    }
}

fn handle_variable_assignment(
    variable_assignment: &VariableAssignment,
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    let value = exec_ast(&variable_assignment.value, variables, id_path, stdout)?;
    let _old_value = variables.insert(variable_assignment.name.to_string(), value);
    Ok(ReturnValue::None)
}

fn handle_function_call(
    function_call: &FunctionCall,
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    if function_call.is_builtin {
        match function_call.name.as_str() {
            "print" => {
                for arg in &function_call.argv {
                    let arg = exec_ast(arg, variables, id_path, stdout)?;
                    stdout.push(arg.to_string());
                }
            }
            _ => todo!("FunctionCall"),
        }
    }
    Ok(ReturnValue::None)
}

fn handle_sequence(
    sequence: &[ASTNode],
    variables: &mut HashMap<String, ReturnValue>,
    id_path: &mut Vec<u32>,
    stdout: &mut Vec<String>,
) -> AstResult {
    sequence
        .iter()
        .find_map(|node| {
            let return_value = exec_ast(node, variables, id_path, stdout);
            if return_value != Ok(ReturnValue::None) {
                Some(return_value)
            } else {
                None
            }
        })
        .unwrap_or(Ok(ReturnValue::None))
}

fn get_bool(return_value: ReturnValue) -> bool {
    match return_value {
        ReturnValue::Bool(val) => val,
        ReturnValue::None => false,
        ReturnValue::String_(val) => todo!("error should return a bool, not a string ({val})"),
        ReturnValue::Int(val) => val != 0,
        ReturnValue::Float(val) => val != 0.0,
    }
}
