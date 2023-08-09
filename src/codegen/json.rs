use crate::parser::{exprs::Table, AST};

use super::Codegen;

pub struct JsonGenerator;
impl Codegen for JsonGenerator {
    fn generate_pair(&self, pair: crate::parser::exprs::Pair) -> String {
        format!("\"{}\":\"{}\"", pair.key, self.generate_node(*pair.value))
    }
    fn generate_table(&self, table: Table) -> String {
        format!("\"{}\":{}", table.ident, self.generate(table.children))
    }
    fn generate(&self, nodes: AST) -> String {
        let mut str = String::from("");
        for node in nodes {
            str += &self.generate_node(node);
        }
        format!("{{{str}}}")
    }
}
