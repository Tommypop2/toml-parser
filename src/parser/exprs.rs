use crate::tokenizer::tokens::Lit;

#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub value: Box<Node>,
}
impl Pair {
    pub fn new(key: String, value: Node) -> Self {
        Self {
            key,
            value: Box::new(value),
        }
    }
}
#[derive(Debug)]
pub struct Table {
    pub ident: String,
    pub pairs: Vec<Pair>,
}
impl Table {
    pub fn new(ident: String, pairs: Vec<Pair>) -> Self {
        Self { ident, pairs }
    }
}
#[derive(Debug)]
pub enum Node {
    Pair(Pair),
    Array(Vec<Lit>),
    Lit(Lit),
    Table(Table),
}
