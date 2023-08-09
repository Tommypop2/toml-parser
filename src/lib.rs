pub mod codegen;
pub mod parser;
pub mod tokenizer;
use parser::{parse, AST};
use tokenizer::{tokenize, tokens::Token, Tokens};
fn remove_whitespace(tokens: &mut Tokens) {
    tokens.retain(|e| {
        if let Token::Whitespace(_) = &e.token {
            false
        } else {
            true
        }
    })
}
pub fn run(toml: &str) -> Result<AST, ()> {
    let mut tokens = tokenize(toml).unwrap();
    remove_whitespace(&mut tokens);
    dbg!(&tokens);
    let res = parse(tokens);
    res
}
