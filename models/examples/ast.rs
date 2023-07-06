use models::ast::*;
use std::collections::HashMap;

pub fn ast_example() -> ASTNode {
    ASTNode {
        id: 0,
        data: ASTNodeData::Sequence(vec![
            ASTNode {
                id: 1,
                data: ASTNodeData::VariableAssignment(VariableAssignment {
                    name: "age de Bob".to_string(),
                    value: Box::new(ASTNode {
                        id: 2,
                        data: ASTNodeData::Input("6".to_string()),
                    }),
                }),
            },
            ASTNode {
                id: 3,
                data: ASTNodeData::While(While {
                    is_do: false,
                    condition: Box::new(ASTNode {
                        id: 4,
                        data: ASTNodeData::Input("{age de Bob} < 13".to_string()),
                    }),
                    sequence: vec![
                        ASTNode {
                            id: 5,
                            data: ASTNodeData::VariableAssignment(VariableAssignment {
                                name: "age de Bob".to_string(),
                                value: Box::new(ASTNode {
                                    id: 6,
                                    data: ASTNodeData::Input("{age de Bob} + 1".to_string()),
                                }),
                            }),
                        },
                        ASTNode {
                            id: 7,
                            data: ASTNodeData::FunctionCall(FunctionCall {
                                name: "print".to_string(),
                                is_builtin: true,
                                argv: vec![ASTNode {
                                    id: 8,
                                    data: ASTNodeData::Input(
                                        "Bravo Bob ! tu as maintenant \"{age de Bob}\" ans !"
                                            .to_string(),
                                    ),
                                }],
                            }),
                        },
                    ],
                }),
            },
            ASTNode {
                id: 9,
                data: ASTNodeData::FunctionCall(FunctionCall {
                    name: "print".to_string(),
                    is_builtin: true,
                    argv: vec![ASTNode {
                        id: 10,
                        data: ASTNodeData::Input(
                            "Bob est parti a l'age de {age de Bob} !".to_string(),
                        ),
                    }],
                }),
            },
            ASTNode {
                id: 11,
                data: ASTNodeData::FunctionDeclaration(FunctionDeclaration {
                    name: "print text and number".to_string(),
                    sequence: vec![
                        ASTNode {
                            id: 14,
                            data: ASTNodeData::FunctionCall(FunctionCall {
                                name: "print".to_string(),
                                is_builtin: true,
                                argv: vec![ASTNode {
                                    id: 15,
                                    data: ASTNodeData::Input("{text}".to_string()),
                                }],
                            }),
                        },
                        ASTNode {
                            id: 16,
                            data: ASTNodeData::FunctionCall(FunctionCall {
                                name: "print".to_string(),
                                is_builtin: true,
                                argv: vec![ASTNode {
                                    id: 17,
                                    data: ASTNodeData::Input("{number}".to_string()),
                                }],
                            }),
                        },
                    ],
                    argv: HashMap::from([
                        (
                            "text".to_string(),
                            VariableAssignment {
                                name: "text".to_string(),
                                value: Box::new(ASTNode {
                                    id: 12,
                                    data: ASTNodeData::Input("".to_string()),
                                }),
                            },
                        ),
                        (
                            "number".to_string(),
                            VariableAssignment {
                                name: "number".to_string(),
                                value: Box::new(ASTNode {
                                    id: 13,
                                    data: ASTNodeData::Input("".to_string()),
                                }),
                            },
                        ),
                    ]),
                }),
            },
        ]),
    }
}

fn main() {
    use std::io::Write;
    let ast = ast_example();
    let json = serde_json::to_string_pretty(&ast).unwrap();
    let mut file = std::fs::File::create("./models/examples/ast.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
