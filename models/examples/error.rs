use models::error::*;

pub fn error_example() -> Vec<ErrorMessage> {
    vec![
        ErrorMessage::new(
            vec![0],
            ErrorType::InfiniteLoop {
                reaches: 546,
                max: 500,
            },
            None,
        ),
        ErrorMessage::new(
            vec![0],
            ErrorType::InfiniteLoop {
                reaches: 546,
                max: 500,
            },
            Some("test"),
        ),
    ]
}

fn main() {
    use std::io::Write;
    let ast = error_example();
    let json = serde_json::to_string_pretty(&ast).unwrap();
    let mut file = std::fs::File::create("./models/examples/error.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
