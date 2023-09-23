use std::str::Chars;

use self::{tokenizer::Tokenizer, tokens::State};

pub mod tokenizer;
pub mod tokens;
pub fn tokenize(chars: &mut Chars) {
    let state = State::new();
    let tokenizer = Tokenizer::new(state);
    let res = tokenizer.tokenize(chars);
    dbg!(res)
}
