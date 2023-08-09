use crate::parser::exprs::{Pair, Table};

use super::Codegen;
pub struct TomlGenerator;
impl Codegen for TomlGenerator {
    fn generate_pair(&self, pair: Pair) -> String {
        format!("{} = {}", pair.key, self.generate_node(*pair.value))
    }
    fn generate_table(&self, table: Table) -> String {
        let header = format!("[{}]", table.ident);
        let mut contents = String::from("");
        for child in table.children {
            contents += &self.generate_node(child);
        }
        header + &contents
    }
}
