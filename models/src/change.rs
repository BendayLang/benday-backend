use serde::{Deserialize, Serialize};

use crate::ast::ASTNode;

#[derive(Deserialize, Serialize)]
pub struct Change {
    #[serde(rename = "idPath")]
    pub id_path: Vec<usize>,
    #[serde(flatten)]
    pub data: ChangeData,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ChangeData {
    #[serde(rename = "replace")]
    Replace(ASTNode),
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "move")]
    Move(Move),
    #[serde(rename = "insert")]
    Insert(Insert),
}

#[derive(Deserialize, Serialize)]
pub struct Insert {
    #[serde(rename = "innerIdPath")]
    pub inner_id_path: Vec<usize>, // TODO quel est le meilleur format ?
    #[serde(rename = "astNode")]
    pub models: ASTNode,
}

#[derive(Deserialize, Serialize)]
pub struct Move {
    #[serde(rename = "newParentIdPath")]
    pub new_parent_id_path: Vec<usize>,
    #[serde(rename = "innerIdPath")]
    pub inner_id_path: Vec<usize>, // TODO quel est le meilleur format ?
}

// TODO -> specify the inner path of the node
// pub enum InnerPath {
//     /// Left or right = name or value
//     VariableAssignment(either::Either<(), ()>),
//     /// Left = name, right = value(index)
//     FunctionCall(either::Either<(), usize>),

//     While...
// }
