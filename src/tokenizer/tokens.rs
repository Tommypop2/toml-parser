pub enum TokenKind {
    Start,
    End,
    Ident,
    String,
    Invalid,
}
pub enum TokenValue {
    Ident(String),
    String(String),
}
pub enum Pos {
    DUMMY,
    POS(u32),
}
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}
pub const DUMMY_SP: Span = Span {
    start: Pos::DUMMY,
    end: Pos::DUMMY,
};
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
