use crate::{
    parser::{
        exprs::{Node, Pair},
        AST,
    },
    tokenizer::tokens::Lit,
};
fn generate_lit(lit: &Lit) -> String {
    match lit {
        Lit::StringLit(s) => s.to_string(),
        Lit::Integer(i) => i.to_string(),
        Lit::Float(f) => f.to_string(),
    }
}
fn generate_pair(pair: Pair) -> String {
    format!("{} = {}", pair.key, generate_node(*pair.value))
}
fn generate_array(arr: Vec<Lit>) -> String {
    let mut str = String::from("[");
    let len = arr.len();
    for i in 0..(len - 1) {
        str += &format!("{},", generate_lit(&arr[i]))
    }
    str += &generate_lit(&arr[len - 1]);
    str += "]";
    str
}
pub fn generate_node(node: Node) -> String {
    match node {
        Node::Lit(l) => generate_lit(&l),
        Node::Pair(p) => generate_pair(p),
        Node::Array(a) => generate_array(a),
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
