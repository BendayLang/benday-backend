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
    Sequence(Sequence),
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

type Sequence = Vec<ASTNode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct While {
    #[serde(rename = "isDo")]
    pub is_do: bool,
    pub condition: Box<ASTNode>,
    pub sequence: Sequence,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct If {
    pub condition: Box<ASTNode>,
    pub sequence: Sequence,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct IfElse {
    #[serde(rename = "if")]
    pub if_: If,
    pub elif: Option<Vec<If>>,
    #[serde(rename = "else")]
    pub else_: Option<Sequence>,
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
    pub sequence: Sequence,
}
