#[cfg(test)]
mod tests {
    use crate::exectute::execute::exec_ast;
    use models::{ast::*, return_value::ReturnValue};
    use std::collections::HashMap;

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
                        is_builtin: true,
                        name: "print".to_string(),
                        argv: vec![ASTNode {
                            id: 4,
                            data: ASTNodeData::Input("{x}".to_string()),
                        }],
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(stdout, vec!["42"]);
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(42));
    }

    #[test]
    fn test_exec_ast_input() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Input("42".to_string()),
        };
        let mut variables = HashMap::new();
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::Int(42)));
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
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(42)));
    }

    #[test]
    fn test_exec_ast_builtin_function_call() {
        // TODO test the stdout
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::FunctionCall(FunctionCall {
                is_builtin: true,
                name: "print".to_string(),
                argv: vec![ASTNode {
                    id: 1,
                    data: ASTNodeData::Input("42".to_string()),
                }],
            }),
        };
        let mut variables = HashMap::new();
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(stdout, vec!["42"]);
    }

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
                            data: ASTNodeData::Input("{x}".to_string()),
                        }],
                    }),
                },
            ]),
        };
        let mut variables = HashMap::new();
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(42)));
        assert_eq!(stdout, vec!["42"]);
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
                sequence: vec![
                    ASTNode {
                        id: 4,
                        data: ASTNodeData::FunctionCall(FunctionCall {
                            is_builtin: true,
                            name: "print".to_string(),
                            argv: vec![ASTNode {
                                id: 5,
                                data: ASTNodeData::Input("{x}".to_string()),
                            }],
                        }),
                    },
                    ASTNode {
                        id: 2,
                        data: ASTNodeData::VariableAssignment(VariableAssignment {
                            name: "x".to_string(),
                            value: Box::new(ASTNode {
                                id: 3,
                                data: ASTNodeData::Input("{x} + 1".to_string()),
                            }),
                        }),
                    },
                ],
            }),
        };
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), ReturnValue::Int(0));
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(10)));
        assert_eq!(
            stdout,
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        );
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
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert_eq!(variables.get("x"), Some(&ReturnValue::Int(13)));
        assert!(stdout.is_empty());
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
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
        assert!(stdout.is_empty());
        assert_eq!(variables.get("x").unwrap(), &ReturnValue::Int(42));
    }

    #[test]
    fn math_expression() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::Input("2 + 2 - {x}".to_string()),
        };
        let mut variables = HashMap::from([("x".to_string(), ReturnValue::Int(42))]);
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
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
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
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
        let mut stdout = Vec::new();
        let mut id_path = Vec::new();
        let return_value = exec_ast(&ast, &mut variables, &mut id_path, &mut stdout);
        assert_eq!(return_value, Ok(ReturnValue::None));
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
