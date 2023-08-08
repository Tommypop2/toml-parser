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
}

#[derive(Debug)]
pub struct PositionedToken {
    start: usize,
    end: usize,
    token: Token,
}
impl PositionedToken {
    pub fn new(start: usize, end: usize, token: Token) -> Self {
        PositionedToken { start, end, token }
    }
}
