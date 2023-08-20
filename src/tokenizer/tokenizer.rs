use super::{
    cursor::Cursor,
    tokens::{Token, TokenKind},
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
        let start_ind = self.cursor.next_i;
        let c = self.cursor.next();
        match &self.state {
            TokenKind::Start => match c {
                ' ' | '\n' | '\t' | '\r' => {}
                'A'..='Z' | 'a'..='z' => {}
                _ => {}
            },
            TokenKind::Ident(i) => {}
            _ => {}
        }
    }
}
