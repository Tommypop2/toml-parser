use std::fs;

use toml_parser::codegen::{generate, json::JsonGenerator};

#[test]
fn strings() {
    let toml = fs::read_to_string("./test.toml").unwrap();
    let ast = toml_parser::get_ast(&toml).unwrap();
    // dbg!(&ast);
    let generated = generate(ast, JsonGenerator);
    dbg!(generated);
    assert_eq!(1, 1)
}
