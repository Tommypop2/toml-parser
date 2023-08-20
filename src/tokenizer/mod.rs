use std::str::Chars;

use self::{cursor::Cursor, tokenizer::Tokenizer};

pub mod cursor;
pub mod tokenizer;
pub mod tokens;

pub fn tokenize(chrs: Chars) {
    let cursor = Cursor::new(chrs);
    let tokenizer = Tokenizer::new(cursor);
    let collected = tokenizer.into_iter().collect::<Vec<_>>();
    dbg!(collected);
}
