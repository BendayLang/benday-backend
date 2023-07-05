use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ASTNode {
    pub id: u32,
    #[serde(flatten)]
    pub data: ASTNodeData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum ASTNodeData {
    #[serde(rename = "sequence")]
    Sequence(Vec<ASTNode>),
    #[serde(rename = "while")]
    While(While),
    #[serde(rename = "ifElse")]
    IfElse(IfElse),
    #[serde(rename = "input")]
    Input(String),
    #[serde(rename = "variableAssignment")]
    VariableAssignment(VariableAssignment),
    #[serde(rename = "functionCall")]
    FunctionCall(FunctionCall),
    #[serde(rename = "functionDeclaration")]
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct While {
    #[serde(rename = "isDo")]
    pub is_do: bool,
    pub condition: Box<ASTNode>,
    pub sequence: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct If {
    pub condition: Box<ASTNode>,
    pub sequence: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct IfElse {
    #[serde(rename = "if")]
    pub if_: If,
    pub elif: Option<Vec<If>>,
    #[serde(rename = "else")]
    pub else_: Option<If>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Box<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct FunctionCall {
    pub name: String, // TODO: un id ?
    #[serde(rename = "isBuiltin")]
    pub is_builtin: bool,
    pub argv: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub argv: HashMap<String, VariableAssignment>,
}

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
