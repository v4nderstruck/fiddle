use super::{
    tokens::{ArithOperation, CmpOperation, LexState, Token, F64},
    Tokenable,
};

// TODO: Add support for comments

#[derive(Debug)]
pub struct TAVariable {
    coef: Option<TANum>,
    sign: Option<i8>,
    name: Option<String>,
    dead: bool,
}

#[derive(Debug)]
pub struct TAArithOp {
    op: Option<ArithOperation>,
    dead: bool,
}

#[derive(Debug)]
pub struct TALParan {
    p: Option<char>,
    dead: bool,
}

#[derive(Debug)]
pub struct TARParan {
    p: Option<char>,
    dead: bool,
}

#[derive(Debug)]
pub struct TANum {
    num: Option<f64>,
    int_part: bool,
    pow_dec: i32,
    signess: i8,
    dead: bool,
}

#[derive(Debug)]
pub struct TACmp {
    cmp: Option<CmpOperation>,
    dead: bool,
}

#[derive(Debug)]
pub struct TAFun {
    fun: Option<String>,
    last_char: Option<char>,
    dead: bool,
}

impl TAFun {
    pub fn new() -> Self {
        Self {
            fun: None,
            last_char: None,
            dead: false,
        }
    }
}

impl Tokenable for TAFun {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead {
            return LexState::NoMatch;
        }
        if self.last_char.is_none() && (c == 'm' || c == 's') {
            self.last_char = Some(c);
            return LexState::Match;
        }
        if self.last_char == Some('s') && c == 't' {
            self.last_char = Some(c);
            self.fun = Some(String::from("st"));
            return LexState::Final;
        }
        if self.last_char == Some('m') && (c == 'a' || c == 'i') {
            self.last_char = Some(c);
            return LexState::Match;
        }
        if self.last_char == Some('a') && c == 'x' {
            self.fun = Some(String::from("max"));
            return LexState::Final;
        }
        if self.last_char == Some('i') && c == 'n' {
            self.fun = Some(String::from("min"));
            return LexState::Final;
        }

        self.dead = true;
        LexState::NoMatch
    }
    fn reset(&mut self) {
        self.fun = None;
        self.dead = false;
        self.last_char = None;
    }
    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }

        self.fun.as_ref().map(|s| Token::Fun(s.clone()))
    }
}

impl TACmp {
    pub fn new() -> Self {
        Self {
            cmp: None,
            dead: false,
        }
    }
}

impl Tokenable for TACmp {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead {
            return LexState::NoMatch;
        }
        match c {
            '=' => {
                if let Some(prev) = &self.cmp {
                    match prev {
                        CmpOperation::Lt => {
                            self.cmp = Some(CmpOperation::Leq);
                            LexState::Final
                        }
                        CmpOperation::Gt => {
                            self.cmp = Some(CmpOperation::Geq);
                            LexState::Final
                        }
                        _ => {
                            self.dead = true;
                            LexState::NoMatch
                        }
                    }
                } else if self.cmp.is_none() {
                    self.cmp = Some(CmpOperation::Eq);
                    return LexState::Final;
                } else {
                    self.dead = true;
                    return LexState::NoMatch;
                }
            }
            '<' => {
                self.cmp = Some(CmpOperation::Lt);
                LexState::Final
            }
            '>' => {
                self.cmp = Some(CmpOperation::Gt);
                LexState::Final
            }
            _ => {
                self.dead = true;
                LexState::NoMatch
            }
        }
    }

    fn reset(&mut self) {
        self.dead = false;
        self.cmp = None;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(cmp) = &self.cmp {
            return Some(Token::Cmp(cmp.clone()));
        }
        None
    }
}

impl TANum {
    pub fn new() -> Self {
        Self {
            num: None,
            int_part: true,
            pow_dec: 1,
            signess: 1,
            dead: false,
        }
    }
}

impl Tokenable for TANum {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead {
            return LexState::NoMatch;
        }
        match c {
            '0'..='9' => {
                if self.int_part {
                    if self.num.is_none() {
                        self.num = Some(0.0);
                    }
                    self.num = Some(self.num.unwrap() * 10.0 + c.to_digit(10).unwrap() as f64);
                } else {
                    self.num = Some(
                        self.num.unwrap()
                            + c.to_digit(10).unwrap() as f64 / 10.0f64.powi(self.pow_dec),
                    );
                    self.pow_dec += 1
                }
                LexState::Final
            }
            '.' => {
                if self.int_part && self.num.is_some() {
                    self.int_part = false;
                    LexState::Final
                } else {
                    self.dead = true;
                    LexState::NoMatch
                }
            }
            '-' => {
                if self.num.is_none() && self.signess == 1 {
                    self.signess = -1;
                    return LexState::Match;
                }
                self.dead = true;
                LexState::NoMatch
            }
            _ => {
                self.dead = true;
                LexState::NoMatch
            }
        }
    }

    fn reset(&mut self) {
        self.dead = false;
        self.signess = 1;
        self.pow_dec = 1;
        self.num = None;
        self.int_part = true;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(n) = self.num {
            return Some(Token::Num(F64(n * self.signess as f64)));
        }
        None
    }
}

impl TAVariable {
    pub fn new() -> Self {
        Self {
            coef: None,
            sign: None,
            name: None,
            dead: false,
        }
    }
}

impl TAArithOp {
    pub fn new() -> Self {
        Self {
            op: None,
            dead: false,
        }
    }
}

impl TALParan {
    pub fn new() -> Self {
        Self {
            p: None,
            dead: false,
        }
    }
}

impl TARParan {
    pub fn new() -> Self {
        Self {
            p: None,
            dead: false,
        }
    }
}

impl Tokenable for TAVariable {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead {
            return LexState::NoMatch;
        }
        if c.is_alphabetic() || c == '_' {
            if self.name.is_none() {
                self.name = Some(String::new());
            }
            self.name.as_mut().unwrap().push(c);
            return LexState::Final;
        }
        if c == '-' {
            self.sign = Some(-1);
        }
        if (c.is_numeric() || c == '.' || c == '-') && self.name.is_none() {
            if self.coef.is_none() {
                self.coef = Some(TANum::new());
            }

            match self.coef.as_mut().unwrap().consume_char(c) {
                LexState::NoMatch => return LexState::NoMatch,
                _ => return LexState::Match,
            }
        }
        if c.is_numeric() && self.name.is_some() {
            self.name.as_mut().unwrap().push(c);
            return LexState::Final;
        }

        LexState::NoMatch
    }
    fn reset(&mut self) {
        self.dead = false;
        self.name = None;
        self.coef = None;
        self.sign = None;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(v) = &self.name {
            if let Some(num) = &self.coef {
                let possible_sign = self.sign.unwrap_or(1);
                let token = num
                    .tokenize()
                    .unwrap_or(Token::Num(F64(1.0 * possible_sign as f64)));
                if let Token::Num(coef) = token {
                    return Some(Token::Variable(v.clone(), coef));
                }
            }
            return Some(Token::Variable(v.clone(), F64(1.0)));
        }
        None
    }
}

impl Tokenable for TAArithOp {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead || self.op.is_some() {
            self.dead = true;
            return LexState::NoMatch;
        }
        match c {
            '+' => {
                self.op = Some(ArithOperation::Add);
                LexState::Final
            }
            '/' => {
                self.op = Some(ArithOperation::Div);
                LexState::Final
            }
            '*' => {
                self.op = Some(ArithOperation::Mul);
                LexState::Final
            }
            '-' => {
                self.op = Some(ArithOperation::Sub);
                LexState::Final
            }
            _ => {
                self.dead = true;
                self.op = None;
                LexState::NoMatch
            }
        }
    }

    fn reset(&mut self) {
        self.dead = false;
        self.op = None;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(n) = self.op {
            return Some(Token::ArithOp(n));
        }
        None
    }
}

impl Tokenable for TALParan {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead || self.p.is_some() {
            self.dead = true;
            return LexState::NoMatch;
        }
        if c == '(' || c == '[' || c == '{' {
            self.p = Some(c);
            LexState::Final
        } else {
            self.dead = true;
            LexState::NoMatch
        }
    }

    fn reset(&mut self) {
        self.dead = false;
        self.p = None;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(p) = self.p {
            return Some(Token::LParen(p));
        }
        None
    }
}

impl Tokenable for TARParan {
    fn consume_char(&mut self, c: char) -> LexState {
        if self.dead || self.p.is_some() {
            self.dead = true;
            return LexState::NoMatch;
        }
        if c == ')' || c == ']' || c == '}' {
            self.p = Some(c);
            LexState::Final
        } else {
            self.dead = true;
            LexState::NoMatch
        }
    }

    fn reset(&mut self) {
        self.dead = false;
        self.p = None;
    }

    fn tokenize(&self) -> Option<super::tokens::Token> {
        if self.dead {
            return None;
        }
        if let Some(p) = self.p {
            return Some(Token::RParen(p));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{
        token_automata::{TAArithOp, TACmp, TAFun, TALParan, TANum, TARParan, TAVariable},
        tokens::{ArithOperation, CmpOperation, Token, F64},
        Tokenable,
    };

    fn tokenize(automata: &mut dyn Tokenable, s: &str) -> Option<Token> {
        for c in s.chars() {
            automata.consume_char(c);
        }
        automata.tokenize()
    }
    #[test]
    fn test_cmp() {
        {
            let s = "=";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Cmp(CmpOperation::Eq)) == tokenize(automata, s));
        }
        {
            let s = "<";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Cmp(CmpOperation::Lt)) == tokenize(automata, s));
        }
        {
            let s = "<=";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Cmp(CmpOperation::Leq)) == tokenize(automata, s));
        }
        {
            let s = ">=";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Cmp(CmpOperation::Geq)) == tokenize(automata, s));
        }
        {
            let s = "==";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = ".=";
            let mut automata = TACmp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
    }
    #[test]
    fn test_var() {
        {
            let s = "x";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Variable(String::from(s), F64(1.0))) == tokenize(automata, s));
        }
        {
            let s = "-1.209xa_1";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            let t = tokenize(automata, s).unwrap();
            if let Token::Variable(s, n) = t {
                assert!(s == String::from("xa_1"));
                assert!(F64(-1.209) == n);
            } else {
                panic!("Not a variable");
            }
        }
        {
            let s = "xa_";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Variable(String::from(s), F64(1.0))) == tokenize(automata, s));
        }
        {
            let s = "a_19";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Variable(String::from(s), F64(1.0))) == tokenize(automata, s));
        }
        {
            let s = "1_a";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::Variable(String::from("_a"), F64(1.0))) == tokenize(automata, s));
        }
        {
            let s = "10";
            let mut automata = TAVariable::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
    }

    #[test]
    fn test_op() {
        {
            let s = "--";
            let mut automata = TAArithOp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "+";
            let mut automata = TAArithOp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::ArithOp(ArithOperation::Add)) == tokenize(automata, s));
        }
        {
            let s = "*";
            let mut automata = TAArithOp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::ArithOp(ArithOperation::Mul)) == tokenize(automata, s));
        }
        {
            let s = "/";
            let mut automata = TAArithOp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::ArithOp(ArithOperation::Div)) == tokenize(automata, s));
        }
        {
            let s = "-";
            let mut automata = TAArithOp::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::ArithOp(ArithOperation::Sub)) == tokenize(automata, s));
        }
    }

    #[test]
    fn test_paren() {
        {
            let s = "([";
            let mut automata = TALParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "[";
            let mut automata = TALParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::LParen('[')) == tokenize(automata, s));
        }
        {
            let s = "(";
            let mut automata = TALParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::LParen('(')) == tokenize(automata, s));
        }
        {
            let s = "{";
            let mut automata = TALParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::LParen('{')) == tokenize(automata, s));
        }
        {
            let s = ")]";
            let mut automata = TARParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "]";
            let mut automata = TARParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::RParen(']')) == tokenize(automata, s));
        }
        {
            let s = ")";
            let mut automata = TARParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::RParen(')')) == tokenize(automata, s));
        }
        {
            let s = "}";
            let mut automata = TARParan::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(Some(Token::RParen('}')) == tokenize(automata, s));
        }
    }

    #[test]
    fn test_num() {
        {
            let s = "yav";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "1";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(1.0))));
        }
        {
            let s = "1.0";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(1.0))));
        }
        {
            let s = "1.235";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(1.235))));
        }
        {
            let s = "1337";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(1337.0))));
        }
        {
            let s = "--1337.211028";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "-127.i211028";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "-1337.42";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(-1337.42))));
        }
        {
            let s = "0.4269";
            let mut automata = TANum::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Num(F64(0.4269))));
        }
    }
    #[test]
    fn test_fun() {
        {
            let s = "min";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Fun(String::from(s))));
        }
        {
            let s = "max";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Fun(String::from(s))));
        }
        {
            let s = "st";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s) == Some(Token::Fun(String::from(s))));
        }
        {
            let s = "s";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "mis";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
        {
            let s = "stmin";
            let mut automata = TAFun::new();
            let automata: &mut dyn Tokenable = &mut automata;
            assert!(tokenize(automata, s).is_none());
        }
    }
}
