use toml_parser::tokenizer::cursor::Cursor;

#[test]
fn test_cursor() {
    let chars = "1234567";
    let cursor = Cursor::new(chars.chars());
    let constructed = cursor.collect::<String>();
    assert_eq!(chars.to_string(), constructed);
}
