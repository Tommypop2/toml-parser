use std::str::Chars;

use self::tokenizer::Tokenizer;

pub mod tokenizer;
pub mod tokens;
pub fn tokenize(chars: &mut Chars) -> Tokenizer {
    let mut tokenizer: Tokenizer = Tokenizer::new();
    tokenizer.tokenize(chars);
    tokenizer
}
