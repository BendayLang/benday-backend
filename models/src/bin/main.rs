#[cfg(debug_assertions)]
fn main() {
    let ast = ast_node::ast_example();

    let json = serde_json::to_string(&ast).unwrap();
    println!("{}", json);
}

#[cfg(not(debug_assertions))]
fn main() {
    println!("nothing to do");
}
