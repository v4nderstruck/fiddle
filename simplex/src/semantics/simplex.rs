use ndarray::{Array, Ix1, Ix2};

use crate::parser::ast::{ASTNode, ASTNodeTypes, AST};

pub struct Simplex {
    objective: Array<f64, Ix1>,   // c^T = (c_1, ..., c_n)
    constraints: Array<f64, Ix2>, // A = p x n
    bounds: Array<f64, Ix1>,      // b = (b_1, ..., b_p)
}

impl Simplex {
    fn normalize_objective(ast: &AST, obj_root: usize) -> anyhow::Result<Array<f64, Ix1>> {
        ast.print_subtree(obj_root, 0);
        todo!()
    }

    fn from(value: AST) -> anyhow::Result<Self> {
        let objective;
        if let Some(obj_root) = value.find_root(ASTNodeTypes::Objective) {
            objective = Self::normalize_objective(&value, obj_root)?;
            todo!()
        } else {
            anyhow::bail!("Objective not found")
        }
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
        }
    }
}
