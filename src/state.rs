use ast_node;
use std::{path::PathBuf, sync::Mutex};

pub struct State {
    pub project_path: Mutex<PathBuf>,
    pub ast: Mutex<Vec<ast_node::ASTNode>>,
}
