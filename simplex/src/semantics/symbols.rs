use std::collections::HashMap;

use crate::parser::ast::AST;

#[derive(Debug)]
pub enum Symbol {
    Obj(f64),         // coefficient
    Constr(u32, f64), // (row, coefficient)
}

/// Variables and their coeefficients
#[derive(Debug)]
pub struct SymbolTable {
    pub table: HashMap<String, Vec<Symbol>>,
}

impl From<AST> for SymbolTable {
    fn from(value: AST) -> Self {
        todo!()
    }
}
