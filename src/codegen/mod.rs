use crate::{
    parser::{
        exprs::{Node, Pair},
        AST,
    },
    tokenizer::tokens::Lit,
};
fn generate_lit(lit: Lit) -> String {
    match lit {
        Lit::StringLit(s) => s,
        Lit::Integer(i) => i.to_string(),
        Lit::Float(f) => f.to_string(),
    }
}
fn generate_pair(pair: Pair) -> String {
    format!("{} = {}", pair.key, generate_node(*pair.value))
}
pub fn generate_node(node: Node) -> String {
    match node {
        Node::Lit(l) => generate_lit(l),
        Node::Pair(p) => generate_pair(p),
        _ => "".into(),
    }
}
pub fn generate(nodes: AST) -> String {
    let mut str = String::from("");
    for node in nodes {
        str += &(generate_node(node) + "\n")
    }
    str
}
