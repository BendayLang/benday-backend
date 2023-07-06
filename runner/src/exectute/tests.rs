use super::runner;
use crate::exectute::{execute::execute_node, VariableMap};
use models::{
    ast::*,
    error::ErrorMessage,
    return_value::{self, ReturnValue},
    runner::RunnerResult,
};
use std::{collections::HashMap, env::var};

fn test(
    ast: ASTNode,
    basic_variables: Option<VariableMap>,
    expected_stdout: Option<Vec<String>>,
    expected_variables: Option<VariableMap>,
    expected_return_value: Option<RunnerResult>,
) {
    let (return_value, stdout_result, var_result) = if let Some(mut variables) = basic_variables {
        let mut stdout: Vec<String> = Vec::<String>::new();
        let id_path = &mut Vec::new();
        let runner_result = execute_node(&ast, &mut variables, id_path, &mut stdout);
        (runner_result, stdout, variables)
    } else {
        runner(&ast)
    };

    assert_eq!(stdout_result, expected_stdout.map_or(Vec::new(), |v| v));
    assert_eq!(
        var_result,
        expected_variables.map_or(VariableMap::new(), |v| v)
    );
    assert_eq!(
        return_value,
        expected_return_value.map_or(Ok(ReturnValue::None), |v| v)
    );
}

#[test]
fn should_do_nothing_when_empty_sequence() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![]),
    };
    test(ast, None, None, None, None);
}

#[test]
fn should_error_when_root_node_is_not_a_sequence() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::RawText("Hello world".to_string()),
    };
    let error_message =
        ErrorMessage::new(vec![], models::error::ErrorType::RootIsNotSequence, None);
    test(ast, None, None, None, Some(Err(vec![error_message])));
}

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
                        data: ASTNodeData::RawText("42".to_string()),
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
                        data: ASTNodeData::RawText("{x}".to_string()),
                    }],
                }),
            },
        ]),
    };
    test(
        ast,
        None,
        Some(vec!["42".to_string()]),
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(42),
        )])),
        Some(Ok(ReturnValue::None)),
    );
}

#[test]
fn input() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::RawText("42".to_string()),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::Int(42)));
}

#[test]
fn builtin_function_call() {
    // TODO test the stdout
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::FunctionCall(FunctionCall {
            is_builtin: true,
            name: "print".to_string(),
            argv: vec![ASTNode {
                id: 1,
                data: ASTNodeData::RawText("42".to_string()),
            }],
        }),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(stdout, vec!["42"]);
}

#[test]
fn sequence() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![
            ASTNode {
                id: 1,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "x".to_string(),
                    value: Box::new(ASTNode {
                        id: 2,
                        data: ASTNodeData::RawText("42".to_string()),
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
                        data: ASTNodeData::RawText("{x}".to_string()),
                    }],
                }),
            },
        ]),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(
        variables.get(&("x".to_string(), 0)),
        Some(&ReturnValue::Int(42))
    );
    assert_eq!(stdout, vec!["42"]);
}

#[test]
fn should_nanna_while() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::While(While {
            is_do: false,
            condition: Box::new(ASTNode {
                id: 1,
                data: ASTNodeData::RawText("{x} < 10".to_string()),
            }),
            sequence: vec![
                ASTNode {
                    id: 4,
                    data: ASTNodeData::FunctionCall(FunctionCall {
                        is_builtin: true,
                        name: "print".to_string(),
                        argv: vec![ASTNode {
                            id: 5,
                            data: ASTNodeData::RawText("{x}".to_string()),
                        }],
                    }),
                },
                ASTNode {
                    id: 2,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 3,
                            data: ASTNodeData::RawText("{x} + 1".to_string()),
                        }),
                    }),
                },
            ],
        }),
    };
    let mut variables = HashMap::new();
    variables.insert(("x".to_string(), 0), ReturnValue::Int(0));
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(
        variables.get(&("x".to_string(), 0)),
        Some(&ReturnValue::Int(10))
    );
    assert_eq!(
        stdout,
        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
    );
}

#[test]
fn if_else() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![
            ASTNode {
                id: 1,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "x".to_string(),
                    value: Box::new(ASTNode {
                        id: 2,
                        data: ASTNodeData::RawText("10".to_string()),
                    }),
                }),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::IfElse(IfElse {
                    if_: If {
                        condition: Box::new(ASTNode {
                            id: 4,
                            data: ASTNodeData::RawText("{x} > 10".to_string()),
                        }),
                        sequence: vec![ASTNode {
                            id: 5,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "x".to_string(),
                                value: Box::new(ASTNode {
                                    id: 6,
                                    data: ASTNodeData::RawText("{x} + 1".to_string()),
                                }),
                            }),
                        }],
                    },
                    elif: Some(vec![If {
                        condition: Box::new(ASTNode {
                            id: 7,
                            data: ASTNodeData::RawText("{x} > 20".to_string()),
                        }),
                        sequence: vec![ASTNode {
                            id: 8,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "x".to_string(),
                                value: Box::new(ASTNode {
                                    id: 9,
                                    data: ASTNodeData::RawText("{x} + 2".to_string()),
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
                                data: ASTNodeData::RawText("{x} + 3".to_string()),
                            }),
                        }),
                    }]),
                }),
            },
        ]),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(
        variables.get(&("x".to_string(), 0)),
        Some(&ReturnValue::Int(13))
    );
    assert!(stdout.is_empty());
}

#[test]
fn variable_assignment() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![ASTNode {
            id: 1,
            data: ASTNodeData::VariableAssignment(VariableAssignment {
                name: "x".to_string(),
                value: Box::new(ASTNode {
                    id: 2,
                    data: ASTNodeData::RawText("42".to_string()),
                }),
            }),
        }]),
    };
    let (return_value, stdout, variables) = runner(&ast);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert!(stdout.is_empty());
    assert_eq!(
        variables.get(&("x".to_string(), 0)),
        Some(&ReturnValue::Int(42))
    );
}

#[test]
fn math_expression() {
    let ast = ASTNode {
        id: 1,
        data: ASTNodeData::RawText("2 + 2 - {x}".to_string()),
    };
    let mut variables: VariableMap = HashMap::from([(("x".to_string(), 0), ReturnValue::Int(42))]);
    let mut stdout = Vec::new();
    let mut id_path = vec![0, 1];
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::Int(-38)));
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
                        data: ASTNodeData::RawText("42".to_string()),
                    }),
                }),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "x".to_string(),
                    value: Box::new(ASTNode {
                        id: 4,
                        data: ASTNodeData::RawText("24".to_string()),
                    }),
                }),
            },
        ]),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(
        variables.get(&("x".to_string(), 0)),
        Some(&ReturnValue::Int(24))
    );
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
                        data: ASTNodeData::RawText("42".to_string()),
                    }),
                }),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "x".to_string(),
                    value: Box::new(ASTNode {
                        id: 4,
                        data: ASTNodeData::RawText("{x} + 1".to_string()),
                    }),
                }),
            },
        ]),
    };
    let mut variables = HashMap::new();
    let mut stdout = Vec::new();
    let mut id_path = Vec::new();
    let return_value = execute_node(&ast, &mut variables, &mut id_path, &mut stdout);
    assert_eq!(return_value, Ok(ReturnValue::None));
    assert_eq!(
        variables.get(&("x".to_string(), 0)).unwrap(),
        &ReturnValue::Int(43)
    );
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
