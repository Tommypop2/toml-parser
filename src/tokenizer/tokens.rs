// #[derive(Debug)]
// pub struct Comment {
//     content: String,
// }
#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    StringLit(String),
    Integer(i64),
    Float(f64),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Comment(String),
    Ident(String),
    Lit(Lit),
    Punctuation(char),
    Whitespace(char),
}

#[derive(Debug, Clone)]
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
