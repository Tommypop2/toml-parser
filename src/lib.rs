pub mod codegen;
pub mod parser;
pub mod tokenizer;
use codegen::{generate, json::JsonGenerator};
use parser::{parse, AST};
use tokenizer::{tokenize, tokens::Token, Tokens};
use wasm_bindgen::prelude::wasm_bindgen;
fn remove_whitespace(tokens: &mut Tokens) {
    tokens.retain(|e| {
        if let Token::Whitespace(_) = &e.token {
            false
        } else {
            true
        }
    })
}
#[wasm_bindgen]
pub fn into_json(toml: &str) -> String {
    let ast = get_ast(toml).unwrap();
    let json = generate(ast, JsonGenerator);
    json
}
pub fn get_ast(toml: &str) -> Result<AST, ()> {
    let mut tokens = tokenize(toml).unwrap();
    remove_whitespace(&mut tokens);
    let res = parse(tokens);
    res
}
