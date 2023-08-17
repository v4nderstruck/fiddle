use std::collections::{hash_map::Entry, HashMap};

use crate::{
    lexer::tokens::Token,
    parser::ast::{ASTNodeTypes, AST},
};

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

impl SymbolTable {
    fn collect_symbols(&mut self, ast: &AST, root: usize) {
        todo!()
    }
}

impl From<AST> for SymbolTable {
    fn from(ast: AST) -> Self {
        let mut table = Self {
            table: HashMap::new(),
        };

        if let Some(obj_root) = ast.find_root(ASTNodeTypes::Objective) {
            table.collect_symbols(&ast, obj_root);
        }
        if let Some(obj_root) = ast.find_root(ASTNodeTypes::Constraints) {
            table.collect_symbols(&ast, obj_root);
        }
        table
    }
}
