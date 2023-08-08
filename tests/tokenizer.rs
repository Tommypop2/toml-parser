use std::fs;

use toml_parser::tokenizer;

#[test]
fn strings() {
    let toml = fs::read_to_string("./test.toml").unwrap();
    let res = tokenizer::tokenize(&toml).unwrap();
    dbg!(&res);
    assert_eq!(1, 1)
}
