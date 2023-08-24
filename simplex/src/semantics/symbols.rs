use std::collections::{hash_map::Entry, HashMap};

use crate::{
    lexer::tokens::{ArithOperation, Token, F64},
    parser::ast::{ASTNodeTypes, AST},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Obj(F64),         // coefficient
    Constr(u32, F64), // (row, coefficient)
    RHS(u32, F64),    // row, value
}

/// Variables and their coeefficients
#[derive(Debug)]
pub struct SymbolTable {
    pub table: HashMap<String, Vec<Symbol>>,
}

impl SymbolTable {
    fn collect_objective_symbols(&mut self, ast: &AST, root: usize, prev_sign: i8) -> i8 {
        let node = &ast.nodes[root];
        let mut sign = prev_sign;
        if let Some(data_index) = node.data_index {
            let token = &ast.data[data_index];
            match token {
                Token::Variable(name, coef) => {
                    let corrected_sign = prev_sign as f64 * coef.0;
                    match self.table.entry(name.clone()) {
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(Symbol::Obj(F64(corrected_sign)));
                        }
                        Entry::Vacant(e) => {
                            e.insert(vec![Symbol::Obj(F64(corrected_sign))]);
                        }
                    }
                }
                Token::ArithOp(op) => match op {
                    ArithOperation::Add => {
                        sign = 1;
                    }
                    ArithOperation::Sub => {
                        sign = -1;
                    }
                    _ => {}
                },
                _ => {}
            }
            return sign;
        }
        let mut new_sign = sign;
        if let Some(children) = &node.children {
            for child in children {
                new_sign = self.collect_objective_symbols(ast, *child, new_sign)
            }
        }
        new_sign
    }
    fn collect_constraints_symbols(&mut self, ast: &AST, root: usize) {
        let node = &ast.nodes[root];
        if let Some(children) = &node.children {
            for (i, child) in children.iter().enumerate() {
                let child_node = &ast.nodes[*child];
                match child_node.node_type {
                    ASTNodeTypes::Constraint => {
                        self.collect_constraint_symbol(ast, *child, i as u32, 1);
                    }
                    _ => {}
                }
            }
        }
    }
    fn collect_constraint_symbol(&mut self, ast: &AST, root: usize, row: u32, prev_sign: i8) -> i8 {
        let node = &ast.nodes[root];

        let mut sign = prev_sign;
        if let Some(data_index) = node.data_index {
            let token = &ast.data[data_index];
            match token {
                Token::Variable(name, coef) => {
                    let corrected_coef = coef.0 * prev_sign as f64;

                    match self.table.entry(name.clone()) {
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(Symbol::Constr(row, F64(corrected_coef)));
                        }
                        Entry::Vacant(e) => {
                            e.insert(vec![Symbol::Constr(row, F64(corrected_coef))]);
                        }
                    }
                }
                Token::Num(n) => {
                    if node.node_type == ASTNodeTypes::RHS {
                        match self.table.entry("RHS".to_string()) {
                            Entry::Occupied(mut e) => {
                                e.get_mut().push(Symbol::RHS(row, n.clone()));
                            }
                            Entry::Vacant(e) => {
                                e.insert(vec![Symbol::RHS(row, n.clone())]);
                            }
                        }
                    }
                }
                Token::ArithOp(op) => match op {
                    ArithOperation::Add => sign = 1,
                    ArithOperation::Sub => sign = -1,
                    _ => {}
                },
                _ => {}
            }
        }

        let mut new_sign = sign;
        if let Some(children) = &node.children {
            for child in children {
                new_sign = self.collect_constraint_symbol(ast, *child, row, new_sign)
            }
        }
        new_sign
    }
}

impl From<AST> for SymbolTable {
    fn from(ast: AST) -> Self {
        let mut table = Self {
            table: HashMap::new(),
        };

        if let Some(obj_root) = ast.find_root(ASTNodeTypes::Objective) {
            table.collect_objective_symbols(&ast, obj_root, 1);
        }
        if let Some(obj_root) = ast.find_root(ASTNodeTypes::Constraints) {
            table.collect_constraints_symbols(&ast, obj_root);
        }
        table
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::tokens::F64,
        parser::ast::construct_ast,
        semantics::symbols::{Symbol, SymbolTable},
    };

    #[test]
    fn test_symbol_table() {
        {
            let input = "
max {x1 - x2 }
st {
    -1.21x1 >= 1000
    x2 >= 1000
}
";
            let ast = construct_ast(input.as_bytes());
            assert!(ast.is_ok());
            let ast = ast.unwrap();
            let t = SymbolTable::from(ast);
            assert_eq!(t.table.len(), 3);
            assert_eq!(
                *t.table.get("x1").unwrap(),
                vec![Symbol::Obj(F64(1.0)), Symbol::Constr(0, F64(-1.21))]
            );
            assert_eq!(
                *t.table.get("x2").unwrap(),
                vec![Symbol::Obj(F64(-1.0)), Symbol::Constr(1, F64(1.0))]
            );
            assert_eq!(
                *t.table.get("RHS").unwrap(),
                vec![Symbol::RHS(0, F64(1000.0)), Symbol::RHS(1, F64(1000.0))]
            );
        }
    }

    #[test]
    fn test_symbol_table_2() {
        {
            let input = "
max {1.2x1 - 3x2 + 202.1 / 2 + x3}
st {
    -1.21x1 - x2>= 1000
    x2 + -0.2x3 - y2 >= 100
    x1 - -120.1y3 >= -1.1 
}
";
            let ast = construct_ast(input.as_bytes());
            assert!(ast.is_ok());
            let ast = ast.unwrap();
            let t = SymbolTable::from(ast);
            assert_eq!(
                *t.table.get("x1").unwrap(),
                vec![
                    Symbol::Obj(F64(1.2)),
                    Symbol::Constr(0, F64(-1.21)),
                    Symbol::Constr(2, F64(1.0))
                ]
            );
            assert_eq!(
                *t.table.get("x2").unwrap(),
                vec![
                    Symbol::Obj(F64(-3.0)),
                    Symbol::Constr(0, F64(-1.0)),
                    Symbol::Constr(1, F64(1.0))
                ]
            );
            assert_eq!(
                *t.table.get("x3").unwrap(),
                vec![Symbol::Obj(F64(1.0)), Symbol::Constr(1, F64(-0.2)),]
            );
            assert_eq!(
                *t.table.get("y2").unwrap(),
                vec![Symbol::Constr(1, F64(-1.0)),]
            );
            assert_eq!(
                *t.table.get("y3").unwrap(),
                vec![Symbol::Constr(2, F64(120.1)),]
            );
            assert_eq!(
                *t.table.get("RHS").unwrap(),
                vec![
                    Symbol::RHS(0, F64(1000.0)),
                    Symbol::RHS(1, F64(100.0)),
                    Symbol::RHS(2, F64(-1.1))
                ]
            );
        }
    }
}
