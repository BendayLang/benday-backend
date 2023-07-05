use ast_node;
use serde::{Deserialize, Serialize};

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
    Replace(ast_node::ASTNode),
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
    pub ast_node: ast_node::ASTNode,
}

#[derive(Deserialize, Serialize)]
pub struct Move {
    #[serde(rename = "newParentIdPath")]
    pub new_parent_id_path: Vec<usize>,
    #[serde(rename = "innerIdPath")]
    pub inner_id_path: Vec<usize>, // TODO quel est le meilleur format ?
}

pub fn test_to_json() {
    let ast: ast_node::ASTNode = ast_node::ASTNode {
        id: 0,
        data: ast_node::ASTNodeData::Sequence(vec![]),
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
                ast_node: ast.clone(),
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
