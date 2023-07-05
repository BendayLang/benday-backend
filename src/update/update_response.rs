use ast_node;
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

pub fn test_to_json() {
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
