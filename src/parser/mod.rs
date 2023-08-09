pub mod exprs;

use crate::tokenizer::{tokens::Token, Tokens};

use self::exprs::{Node, Pair};
pub type AST = Vec<Node>;
// Build up our AST
pub fn parse(tokens: Tokens) -> Result<AST, ()> {
    let mut nodes: AST = vec![];
    let mut i = 0;
    while i < tokens.len() - 1 {
        let current = &tokens[i];
        if let Token::Ident(ident) = &current.token {
            // Need to consume if next item is equals operator, and the item after that is a value, otherwise error
            if let Token::Punctuation(op) = &tokens[i + 1].token {
                if *op == '=' {
                    if let Token::Lit(lit) = &tokens[i + 2].token {
                        nodes.push(Node::Pair(Pair::new(ident.to_string(), lit.clone())));
                        // Need to skip these 2 tokens next time since we've already captured them
                        i += 2;
                    }
                }
            }
        }
        i += 1;
    }
    Ok(nodes)
}
