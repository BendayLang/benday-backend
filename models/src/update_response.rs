use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ErrorMessage {
    #[serde(rename = "idPath")]
    pub id_path: Vec<usize>,
    #[serde(rename = "customMessage", skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[serde(rename = "type")]
    pub type_: ErrorType,
    pub level: ErrorLevel,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
// #[serde(tag = "type", content = "data")]
pub enum ErrorType {
    InfiniteLoop,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum ErrorLevel {
    Warning,
    Error,
}
