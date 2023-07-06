use crate::runner::IdPath;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub id_path: IdPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    pub error: ErrorType,
    level: ErrorLevel,
}

impl ErrorMessage {
    pub fn new(id_path: IdPath, type_: ErrorType, custom_message: Option<&str>) -> Self {
        Self {
            id_path,
            custom_message: custom_message.map_or(None, |s| Some(s.to_string())),
            error: type_,
            level: ErrorLevel::Error,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ErrorLevel {
    Warning,
    Error,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ValueType {
    #[serde(rename = "string")]
    String_,
    Int,
    Float,
    Bool,
    None,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum ErrorType {
    InfiniteLoop {
        reaches: usize,
        max: usize,
    },
    VariableExpansionError(VariableExpansionError),
    RootIsNotSequence,
    ConditionAlwaysFalse,
    ConditionAlwaysTrue,
    InvalidType {
        accepted: Vec<ValueType>,
        found: ValueType,
    },
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum VariableExpansionError {
    VariableNotExpandable,
    MissingClosingBracket,
    MissingOpeningBracket,
    BracketOrder,
    VariableNotFound(String),
}
