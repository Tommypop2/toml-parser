pub mod json;
pub mod toml;
use crate::{
    parser::{
        exprs::{Node, Pair, Table},
        AST,
    },
    tokenizer::tokens::Lit,
};
pub struct DefaultCodegen;

pub trait Codegen {
    fn generate_pair(&self, pair: Pair) -> String;
    fn generate_table(&self, table: Table) -> String;
    fn generate_array(&self, arr: Vec<Lit>) -> String {
        let mut str = String::from("[");
        let len = arr.len();
        for i in 0..(len - 1) {
            str += &format!("{},", self.generate_lit(&arr[i]))
        }
        str += &self.generate_lit(&arr[len - 1]);
        str += "]";
        str
    }
    fn generate_lit(&self, lit: &Lit) -> String {
        match lit {
            Lit::StringLit(s) => s.to_string(),
            Lit::Integer(i) => i.to_string(),
            Lit::Float(f) => f.to_string(),
        }
    }
    fn generate_node(&self, node: Node) -> String {
        match node {
            Node::Lit(l) => self.generate_lit(&l),
            Node::Pair(p) => self.generate_pair(p) + "\n",
            Node::Array(a) => self.generate_array(a) + "\n",
            Node::Table(t) => self.generate_table(t) + "\n",
        }
    }
    fn generate(&self, nodes: AST) -> String {
        let mut str = String::from("");
        for node in nodes {
            str += &self.generate_node(node);
        }
        str
    }
}
pub fn generate<T: Codegen>(ast: AST, generator: T) -> String {
    generator.generate(ast)
}
