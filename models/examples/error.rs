use models::error::*;

pub fn error_example() -> Vec<ErrorMessage> {
    vec![
        ErrorMessage {
            custom_message: None,
            id_path: vec![0],
            type_: ErrorType::InfiniteLoop {
                reaches: 546,
                max: 500,
            },
            level: ErrorLevel::Warning,
        },
        ErrorMessage {
            custom_message: Some("test".to_string()),
            id_path: vec![0],
            type_: ErrorType::InfiniteLoop {
                reaches: 546,
                max: 500,
            },
            level: ErrorLevel::Warning,
        },
    ]
}

fn main() {
    use std::io::Write;
    let ast = error_example();
    let json = serde_json::to_string_pretty(&ast).unwrap();
    let mut file = std::fs::File::create("./models/examples/error.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
