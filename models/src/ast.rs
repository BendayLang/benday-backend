use std::collections::HashMap;

pub type Id = u32;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ASTNode {
    pub id: Id,
    #[serde(flatten)]
    pub data: ASTNodeData,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ASTNodeData {
    Sequence(Sequence),
    While(While),
    IfElse(IfElse),
    RawText(String), // could be rename to Literal
    VariableAssignment(VariableAssignment),
    FunctionCall(FunctionCall),
    FunctionDeclaration(FunctionDeclaration),
}

type Sequence = Vec<ASTNode>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct While {
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
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub name: String, // TODO: un id ?
    pub is_builtin: bool,
    pub argv: Vec<ASTNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub argv: HashMap<String, VariableAssignment>,
    pub sequence: Sequence,
}
