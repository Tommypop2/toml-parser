use super::{
    cursor::Cursor,
    tokens::{Span, Token, TokenKind, DUMMY_SP},
};

pub struct Tokenizer<'a> {
    state: TokenKind,
    cursor: Cursor<'a>,
}
impl<'a> Tokenizer<'a> {
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self {
            state: TokenKind::Start,
            cursor,
        }
    }
    pub fn next(&mut self) {
        let c = self.cursor.next();
        match &self.state {
            TokenKind::Start => match c {
                ' ' | '\n' | '\t' | '\r' => {}
                'A'..='Z' | 'a'..='z' => self.state = TokenKind::Ident,
                _ => {}
            },
            _ => {}
        }
    }
}
