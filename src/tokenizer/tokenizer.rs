use super::{cursor::Cursor, tokens::Token};

pub struct Tokenizer<'a> {
    state: Token,
    cursor: Cursor<'a>,
}
impl<'a> Tokenizer<'a> {
    fn next(&mut self) {
        match self.state {
            Token::Ident => {}
            Token::String => {}
        }
    }
}
