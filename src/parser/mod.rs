pub mod arrays;
pub mod exprs;

use crate::tokenizer::{tokens::Token, Tokens};

use self::{
    arrays::get_lits,
    exprs::{Node, Pair},
};
fn capture_until<'a>(tokens: &'a Tokens, i: &mut usize, stop: Token) -> Vec<&'a Token> {
    let mut captured: Vec<&Token> = vec![];
    loop {
        *i += 1;
        let current = &tokens[*i];
        if current.token == stop {
            break;
        }
        captured.push(&current.token)
    }
    captured
}
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
                        nodes.push(Node::Pair(Pair::new(ident.to_string(), Node::Lit(lit.clone()))));
                        // Need to skip these 2 tokens next time since we've already captured them
                        i += 2;
                    } else if let Token::Punctuation(p) = &tokens[i + 2].token {
                        if *p == '[' {
                            // Dealing with an array
                            // Capture until close
                            i += 2;
                            let contents = capture_until(&tokens, &mut i, Token::Punctuation(']'));
                            let lits = get_lits(&contents);
                            nodes.push(Node::Pair(Pair::new(ident.to_string(), Node::Array(lits))));
                            i += contents.len();
                        }
                    }
                }
            }
        }
        i += 1;
    }
    Ok(nodes)
}
