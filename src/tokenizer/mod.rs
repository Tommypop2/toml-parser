use std::str::Chars;

use self::{cursor::Cursor, tokenizer::Tokenizer, tokens::Token};

mod cursor;
mod tokenizer;
mod tokens;

pub fn tokenize(chrs: Chars) {
    let cursor = Cursor::new(chrs);
    let mut tokenizer = Tokenizer::new(cursor);
    let tokens: Vec<Token> = vec![];
    loop {
        let next = tokenizer.next();
        
    }
}
