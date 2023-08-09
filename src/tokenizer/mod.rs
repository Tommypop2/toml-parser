use self::{
    capturer::{capture_until, Capture},
    tokens::{Lit, PositionedToken, Token},
};

pub mod capturer;
pub mod tokens;
const SINGLE_CHAR_TOKENS: [char; 5] = ['=', '[', ']', '.', ','];
pub type Tokens = Vec<PositionedToken>;

pub fn tokenize(toml: &str) -> Result<Tokens, ()> {
    let bytes: &[u8] = toml.as_bytes();
    let mut i = 0;
    let mut tokens: Tokens = vec![];
    let len = bytes.len();
    while i < len {
        let char = bytes[i] as char;
        // Start of string
        if char == '"' {
            let start_i = i;
            let captured = capture_until(bytes, &mut i, Capture::Char('"'));
            tokens.push(PositionedToken::new(
                start_i,
                i,
                Token::Lit(Lit::StringLit(captured)),
            ))
        }
        if SINGLE_CHAR_TOKENS.contains(&char) {
            tokens.push(PositionedToken::new(i, i, Token::Punctuation(char)))
        }
        if char.is_numeric() {
            let start_i = i;
            // Capture until whitespace or single char token
            let mut chars = String::from("");
            let mut floating_point = false;
            loop {
                let inside_char = bytes[i] as char;
                if inside_char == '.' {
                    floating_point = true;
                } else if inside_char.is_whitespace() || SINGLE_CHAR_TOKENS.contains(&inside_char) {
                    i -= 1;
                    break;
                }
                chars.push(inside_char);
                i += 1;
            }
            let token = if floating_point {
                Token::Lit(Lit::Float(chars.parse().unwrap()))
            } else {
                Token::Lit(Lit::Integer(chars.parse().unwrap()))
            };
            tokens.push(PositionedToken::new(start_i, i, token))
        }
        // Identifier
        if char.is_alphabetic() {
            let start_i = i;
            // Capture until whitespace or single char token
            let mut ident = String::from("");
            loop {
                let inside_char = bytes[i] as char;
                if inside_char.is_whitespace() || SINGLE_CHAR_TOKENS.contains(&inside_char) {
                    i -= 1;
                    break;
                }
                ident.push(inside_char);
                i += 1;
            }
            tokens.push(PositionedToken::new(start_i, i, Token::Ident(ident)))
        }
        if char == '#' {
            let start_i = i;
            // Capture rest of line as a comment
            let comment = capture_until(bytes, &mut i, Capture::Char('\n'));
            tokens.push(PositionedToken::new(start_i, i, Token::Comment(comment)))
        }
        if char.is_whitespace() {
            tokens.push(PositionedToken::new(i, i, Token::Whitespace(char)))
        }
        i += 1;
    }
    Ok(tokens)
}
