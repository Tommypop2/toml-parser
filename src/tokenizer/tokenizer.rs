use std::str::Chars;

use super::tokens::{State, TokenKind, TokenValue};

pub struct Tokenizer {
    state: State,
}
impl Tokenizer {
    pub fn new(state: State) -> Self {
        Self { state }
    }
    pub fn tokenize(&self, chars: &mut Chars) {
        let mut state: TokenKind = TokenKind::Start;
        let value: Option<String> = None;

        for char in chars {
            match state {
                TokenKind::Start => match char {
                    '#' => {
                        
                    }
                    _ => {}
                },
                TokenKind::Comment => {}
                TokenKind::End => {}
            }
        }
    }
}
