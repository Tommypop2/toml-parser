use std::fs;

use toml_parser::codegen::generate;

#[test]
fn strings() {
    let toml = fs::read_to_string("./test.toml").unwrap();
    let ast = toml_parser::run(&toml).unwrap();
    let generated = generate(ast);
    dbg!(generated);
    assert_eq!(1, 1)
}
