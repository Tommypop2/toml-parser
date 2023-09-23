use std::str::Chars;

use super::tokens::{Lit, Token, TokenKind, TokenValue};

pub struct Tokenizer {
    value: Option<String>,
    state: TokenKind,
    pub token_values: Vec<TokenValue>,
    pub tokens: Vec<Token>,
}
impl Tokenizer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            value: None,
            token_values: vec![],
            state: TokenKind::Start,
        }
    }
    fn set_state(&mut self, state: TokenKind) {
        // Push our previous state
        if self.state != TokenKind::Temp {
            let ind_value = if let Some(s) = &self.value {
                self.token_values.push(s.clone());
                Some(self.token_values.len() - 1)
            } else {
                None
            };
            self.tokens.push(Token::new(self.state.clone(), ind_value));
        }
        self.state = state;
        self.value = None;
    }
    /**
     * Adds a char to the currently accumulating value string.
     */
    fn accumulate(&mut self, char: char) {
        if let Some(s) = &mut self.value {
            s.push(char)
        } else {
            self.value = Some(String::from(char))
        }
    }
    pub fn tokenize(&mut self, chars: &mut Chars) {
        for (i, char) in chars.enumerate() {
            match &self.state {
                TokenKind::Start | TokenKind::Temp => match char {
                    '#' => self.set_state(TokenKind::Comment),
                    'a'..='z' | 'A'..='Z' => {
                        self.set_state(TokenKind::Identifier);
                        self.accumulate(char)
                    }
                    '=' => self.set_state(TokenKind::Equals),
                    '"' => {
                        self.set_state(TokenKind::Lit(Lit::Str));
                    }
                    '0'..='9' => {
                        self.set_state(TokenKind::Lit(Lit::Int));
                        self.accumulate(char)
                    }
                    '[' => {
                        self.set_state(TokenKind::OpenBracket);
                    }
                    ']' => {
                        self.set_state(TokenKind::CloseBracket);
                    }
                    '.' => self.set_state(TokenKind::Dot),
                    _ => {}
                },
                TokenKind::Identifier => match char {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => self.accumulate(char),
                    _ => self.set_state(TokenKind::Temp),
                },
                TokenKind::Comment => match char {
                    '\n' => self.set_state(TokenKind::Temp),
                    _ => self.accumulate(char),
                },
                TokenKind::Lit(l) => match l {
                    Lit::Str => match char {
                        '"' => self.set_state(TokenKind::Temp),
                        _ => self.accumulate(char),
                    },
                    Lit::Float | Lit::Int => match char {
                        c if c.is_whitespace() => self.set_state(TokenKind::Temp),
                        _ => self.accumulate(char),
                    },
                },
                TokenKind::End => {}
                _ => self.set_state(TokenKind::Temp),
            }
        }
        self.set_state(TokenKind::End);
    }
}
