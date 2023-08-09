use crate::tokenizer::tokens::{Lit, Token, PositionedToken};

pub fn get_lits<'a>(contents: &Vec<PositionedToken>) -> Vec<Lit> {
    let mut lits: Vec<Lit> = vec![];
    for token in contents {
        // Don't bother handling incorrect syntax
        if let Token::Lit(lit) = &token.token {
            lits.push(lit.clone())
        }
    }
    lits
}
