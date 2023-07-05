#[cfg(test)]
mod tests {
    use super::*;

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
                            data: ASTNodeData::Input("x + 1".to_string()),
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

    #[test]
    fn function_call() {
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
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
    }

    #[test]
    fn function_declaration() {
        let ast = ASTNode {
            id: 0,
            data: ASTNodeData::FunctionDeclaration(FunctionDeclaration {
                name: "foo".to_string(),
                argv: vec![],
                body: Box::new(ASTNode {
                    id: 1,
                    data: ASTNodeData::Sequence(vec![
                        ASTNode {
                            id: 2,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "x".to_string(),
                                value: Box::new(ASTNode {
                                    id: 3,
                                    data: ASTNodeData::Input("42".to_string()),
                                }),
                            }),
                        },
                        ASTNode {
                            id: 4,
                            data: ASTNodeData::FunctionCall(FunctionCall {
                                is_builtin: true,
                                name: "print".to_string(),
                                argv: vec![ASTNode {
                                    id: 5,
                                    data: ASTNodeData::Input("x".to_string()),
                                }],
                            }),
                        },
                    ]),
                }),
            }),
        };
        let mut variables = HashMap::new();
        let return_value = exec_ast(&ast, &mut variables);
        assert_eq!(return_value, ReturnValue::None);
    }
}
