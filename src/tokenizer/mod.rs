use self::tokens::{PositionedToken, Token};

pub mod tokens;
const SINGLE_CHAR_TOKENS: [char; 4] = ['=', '[', ']', '.'];
pub fn tokenize(toml: &str) -> Result<Vec<PositionedToken>, ()> {
    let bytes = toml.as_bytes();
    let mut i = 0;
    let mut tokens: Vec<PositionedToken> = vec![];
    let len = bytes.len();
    while i < len {
        let char = bytes[i] as char;
        // Start of string
        if char == '"' {
            let start_i = i;
            let mut str = String::from("");
            // Capture all until next '"'
            loop {
                i += 1;
                let inside_char = bytes[i] as char;
                if inside_char == '"' {
                    break;
                }
                str.push(inside_char);
            }
            tokens.push(PositionedToken::new(start_i, i, Token::StringLit(str)))
        }
        // Identifier
        if char.is_alphabetic() {
            let start_i = i;
            // Capture until whitespace or single char token
            let mut ident = String::from("");
            loop {
                let inside_char = bytes[i] as char;
                if inside_char.is_whitespace() || SINGLE_CHAR_TOKENS.contains(&inside_char) {
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
            let mut comment = String::from("");
            loop {
                i += 1;
                let inside_char = bytes[i] as char;
                if inside_char == '\n' {
                    break;
                }
                comment.push(inside_char);
            }
            tokens.push(PositionedToken::new(start_i, i, Token::Comment(comment)))
        }
        if SINGLE_CHAR_TOKENS.contains(&char) {
            tokens.push(PositionedToken::new(i, i, Token::Punctuation(char)))
        }
        i += 1;
    }
    Ok(tokens)
}
