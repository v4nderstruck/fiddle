use std::io::BufRead;

use crate::lexer::{
    tokenizer::TokenizerIterator,
    tokens::{ArithOperation, Token, F64},
};

use super::ast::{ASTNodeTypes, AST};

fn pull_and_compare_content<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    cmp: &[Token],
    err_msg: &str,
) -> anyhow::Result<Token> {
    match iterator.next() {
        Some(t) => {
            if !cmp.contains(&t) {
                iterator.raise_parsing_error(&format!("{} but got {:?}", err_msg, t));
                anyhow::bail!("parsing error")
            } else {
                Ok(t)
            }
        }
        None => {
            iterator.raise_parsing_error(&format!("{} but got EOF", err_msg));
            anyhow::bail!("parsing error")
        }
    }
}

fn pull_and_compare_token<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    cmp: &[Token],
    err_msg: &str,
) -> anyhow::Result<Token> {
    match iterator.next() {
        Some(t) => {
            if !cmp
                .iter()
                .any(|x| core::mem::discriminant(&t) == core::mem::discriminant(x))
            {
                iterator.raise_parsing_error(&format!("{} but got {:?}", err_msg, t));
                anyhow::bail!("parsing error")
            } else {
                Ok(t)
            }
        }
        None => {
            iterator.raise_parsing_error(&format!("{} but got EOF", err_msg));
            anyhow::bail!("parsing error")
        }
    }
}

pub fn ignore_eol<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    match iterator.peek() {
        Some(Token::EOL) => {
            let _ = pull_and_compare_content(iterator, &[Token::EOL], "expected line break")?;
            ignore_eol(iterator, ast, parent)
        }
        Some(_) => Ok(()),
        None => {
            iterator.raise_parsing_error("expected line break");
            anyhow::bail!("parsing error")
        }
    }
}
pub fn program<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
) -> anyhow::Result<()> {
    match iterator.peek() {
        Some(Token::Fun(s)) => {
            if s == String::from("max") || s == String::from("min") {
                let program_node = ast.insert_node(None, None, ASTNodeTypes::Program);
                objective(iterator, ast, program_node)?;
                // check EOL, but dont use it
                let _ = pull_and_compare_content(iterator, &[Token::EOL], "expected line break")?;
                ignore_eol(iterator, ast, program_node)?;
                constraints(iterator, ast, program_node)?;
                Ok(())
            } else {
                iterator.raise_parsing_error("expected objective declaration");
                anyhow::bail!("parsing error")
            }
        }
        Some(Token::EOL) => {
            let program_node = ast.insert_node(None, None, ASTNodeTypes::Program);
            ignore_eol(iterator, ast, program_node)?;
            objective(iterator, ast, program_node)?;
            // check EOL, but dont use it
            let _ = pull_and_compare_content(iterator, &[Token::EOL], "expected line break")?;
            ignore_eol(iterator, ast, program_node)?;
            constraints(iterator, ast, program_node)?;
            Ok(())
        }
        Some(_) | None => {
            iterator.raise_parsing_error("expected objective declaration");
            anyhow::bail!("parsing error")
        }
    }
}

pub fn objective<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    let objective_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Objective);
    ast.insert_node(
        Some(pull_and_compare_content(
            iterator,
            &[
                Token::Fun(String::from("min")),
                Token::Fun(String::from("max")),
            ],
            "expected objective 'min' or 'max'",
        )?),
        Some(objective_node),
        ASTNodeTypes::Token,
    );
    pull_and_compare_content(iterator, &[Token::LParen('{')], "expected '{'")?;
    ignore_eol(iterator, ast, objective_node)?;
    expression(iterator, ast, objective_node)?;
    ignore_eol(iterator, ast, objective_node)?;
    pull_and_compare_content(iterator, &[Token::RParen('}')], "expected '}'")?;
    Ok(())
}

pub fn constraints<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    let constraints_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Constraints);
    pull_and_compare_content(
        iterator,
        &[Token::Fun(String::from("st"))],
        "expected constraints 'st'",
    )?;
    pull_and_compare_content(iterator, &[Token::LParen('{')], "expected '{'")?;
    ignore_eol(iterator, ast, constraints_node)?;
    constraint(iterator, ast, constraints_node)?;
    ignore_eol(iterator, ast, constraints_node)?;
    pull_and_compare_content(iterator, &[Token::RParen('}')], "expected '}'")?;
    Ok(())
}

pub fn constraint<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    match iterator.peek() {
        Some(Token::RParen('}')) => Ok(()),
        Some(Token::Cmp(_))
        | Some(Token::Num(_))
        | Some(Token::Variable(_, _))
        | Some(Token::ArithOp(_))
        | Some(Token::LParen('['))
        | Some(Token::LParen('(')) => {
            let constraints_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Constraint);
            expression(iterator, ast, constraints_node)?;
            ast.insert_node(
                Some(pull_and_compare_token(
                    iterator,
                    &[Token::Cmp(crate::lexer::tokens::CmpOperation::Eq)],
                    "expected comparison operator",
                )?),
                Some(constraints_node),
                ASTNodeTypes::Token,
            );
            ast.insert_node(
                Some(pull_and_compare_token(
                    iterator,
                    &[Token::Num(F64(0.0))],
                    "expected number",
                )?),
                Some(constraints_node),
                ASTNodeTypes::RHS,
            );

            pull_and_compare_content(iterator, &[Token::EOL], "Expected line break")?;
            constraint(iterator, ast, parent)
        }
        Some(_) | None => {
            iterator.raise_parsing_error("expected an constraint");
            anyhow::bail!("parsing error")
        }
    }
}

pub fn expression<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    let expression_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Expression);
    expression_point(iterator, ast, expression_node)?;
    expression_line(iterator, ast, expression_node)?;
    Ok(())
}

pub fn expression_point<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    match iterator.peek() {
        Some(Token::ArithOp(ArithOperation::Add))
        | Some(Token::ArithOp(ArithOperation::Sub))
        | Some(Token::RParen(_))
        | Some(Token::Cmp(_)) => Ok(()),
        Some(Token::Variable(_, _))
        | Some(Token::Num(_))
        | Some(Token::LParen('('))
        | Some(Token::LParen('[')) => {
            let expression_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Expression);
            // FIXME: Here originated error from test case, should revisit theoretic concept
            // simply add coeeficient parser...

            term(iterator, ast, expression_node)?;
            expression_point(iterator, ast, expression_node)
        }
        Some(Token::ArithOp(ArithOperation::Mul)) | Some(Token::ArithOp(ArithOperation::Div)) => {
            let expression_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Expression);
            ast.insert_node(
                Some(pull_and_compare_content(
                    iterator,
                    &[
                        Token::ArithOp(ArithOperation::Mul),
                        Token::ArithOp(ArithOperation::Div),
                    ],
                    "Expected Arithmetic Operation",
                )?),
                Some(expression_node),
                ASTNodeTypes::Token,
            );
            expression_point(iterator, ast, expression_node)
        }
        Some(_) | None => {
            iterator.raise_parsing_error("expected an expression");
            anyhow::bail!("parsing error")
        }
    }
}

pub fn expression_line<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    match iterator.peek() {
        Some(Token::RParen(_)) | Some(Token::Cmp(_)) => Ok(()),
        Some(Token::ArithOp(ArithOperation::Sub)) | Some(Token::ArithOp(ArithOperation::Add)) => {
            let expression_node = ast.insert_node(None, Some(parent), ASTNodeTypes::Expression);
            ast.insert_node(
                Some(pull_and_compare_content(
                    iterator,
                    &[
                        Token::ArithOp(ArithOperation::Add),
                        Token::ArithOp(ArithOperation::Sub),
                    ],
                    "Expected Arithmetic Operation",
                )?),
                Some(expression_node),
                ASTNodeTypes::Token,
            );
            expression_point(iterator, ast, expression_node)?;
            expression_line(iterator, ast, expression_node)
        }
        Some(_) | None => {
            iterator.raise_parsing_error("expected an expression");
            anyhow::bail!("parsing error")
        }
    }
}

pub fn term<R: BufRead>(
    iterator: &mut TokenizerIterator<R>,
    ast: &mut AST,
    parent: usize,
) -> anyhow::Result<()> {
    let term = ast.insert_node(None, Some(parent), ASTNodeTypes::Term);
    match pull_and_compare_token(
        iterator,
        &[
            Token::Variable(String::from("*"), F64(1.0)),
            Token::Num(F64(0.0)),
            Token::LParen('('),
            Token::LParen('['),
        ],
        "expected a term",
    ) {
        Ok(t) => match t {
            Token::Variable(_, _) | Token::Num(_) => {
                ast.insert_node(Some(t), Some(term), ASTNodeTypes::Token);
                Ok(())
            }
            Token::LParen('(') => {
                expression(iterator, ast, term)?;
                pull_and_compare_content(iterator, &[Token::RParen(')')], "expected '{'")?;
                Ok(())
            }
            Token::LParen('[') => {
                expression(iterator, ast, term)?;
                pull_and_compare_content(iterator, &[Token::RParen(']')], "expected '{'")?;
                Ok(())
            }
            _ => {
                iterator.raise_parsing_error("expected a term");
                anyhow::bail!("parsing error")
            }
        },
        Err(e) => Err(e),
    }
}
