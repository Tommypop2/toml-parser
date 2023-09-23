use toml_parser::tokenizer::tokenize;

#[test]
fn tokenizer_test() {
    let toml = "# Hello World!
    asd = 123
    [split.identifier.thing]";
    let tokenizer = tokenize(&mut toml.chars());
    dbg!(tokenizer.tokens, tokenizer.token_values);

    assert_eq!(1, 0)
}
