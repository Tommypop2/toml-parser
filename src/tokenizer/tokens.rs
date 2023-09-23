pub enum TokenKind {
    Start,
    Comment,
    End,
}
pub enum TokenValue {
    Comment(String),
}
pub struct Token {
    ind_kind: u32,
    ind_value: u32,
}
/**
 * The current tokenizer state
 * Stores the TokenKind, and TokenValue separately, so the enums can be as small as possible
 */
pub struct State {
    kinds: Vec<TokenKind>,
    values: Vec<TokenValue>,
}
impl State {
    pub fn new() -> Self {
        Self {
            kinds: vec![],
            values: vec![],
        }
    }
}
