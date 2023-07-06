use super::{math, user_prefs};
use models::runner::{AstResult, IdPath, VariableMap};

use crate::{find_variable::find_variable, variables_expansion::expand_variables};
use models::{
    ast::*,
    error::{ErrorMessage, VariableExpansionError},
    return_value::ReturnValue,
    *,
};
use std::{collections::HashMap, f32::consts::E, process::id, vec};

pub fn execute_node(
    ast: &ASTNode,
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
    // TODO error stack ? -> l'erreur ne force pas forcement l'arret ? (surtout pour le linter)
) -> AstResult {
    id_path.push(ast.id);
    let res = match &ast.data {
        ASTNodeData::Sequence(sequence) => handle_sequence(sequence, variables, id_path, stdout),
        ASTNodeData::While(while_node) => handle_while(while_node, variables, id_path, stdout),
        ASTNodeData::IfElse(ifelse) => handle_if_else(ifelse, variables, id_path, stdout),
        ASTNodeData::RawText(value) => handle_raw_text(value, variables, id_path),
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
    };
    if id_path.pop() != Some(ast.id) {
        panic!("Id path is not correct");
    }
    res
}

fn handle_while(
    while_node: &While,
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
) -> AstResult {
    if while_node.is_do {
        todo!("Implement at the end of the project");
    }
    let mut iteration = 0;
    while iteration != user_prefs::MAX_ITERATION
        && get_bool(execute_node(
            &while_node.condition,
            variables,
            id_path,
            stdout,
        )?)
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
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
) -> AstResult {
    if get_bool(execute_node(
        &ifelse.if_.condition,
        variables,
        id_path,
        stdout,
    )?) {
        return handle_sequence(&ifelse.if_.sequence, variables, id_path, stdout);
    }
    if let Some(elifs) = &ifelse.elif {
        for elif in elifs {
            if get_bool(execute_node(&elif.condition, variables, id_path, stdout)?) {
                return handle_sequence(&elif.sequence, variables, id_path, stdout);
            }
        }
    }
    if let Some(else_) = &ifelse.else_ {
        return handle_sequence(&else_, variables, id_path, stdout);
    }
    Ok(ReturnValue::None)
}

fn handle_raw_text(text: &str, variables: &mut VariableMap, id_path: &IdPath) -> AstResult {
    match expand_variables(text, variables, id_path) {
        Ok(string) => match math::get_math_parsibility(&string) {
            math::MathParsability::Unparsable => Ok(ReturnValue::String_(string)),
            _ => match math::math_expression(&string) {
                Ok(v) => Ok(v),
                Err(_) => todo!(),
            },
        },
        Err(err) => Err(vec![ErrorMessage::new(
            id_path.clone(),
            error::ErrorType::VariableExpansionError(err),
            None,
        )]),
    }
}

fn handle_variable_assignment(
    variable_assignment: &VariableAssignment,
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
) -> AstResult {
    let value = execute_node(&variable_assignment.value, variables, id_path, stdout)?;
    let id = match find_variable(variable_assignment.name.as_str(), variables, id_path) {
        Some((_, id)) => id,
        None => {
            match id_path.len() {
                0 => panic!("the id path is empty"),
                1 => panic!("there is only one element in the id path: '{}'", id_path[0]),
                _ => {}
            }
            // on est sur une nouvelle variable
            // on prend l'avant dernier id du path, car le dernier est celui de la variable et on veut celui du scope
            *(id_path.get(id_path.len() - 2).unwrap())
        }
    };
    let variable_key = (variable_assignment.name.to_string(), id);
    let _old_value = variables.insert(variable_key, value);
    Ok(ReturnValue::None)
}

fn handle_function_call(
    function_call: &FunctionCall,
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
) -> AstResult {
    if function_call.is_builtin {
        match function_call.name.as_str() {
            "print" => {
                for arg in &function_call.argv {
                    let arg = execute_node(arg, variables, id_path, stdout)?;
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
    variables: &mut VariableMap,
    id_path: &mut IdPath,
    stdout: &mut Vec<String>,
) -> AstResult {
    sequence
        .iter()
        .find_map(|node| {
            let return_value = execute_node(node, variables, id_path, stdout);
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
