pub mod ast;
pub mod syntax;

pub struct Objective {}
pub struct Constraint {}

pub struct Simplex {
    objective: Objective,
    constraints: Vec<Constraint>,
}
