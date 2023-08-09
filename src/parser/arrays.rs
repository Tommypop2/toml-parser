use crate::tokenizer::tokens::{Lit, Token};

pub fn get_lits<'a>(contents: &'a Vec<&'a Token>) -> Vec<Lit> {
    let mut lits: Vec<Lit> = vec![];
    for token in contents {
        // Don't bother handling incorrect syntax
        if let Token::Lit(lit) = token {
            lits.push(lit.clone())
        }
    }
    lits
}
