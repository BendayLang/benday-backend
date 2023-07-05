use crate::math;
use crate::variables_expansion::expand_variables;
use models::{
    ASTNode, ASTNodeData, FunctionCall, FunctionDeclaration, If, IfElse, ReturnValue,
    VariableAssignment, While,
};
use std::collections::HashMap;

mod user_prefs {
    pub const MAX_ITERATION: usize = 100;
}

// pub fn execute(ast: &ASTNode) -> (ReturnValue, Vec<String>, HashMap<String, ReturnValue>) {
//     let mut variables = HashMap::new();
//     exec_ast(ast, &mut variables);
// }

fn exec_sequence(
    sequence: &[ASTNode],
    variables: &mut HashMap<String, ReturnValue>,
) -> ReturnValue {
    sequence
        .iter()
        .find_map(|node| {
            let return_value = exec_ast(node, variables);
            if return_value != ReturnValue::None {
                Some(return_value)
            } else {
                None
            }
        })
        .unwrap_or(ReturnValue::None)
}

// TODO new signature : pub fn exec_ast(ast: &ASTNode, variables: &mut HashMap<String, ReturnValue>, id_path: &mut Vec<u32>, stdout: &mut Vec<String>) -> Success<ReturnValue> {
pub fn exec_ast(ast: &ASTNode, variables: &mut HashMap<String, ReturnValue>) -> ReturnValue {
    match &ast.data {
        ASTNodeData::Sequence(sequence) => exec_sequence(sequence, variables),
        ASTNodeData::While(While {
            is_do,
            condition,
            sequence,
        }) => {
            if *is_do {
                let return_value = exec_sequence(&sequence, variables);
                if return_value != ReturnValue::None {
                    return return_value;
                }
            }
            let mut iteration = 0;
            while iteration != user_prefs::MAX_ITERATION
                && get_bool(exec_ast(&condition, variables))
            {
                let return_value = exec_sequence(&sequence, variables);
                if return_value != ReturnValue::None {
                    return return_value;
                }
                iteration += 1;
            }
            if iteration == user_prefs::MAX_ITERATION {
                todo!("break on max iteration ({})", user_prefs::MAX_ITERATION);
            }
            return ReturnValue::None;
        }
        ASTNodeData::IfElse(ifelse) => {
            if get_bool(exec_ast(&ifelse.if_.condition, variables)) {
                return exec_sequence(&ifelse.if_.sequence, variables);
            }
            if let Some(elifs) = &ifelse.elif {
                for elif in elifs {
                    if get_bool(exec_ast(&elif.condition, variables)) {
                        return exec_sequence(&elif.sequence, variables);
                    }
                }
            }
            if let Some(else_) = &ifelse.else_ {
                return exec_sequence(&else_, variables);
            }
            ReturnValue::None
        }
        ASTNodeData::Input(value) => {
            let value: String = if value.contains("{") {
                match expand_variables(value, variables) {
                    Ok(v) => v,
                    Err(()) => todo!("erreur expand_variables, comment reagir ?"),
                }
            } else {
                value.to_string()
            };
            match math::get_math_parsibility(&value) {
                math::MathParsability::Unparsable => ReturnValue::String_(value),
                _ => math::math_expression(&value).unwrap(),
            }
        }
        ASTNodeData::VariableAssignment(VariableAssignment { name, value }) => {
            let value = exec_ast(value, variables);
            let _old_value = variables.insert(name.to_string(), value);
            ReturnValue::None
        }
        ASTNodeData::FunctionCall(FunctionCall {
            is_builtin,
            name,
            argv,
        }) => {
            if *is_builtin && name == "print" {
                for inst in argv {
                    println!("{:?}", exec_ast(inst, variables));
                }
            }
            ReturnValue::None
        }
        ASTNodeData::FunctionDeclaration(FunctionDeclaration {
            argv,
            name,
            sequence,
        }) => {
            todo!("FunctionDeclaration");
            ReturnValue::None
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, Write};

    #[test]
    fn basic_assignation_and_print() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Sequence(vec![
                ASTNode {
                    id: 1,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 2,
                            data: ASTNodeData::Input("42".to_string()),
                        }),
                    }),
                },
                ASTNode {
                    id: 3,
                    data: ASTNodeData::FunctionCall(FunctionCall {
                        is_builtin: false,
                        name: "print".to_string(),
                        argv: vec![ASTNode {
                            id: 4,
                            data: ASTNodeData::Input("x".to_string()),
                        }],
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(42));
    }

    #[test]
    fn test_exec_ast_input() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Input("42".to_string()),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::Int(42));
    }

    #[test]
    fn test_exec_ast_variable_assignment() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::VariableAssignment(VariableAssignment {
                name: "x".to_string(),
                value: Box::new(ASTNode {
                    id: 1,
                    data: ASTNodeData::Input("42".to_string()),
                }),
            }),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(42)));
    }

    // #[test]
    // fn test_exec_ast_builtin_function_call() { // TODO test the stdout
    //     let ast = ASTNode {
    //         id: 0,
    //         data: ASTNodeData::FunctionCall(FunctionCall {
    //             is_builtin: true,
    //             name: "print".to_string(),
    //             argv: vec![ASTNode {
    //                 id: 1,
    //                 data: ASTNodeData::Input("42".to_string()),
    //             }],
    //         }),
    //     };
    //     let mut variables = HashMap::new();
    //     let return_value = exec_ast(&ast, &mut variables);
    //     assert_eq!(return_value, ReturnValue::None);
    // }

    #[test]
    fn test_exec_ast_sequence() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Sequence(vec![
                ASTNode {
                    id: 1,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 2,
                            data: ASTNodeData::Input("42".to_string()),
                        }),
                    }),
                },
                ASTNode {
                    id: 3,
                    data: ASTNodeData::FunctionCall(FunctionCall {
                        is_builtin: true,
                        name: "print".to_string(),
                        argv: vec![ASTNode {
                            id: 4,
                            data: ASTNodeData::Input("x".to_string()),
                        }],
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(42)));
    }

    #[test]
    fn test_exec_ast_while() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::While(While {
                is_do: false,
                condition: Box::new(ASTNode {
                    id: 1,
                    data: ASTNodeData::Input("{x} < 10".to_string()),
                }),
                sequence: vec![ASTNode {
                    id: 2,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 3,
                            data: ASTNodeData::Input("{x} + 1".to_string()),
                        }),
                    }),
                }],
            }),
        };
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), ReturnValue::Int(0));
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(10)));
    }

    #[test]
    fn test_exec_ast_if_else() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Sequence(vec![
                ASTNode {
                    id: 1,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 2,
                            data: ASTNodeData::Input("10".to_string()),
                        }),
                    }),
                },
                ASTNode {
                    id: 3,
                    data: ASTNodeData::IfElse(IfElse {
                        if_: If {
                            condition: Box::new(ASTNode {
                                id: 4,
                                data: ASTNodeData::Input("{x} > 10".to_string()),
                            }),
                            sequence: vec![ASTNode {
                                id: 5,
                                data: ASTNodeData::VariableAssignment(VariableAssignment {
                                    name: "x".to_string(),
                                    value: Box::new(ASTNode {
                                        id: 6,
                                        data: ASTNodeData::Input("{x} + 1".to_string()),
                                    }),
                                }),
                            }],
                        },
                        elif: Some(vec![If {
                            condition: Box::new(ASTNode {
                                id: 7,
                                data: ASTNodeData::Input("{x} > 20".to_string()),
                            }),
                            sequence: vec![ASTNode {
                                id: 8,
                                data: ASTNodeData::VariableAssignment(VariableAssignment {
                                    name: "x".to_string(),
                                    value: Box::new(ASTNode {
                                        id: 9,
                                        data: ASTNodeData::Input("{x} + 2".to_string()),
                                    }),
                                }),
                            }],
                        }]),
                        else_: Some(vec![ASTNode {
                            id: 11,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "x".to_string(),
                                value: Box::new(ASTNode {
                                    id: 12,
                                    data: ASTNodeData::Input("{x} + 3".to_string()),
                                }),
                            }),
                        }]),
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(13)));
    }

    #[test]
    fn variable_assignment() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::VariableAssignment(VariableAssignment {
                name: "x".to_string(),
                value: Box::new(ASTNode {
                    id: 1,
                    data: ASTNodeData::Input("42".to_string()),
                }),
            }),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(42));
    }

    #[test]
    fn math_expression() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Input("2 + 2".to_string()),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::Int(4));
    }

    #[test]
    fn variable_reassignment() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Sequence(vec![
                ASTNode {
                    id: 1,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 2,
                            data: ASTNodeData::Input("42".to_string()),
                        }),
                    }),
                },
                ASTNode {
                    id: 3,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 4,
                            data: ASTNodeData::Input("24".to_string()),
                        }),
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(24));
    }

    #[test]
    fn variable_reassignment_with_math_expression() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Sequence(vec![
                ASTNode {
                    id: 1,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 2,
                            data: ASTNodeData::Input("42".to_string()),
                        }),
                    }),
                },
                ASTNode {
                    id: 3,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 4,
                            data: ASTNodeData::Input("{x} + 1".to_string()),
                        }),
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(43));
    }

    // #[test]
    // fn function_declaration() { // TODO
    //     let ast = ASTNode {
    //         id: 0,
    //         data: ASTNodeData::FunctionDeclaration(FunctionDeclaration {
    //             name: "foo".to_string(),
    //             argv: HashMap::new(),
    //             sequence: vec![ASTNode {
    //                 id: 1,
    //                 data: ASTNodeData::VariableAssignment(VariableAssignment {
    //                     name: "x".to_string(),
    //                     value: Box::new(ASTNode {
    //                         id: 2,
    //                         data: ASTNodeData::Input("42".to_string()),
    //                     }),
    //                 }),
    //             }],
    //         }),
    //     };
    //     let mut variables = HashMap::new();
    //     let return_value = exec_ast(&ast, &mut variables);
    //     assert_eq!(return_value, ReturnValue::None);
    // }
}
