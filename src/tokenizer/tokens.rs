pub enum TokenKind {
    Start,
    End,
    Ident(String),
    String,
}
pub enum Pos {
    DUMMY,
    POS(u32),
}
pub struct Span {
    start: Pos,
    end: Pos,
}
pub struct Token {
    span: Span,
    kind: TokenKind,
}
