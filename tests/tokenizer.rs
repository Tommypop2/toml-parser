use toml_parser::tokenizer::tokenize;

#[test]
fn tokenizer_test() {
    let toml = "key = 123";
    let result = tokenize(toml.chars());
    assert_eq!(1, 0)
}
