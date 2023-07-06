use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub id_path: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(flatten)]
    pub type_: ErrorType,
    pub level: ErrorLevel,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ErrorLevel {
    Warning,
    Error,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ErrorType {
    InfiniteLoop { reaches: usize, max: usize },
}
