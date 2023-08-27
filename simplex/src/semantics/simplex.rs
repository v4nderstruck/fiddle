use std::collections::HashMap;

use ndarray::{Array, Ix1, Ix2};

use crate::{
    parser::ast::{ASTNodeTypes, AST},
    semantics::symbols::{Symbol, SymbolTable},
};

pub struct Simplex {
    tableau: Array<f64, Ix2>,
    symbolCol: HashMap<String, usize>,
}

impl Simplex {
    fn from(value: AST) -> anyhow::Result<Self> {
        let symbols = SymbolTable::from(value);
        let n_symbols = symbols.table.len();
        let mut symbolCol = HashMap::new();
        let mut tableau = Array::zeros((
            symbols.n_constr as usize + 1,
            n_symbols + symbols.n_constr as usize - 1 + 2,
        ));

        let mut free_col = 1;
        let max_col = symbols.n_constr as usize + n_symbols;
        for (key, value_arr) in symbols.table.iter() {
            let mut insert_col = free_col;

            if key == "RHS" {
                insert_col = max_col;
            } else if symbolCol.contains_key(key) {
                insert_col = symbolCol[key];
            } else {
                symbolCol.insert(key.clone(), insert_col);
                free_col += 1;
            }
            for value in value_arr {
                match value {
                    Symbol::Obj(value) => {
                        tableau[[0, insert_col]] = value.0;
                    }
                    Symbol::Constr(row, value) => {
                        tableau[[*row as usize + 1, insert_col]] = value.0;
                    }
                    Symbol::RHS(row, value) => {
                        tableau[[*row as usize + 1, insert_col]] = value.0;
                    }
                }
            }
        }
        for i in free_col..max_col {
            symbolCol.insert(format!("sub_v_{}", i), i);
        }

        println!("{:?}, {:?}", tableau, symbolCol);

        Ok(Self { tableau, symbolCol })
    }
}

#[cfg(test)]
mod test {
    use crate::{parser::ast::construct_ast, semantics::simplex::Simplex};

    #[test]
    fn test_simplex() {
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
            let simplex = Simplex::from(ast).unwrap();
            panic!()
        }
    }
}
