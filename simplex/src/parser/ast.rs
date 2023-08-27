use std::io::BufRead;

use crate::lexer::{tokenizer::Tokenizer, tokens::Token};

use super::syntax::program;

#[derive(Debug, PartialEq, Eq)]
pub enum ASTNodeTypes {
    Token,
    RHS,
    Program,
    Objective,
    Constraints,
    Constraint,
    Expression,
    Term,
}

#[derive(Debug)]
pub struct ASTNode {
    pub node_type: ASTNodeTypes,
    pub parent: Option<usize>,
    pub children: Option<Vec<usize>>,
    pub data_index: Option<usize>,
}

impl ASTNode {
    pub fn new(
        node_type: ASTNodeTypes,
        parent: Option<usize>,
        children: Option<Vec<usize>>,
        data_index: Option<usize>,
    ) -> Self {
        Self {
            node_type,
            parent,
            children,
            data_index,
        }
    }
}

#[derive(Debug)]
pub struct AST {
    pub data: Vec<Token>,
    pub nodes: Vec<ASTNode>,
}

impl AST {
    /// Insert a node described by data_index and parent into the AST and updates parent node's children list
    /// Returns index of the node inserted
    pub fn insert_node(
        &mut self,
        token: Option<Token>,
        parent: Option<usize>,
        node_type: ASTNodeTypes,
    ) -> usize {
        let mut data_index = None;
        if let Some(t) = token {
            self.data.push(t);
            data_index = Some(self.data.len() - 1);
        }
        let node = ASTNode::new(node_type, parent, None, data_index);
        self.nodes.push(node);
        let id = self.nodes.len() - 1;
        if let Some(pidx) = parent {
            let children_of_parent = self.nodes[pidx].children.as_mut();
            if let Some(v) = children_of_parent {
                v.push(id);
            } else {
                self.nodes[pidx].children = Some(vec![id]);
            }
        }
        self.nodes.len() - 1
    }

    pub fn find_root(&self, ntype: ASTNodeTypes) -> Option<usize> {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node.node_type == ntype {
                return Some(idx);
            }
        }
        None
    }

    pub fn print_subtree(&self, root: usize, level: usize) {
        let node = &self.nodes[root];
        let indent = "-".repeat(level * 2);
        if let Some(token_idx) = node.data_index {
            println!("{} {:?}", indent, &self.data[token_idx]);
        }

        if let Some(children) = &node.children {
            for child in children {
                self.print_subtree(*child, level + 1);
            }
        }
    }

    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nodes: Vec::new(),
        }
    }
}

pub fn construct_ast<R: BufRead>(reader: R) -> anyhow::Result<AST> {
    let mut iterator = Tokenizer::new(reader).into_iter();
    let mut ast = AST::new();
    program(&mut iterator, &mut ast)?;
    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::construct_ast;

    #[test]
    fn test_ast() {
        {
            let input = "
max {x1 - x2 }
st {
    -1.21x1 / -a >= 1000
}
";
            let ast = construct_ast(input.as_bytes());
            // println!("{:#?}", ast);
            assert!(ast.is_ok());
        }
    }

    // TODO: Look at this case fix it
    #[test]
    fn test_ast_accidentally_accepted() {
        {
            let input = "
max {x1 - - x2 }
st {
    -1.21+/x1 / -a >= 1000
}
";
            let ast = construct_ast(input.as_bytes());

            println!("{:?}", ast.as_ref().unwrap().nodes);
            assert!(ast.is_ok());
        }
    }
}
