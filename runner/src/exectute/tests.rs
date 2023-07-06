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

    assert_eq!(
        expected_stdout.map_or(Vec::new(), |v| v),
        stdout_result,
        "stdout"
    );
    assert_eq!(
        expected_variables.map_or(VariableMap::new(), |v| v),
        var_result,
        "variables"
    );
    assert_eq!(
        expected_return_value.map_or(Ok(ReturnValue::None), |v| v),
        return_value,
        "return value"
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
fn should_assign_variable_and_print_it() {
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
fn should_return_value_when_raw_text() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::RawText("42".to_string()),
    };
    test(
        ast,
        Some(HashMap::new()),
        None,
        None,
        Some(Ok(ReturnValue::Int(42))),
    );
}

#[test]
fn should_return_when_raw_text_and_stop_there() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![
            ASTNode {
                id: 1,
                data: ASTNodeData::RawText("42".to_string()),
            },
            ASTNode {
                id: 2,
                data: ASTNodeData::RawText("24".to_string()),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::FunctionCall(FunctionCall {
                    is_builtin: true,
                    name: "print".to_string(),
                    argv: vec![ASTNode {
                        id: 4,
                        data: ASTNodeData::RawText("42".to_string()),
                    }],
                }),
            },
        ]),
    };
    test(ast, None, None, None, Some(Ok(ReturnValue::Int(42))));
}
#[test]
fn should_print_raw_text() {
    let ast = ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![ASTNode {
            id: 1,
            data: ASTNodeData::FunctionCall(FunctionCall {
                is_builtin: true,
                name: "print".to_string(),
                argv: vec![ASTNode {
                    id: 2,
                    data: ASTNodeData::RawText("42".to_string()),
                }],
            }),
        }]),
    };
    test(
        ast,
        None,
        Some(vec!["42".to_string()]),
        None,
        Some(Ok(ReturnValue::None)),
    );
}

#[test]
fn should_print_variable_in_a_while_loop() {
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
    test(
        ast,
        Some(HashMap::from([(("x".to_string(), 0), ReturnValue::Int(0))])),
        Some(vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ]),
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(10),
        )])),
        Some(Ok(ReturnValue::None)),
    );
}

#[test]
fn sould_reassign_variable_if_condition_is_true() {
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
    test(
        ast,
        None,
        None,
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(13),
        )])),
        Some(Ok(ReturnValue::None)),
    );
}

#[test]
fn should_return_math_expression_result() {
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
                data: ASTNodeData::RawText("2 + 2 - {x}".to_string()),
            },
        ]),
    };
    test(
        ast,
        None,
        None,
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(42),
        )])),
        Some(Ok(ReturnValue::Int(-38))),
    );
}

#[test]
fn should_reassign_variable() {
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
    test(
        ast,
        None,
        None,
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(24),
        )])),
        Some(Ok(ReturnValue::None)),
    );
}

#[test]
fn should_reassign_variable_and_keep_original_scope() {
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
                data: ASTNodeData::Sequence(vec![ASTNode {
                    id: 4,
                    data: ASTNodeData::VariableAssignment(VariableAssignment {
                        name: "x".to_string(),
                        value: Box::new(ASTNode {
                            id: 5,
                            data: ASTNodeData::RawText("24".to_string()),
                        }),
                    }),
                }]),
            },
        ]),
    };
    test(
        ast,
        None,
        None,
        Some(HashMap::from([(
            ("x".to_string(), 0),
            ReturnValue::Int(24),
        )])),
        None,
    );
}

// fn function_declaration() { // TODO
