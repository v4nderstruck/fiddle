#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub struct F64(pub f64);
impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-6 // precision
    }
}
impl Eq for F64 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmpOperation {
    Eq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Variable(String, F64),   // Variable Name
    ArithOp(ArithOperation), // +, -, *, /
    LParen(char),            // (, [, {
    RParen(char),            // ), ], }
    Fun(String),             // max, min, st
    Num(F64),                // number
    Cmp(CmpOperation),       //
    EOL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexState {
    Match,
    NoMatch,
    Final,
}

#[cfg(test)]
mod test {
    use crate::lexer::tokens::F64;
    #[test]
    fn test_F64_eq() {
        assert!(F64(-1.2089999) == F64(-1.209));
        assert!(F64(-1.2090001) == F64(-1.209));
    }
}
