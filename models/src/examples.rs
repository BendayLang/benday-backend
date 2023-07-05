use crate::{update_request::*, update_response::*, *};
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

pub fn request_json() {
    let ast: ASTNode = ASTNode {
        id: 0,
        data: crate::ASTNodeData::Sequence(vec![]),
    };
    let changes = vec![
        Change {
            id_path: vec![0],
            data: ChangeData::Replace(ast.clone()),
        },
        Change {
            id_path: vec![0],
            data: ChangeData::Insert(Insert {
                inner_id_path: vec![0],
                models: ast.clone(),
            }),
        },
        Change {
            id_path: vec![0],
            data: ChangeData::Move(Move {
                inner_id_path: vec![0],
                new_parent_id_path: vec![0],
            }),
        },
        Change {
            id_path: vec![0],
            data: ChangeData::Delete,
        },
    ];
    let json = serde_json::to_string(&changes).unwrap();
    println!("{}", json);
}

pub fn response_json() {
    let error_message = vec![
        ErrorMessage {
            custom_message: None,
            id_path: vec![0],
            type_: ErrorType::InfiniteLoop,
            level: ErrorLevel::Warning,
        },
        ErrorMessage {
            custom_message: Some("test".to_string()),
            id_path: vec![0],
            type_: ErrorType::InfiniteLoop,
            level: ErrorLevel::Warning,
        },
    ];
    let json = serde_json::to_string(&error_message).unwrap();
    let sejson = serde_json::from_str::<Vec<ErrorMessage>>(&json).unwrap();
    assert_eq!(sejson, error_message);
    println!("{}", json);
}
