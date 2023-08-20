#[derive(Clone, Debug)]
pub enum TokenKind {
    WhiteSpace,
    LineBreak,
    Start,
    End,
    Ident,
    String,
    Invalid,
}
#[derive(Debug)]
pub enum TokenValue {
    Ident(String),
    String(String),
}
#[derive(Debug)]
pub enum Pos {
    DUMMY,
    POS(usize),
}
#[derive(Debug)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}
pub const DUMMY_SP: Span = Span {
    start: Pos::DUMMY,
    end: Pos::DUMMY,
};
#[derive(Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
}
impl Token {
    pub fn new(span: Span, kind: TokenKind, value: Option<TokenValue>) -> Self {
        Self { span, kind, value }
    }
}
