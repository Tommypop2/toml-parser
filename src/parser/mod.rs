pub mod arrays;
pub mod exprs;

use crate::tokenizer::{
    tokens::{PositionedToken, Token},
    Tokens,
};

use self::{
    arrays::get_lits,
    exprs::{Node, Pair, Table},
};
fn capture_until(tokens: &Tokens, i: &mut usize, stop: Token) -> Vec<PositionedToken> {
    let mut captured: Vec<PositionedToken> = vec![];
    loop {
        *i += 1;
        if *i >= tokens.len() {
            break;
        }
        let current = &tokens[*i];
        if current.token == stop {
            break;
        }
        captured.push(current.clone())
    }
    captured
}
pub type AST = Vec<Node>;

// Build up our AST
pub fn parse(tokens: Vec<PositionedToken>) -> Result<AST, ()> {
    let mut nodes: AST = vec![];
    let mut i = 0;
    if tokens.len() == 0 {
        return Err(());
    }
    while i < tokens.len() - 1 {
        let current = &tokens[i];
        // Pairs
        if let Token::Ident(ident) = &current.token {
            if i + 2 >= tokens.len() {
                i += 1;
                continue;
            }
            // Need to consume if next item is equals operator, and the item after that is a value, otherwise error
            if let Token::Punctuation(op) = &tokens[i + 1].token {
                if *op == '=' {
                    if let Token::Lit(lit) = &tokens[i + 2].token {
                        nodes.push(Node::Pair(Pair::new(
                            ident.to_string(),
                            Node::Lit(lit.clone()),
                        )));
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
                        }
                    }
                }
            }
        }
        // Tables
        if let Token::Punctuation(p) = &current.token {
            if *p == '[' {
                // Next should be identifiers
                let mut identifiers: Vec<String> = vec![];
                for ident in capture_until(&tokens, &mut i, Token::Punctuation(']')) {
                    if let Token::Ident(ident) = &ident.token {
                        identifiers.push(ident.clone())
                    } else if let Token::Punctuation(p) = &ident.token {
                        if *p != '.' {
                            return Err(());
                        }
                    } else {
                        return Err(());
                    }
                }
                // Capture until '[' followed by an ident, or EOF
                let mut captured: Vec<PositionedToken> = vec![];
                loop {
                    i += 1;
                    if i >= tokens.len() - 1 {
                        break;
                    }
                    let current = &tokens[i];
                    let next = &tokens[i + 1];
                    if current.token == Token::Punctuation('[') {
                        if let Token::Ident(_) = next.token {
                            break;
                        }
                    }
                    captured.push(current.clone())
                }
                i -= 1;
                let len = identifiers.len();
                let nds = parse(captured).unwrap_or(vec![]);
                let mut table = Node::Table(Table::new(identifiers[len - 1].clone(), nds));
                for i in 1..len {
                    let ind = len - i - 1;
                    let ident = &identifiers[ind];
                    table = Node::Table(Table::new(ident.clone(), vec![table]))
                }
                nodes.push(table);
            }
        }
        i += 1;
    }
    Ok(nodes)
}
