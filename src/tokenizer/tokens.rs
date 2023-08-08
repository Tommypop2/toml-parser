// #[derive(Debug)]
// pub struct Comment {
//     content: String,
// }
#[derive(Debug)]
pub enum Token {
    Comment(String),
    Ident(String),
    StringLit(String),
    Integer(i64),
    Float(f64),
    Punctuation(char),
    Whitespace(char),
}

#[derive(Debug)]
pub struct PositionedToken {
    pub start: usize,
    pub end: usize,
    pub token: Token,
}
impl PositionedToken {
    pub fn new(start: usize, end: usize, token: Token) -> Self {
        PositionedToken { start, end, token }
    }
}
