#[derive(Clone, Debug, PartialEq)]
pub enum Lit {
    Str,
    Float,
    Int,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Start,
    Comment,
    Identifier,
    End,
    Temp,
    Lit(Lit),
    // Single Char tokens
    Equals,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}
// #[derive(Debug)]
// pub enum TokenValue {
//     Comment(String),
// }
pub type TokenValue = String;
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub ind_value: Option<usize>,
}
impl Token {
    pub fn new(kind: TokenKind, ind_value: Option<usize>) -> Self {
        Self { kind, ind_value }
    }
}
