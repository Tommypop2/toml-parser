use super::{
    cursor::Cursor,
    tokens::{Pos, Span, Token, TokenKind},
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
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let TokenKind::End = self.state {
            return None;
        }
        let mut result = Token {
            kind: TokenKind::End,
            span: Span {
                start: Pos::POS(self.cursor.pos),
                end: Pos::DUMMY,
            },
            value: None,
        };
        loop {
            let c = match self.cursor.peek() {
                Some(c) => c,
                None => break,
            };
            match &self.state {
                TokenKind::Start => match c {
                    '\n' | '\t' | '\r' | ' ' => {
                        self.cursor.next();
                    }
                    'A'..='Z' | 'a'..='z' => {
                        result.kind = TokenKind::Ident;
                        result.span.end = Pos::POS(self.cursor.pos);
                        self.cursor.next();
                    }
                    _ => {
                        self.cursor.next();
                    }
                },
                TokenKind::Ident => match c {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        self.cursor.next();
                    }
                    _ => {
                        break;
                    }
                },

                _ => {}
            }
            self.state = result.kind.clone();
        }
        self.state = result.kind.clone();
        result.span.end = Pos::POS(self.cursor.pos);
        Some(result)
    }
}
