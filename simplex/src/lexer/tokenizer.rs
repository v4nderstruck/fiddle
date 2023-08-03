use std::io::BufRead;

use super::{
    token_automata::{TAArithOp, TACmp, TAFun, TALParan, TANum, TARParan, TAVariable},
    tokens::{LexState, Token},
    Tokenable,
};

/// Tokenizer; construct from other modules
#[derive(Debug)]
pub struct Tokenizer<R: BufRead> {
    reader: R,
}

impl<R: BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BufRead> IntoIterator for Tokenizer<R> {
    type Item = Token;
    type IntoIter = TokenizerIterator<R>;
    fn into_iter(self) -> Self::IntoIter {
        TokenizerIterator::new(self)
    }
}

/// The actual lexer impl for the tokenizer
pub struct TokenizerIterator<R: BufRead> {
    tokenizer: Tokenizer<R>,
    current_line_string: Option<String>,
    current_line_number: u32,
    pointer: usize,
    state_machine: Vec<Box<dyn Tokenable>>,
}

impl<R: BufRead> TokenizerIterator<R> {
    fn new(tokenizer: Tokenizer<R>) -> Self {
        let ta_var: Box<dyn Tokenable> = Box::new(TAVariable::new());
        let ta_num: Box<dyn Tokenable> = Box::new(TANum::new());
        let ta_op: Box<dyn Tokenable> = Box::new(TAArithOp::new());
        let ta_lparen: Box<dyn Tokenable> = Box::new(TALParan::new());
        let ta_rparen: Box<dyn Tokenable> = Box::new(TARParan::new());
        let ta_func: Box<dyn Tokenable> = Box::new(TAFun::new());
        let ta_cmp: Box<dyn Tokenable> = Box::new(TACmp::new());

        Self {
            tokenizer,
            current_line_string: None,
            current_line_number: 1,
            pointer: 0,
            state_machine: vec![ta_op, ta_lparen, ta_rparen, ta_cmp, ta_func, ta_num, ta_var],
        }
    }

    fn reset_state_machine(&mut self) {
        for sm in self.state_machine.iter_mut() {
            sm.reset();
        }
    }

    pub fn raise_parsing_error(&self, msg: &str) {
        eprintln!(
            "Parsing error at line {}: {}\nError occured at:{}",
            self.current_line_number,
            msg,
            self.current_line_string.as_ref().unwrap_or(&String::new())
        );
    }

    fn step(&mut self, c: char) -> Vec<(LexState, Option<Token>)> {
        let mut states = vec![];

        for sm in self.state_machine.iter_mut() {
            match sm.consume_char(c) {
                LexState::Match => {
                    states.push((LexState::Match, sm.tokenize()));
                }
                LexState::Final => {
                    states.push((LexState::Final, sm.tokenize()));
                }
                LexState::NoMatch => {
                    states.push((LexState::NoMatch, None));
                }
            }
        }
        states
    }
}

impl<R: BufRead> Iterator for TokenizerIterator<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.reset_state_machine();

        // concept for backtracking automata:
        // 1. Read char from reader
        // 2. current tokens_list = Update states of all automata using step function
        // 3. check diff between tokens_list and current tokens_list
        // 3.1 for a sm: did Match state drop to NoMatch from Match/Final? -> backtrack, return
        //   token
        // 4. repeat 1-4 until we have a token or EOF

        let mut states: Vec<(LexState, Option<Token>)> = vec![];

        loop {
            match &self.current_line_string {
                None => {
                    let mut current_line_string = String::new();
                    let num_bytes = self.tokenizer.reader.read_line(&mut current_line_string);
                    if num_bytes.is_err() {
                        let has_data_left = match self.tokenizer.reader.fill_buf() {
                            Ok(data) => !data.is_empty(),
                            Err(_) => false,
                        };

                        if has_data_left {
                            self.current_line_number += 1;
                            continue;
                        } else {
                            return None;
                        }
                    }
                    println!("read: {}", current_line_string);
                    self.current_line_string = Some(current_line_string);
                    self.pointer = 0;
                }
                Some(current_line_string) => {
                    match &current_line_string.chars().nth(self.pointer) {
                        Some(c) => {
                            if *c == '\n' && states.is_empty() {
                                self.current_line_number += 1;
                                self.pointer += 1;
                                return Some(Token::EOL);
                            }
                            if (*c == '\r' || *c == ' ' || *c == '\t' || *c == '\n')
                                && states.is_empty()
                            {
                                self.pointer += 1;
                                continue;
                            }
                            println!("consume: {}", c);

                            let current_states = self.step(*c);
                            if !states.is_empty() {
                                let mut return_token = None;
                                for (i, (current_matcher, _t)) in current_states.iter().enumerate()
                                {
                                    let (prev_matcher, prev_t) = &states[i];

                                    if *current_matcher == LexState::NoMatch
                                        && *prev_matcher == LexState::Final
                                    {
                                        // skip whitespace etc
                                        if *c == ' ' || *c == '\t' {
                                            self.pointer += 1;
                                            println!("return: {:?}", prev_t.clone());
                                            return prev_t.clone();
                                        }
                                        return_token = Some(prev_t.clone());
                                    }
                                    if *current_matcher != LexState::NoMatch
                                        && *prev_matcher != LexState::NoMatch
                                    {
                                        return_token = None; // longer Match possible
                                    }
                                }

                                if let Some(return_token) = return_token {
                                    println!("return: {:?}", return_token);
                                    return return_token;
                                }
                            }

                            states = current_states;
                            self.pointer += 1;
                        }
                        None => {
                            let has_data_left = match self.tokenizer.reader.fill_buf() {
                                Ok(data) => !data.is_empty(),
                                Err(_) => false,
                            };

                            if has_data_left && states.is_empty() {
                                self.current_line_string = None;
                                continue;
                            }
                            self.current_line_string = None;
                            self.pointer = 0;

                            for (matcher, token) in &states {
                                if LexState::Final == *matcher {
                                    return token.clone();
                                }
                            }
                            return None;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokens::{ArithOperation, CmpOperation, Token, F64};

    #[test]
    fn test_tokenizer_one_liners() {
        {
            let tokenizer = super::Tokenizer::new("(1/2.5 <= x_2 - 2.01)".as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            assert!(tokens[0] == Token::LParen('('));
            assert!(tokens[1] == Token::Num(F64(1.0)));
            assert!(tokens[2] == Token::ArithOp(ArithOperation::Div));
            assert!(tokens[3] == Token::Num(F64(2.5)));
            assert!(tokens[4] == Token::Cmp(CmpOperation::Leq));
            assert!(tokens[5] == Token::Variable("x_2".to_string()));
            assert!(tokens[6] == Token::ArithOp(ArithOperation::Sub));
            assert!(tokens[7] == Token::Num(F64(2.01)));
            assert!(tokens[8] == Token::RParen(')'));
            assert!(tokens.len() == 9);
        }

        {
            let tokenizer = super::Tokenizer::new("max x /-2.1] min stru".as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            assert!(tokens[0] == Token::Fun("max".to_string()));
            assert!(tokens[1] == Token::Variable("x".to_string()));
            assert!(tokens[2] == Token::ArithOp(ArithOperation::Div));
            assert!(tokens[3] == Token::Num(F64(-2.1)));
            assert!(tokens[4] == Token::RParen(']'));
            assert!(tokens[5] == Token::Fun("min".to_string()));
            assert!(tokens[6] == Token::Variable("stru".to_string()));
            assert!(tokens.len() == 7);
        }
        {
            let tokenizer = super::Tokenizer::new("1 + 2".as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            assert!(tokens.len() == 3);
            assert!(tokens[0] == Token::Num(F64(1.0)));
            assert!(tokens[1] == Token::ArithOp(ArithOperation::Add));
            assert!(tokens[2] == Token::Num(F64(2.0)));
        }
        {
            let tokenizer = super::Tokenizer::new("2s + 7x_1 = -1.257ax_22".as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            println!("{:?}", tokens);
            assert!(tokens[0] == Token::Num(F64(2.0)));
            assert!(tokens[1] == Token::Variable("s".to_string()));
            assert!(tokens[2] == Token::ArithOp(ArithOperation::Add));
            assert!(tokens[3] == Token::Num(F64(7.0)));
            assert!(tokens[4] == Token::Variable("x_1".to_string()));
            assert!(tokens[5] == Token::Cmp(CmpOperation::Eq));
            assert!(tokens[6] == Token::Num(F64(-1.257)));
            assert!(tokens[7] == Token::Variable("ax_22".to_string()));
            assert!(tokens.len() == 8);
        }
        {
            let tokenizer =
                super::Tokenizer::new("max    x    /    -2.1]        min     -stru ".as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            println!("{:?}", tokens);
            assert!(tokens[0] == Token::Fun("max".to_string()));
            assert!(tokens[1] == Token::Variable("x".to_string()));
            assert!(tokens[2] == Token::ArithOp(ArithOperation::Div));
            assert!(tokens[3] == Token::Num(F64(-2.1)));
            assert!(tokens[4] == Token::RParen(']'));
            assert!(tokens[5] == Token::Fun("min".to_string()));
            assert!(tokens[6] == Token::ArithOp(ArithOperation::Sub));
            assert!(tokens[7] == Token::Variable("stru".to_string()));
            assert!(tokens.len() == 8);
        }
    }

    #[test]
    fn test_tokenizer_multi_line() {
        {
            let input = "
max {x1 - x2  = -1.291mi}
st {
    -1.21x1 / -a >= 1000
}
";
            let tokenizer = super::Tokenizer::new(input.as_bytes());
            let tokens = tokenizer.into_iter().collect::<Vec<_>>();
            println!("{:?}", tokens);
            let truth = vec![
                Token::EOL,
                Token::Fun("max".to_string()),
                Token::LParen('{'),
                Token::Variable("x1".to_string()),
                Token::ArithOp(ArithOperation::Sub),
                Token::Variable("x2".to_string()),
                Token::Cmp(CmpOperation::Eq),
                Token::Num(F64(-1.291)),
                Token::Variable("mi".to_string()),
                Token::RParen('}'),
                Token::EOL,
                Token::Fun("st".to_string()),
                Token::LParen('{'),
                Token::EOL,
                Token::Num(F64(-1.21)),
                Token::Variable("x1".to_string()),
                Token::ArithOp(ArithOperation::Div),
                Token::ArithOp(ArithOperation::Sub),
                Token::Variable("a".to_string()),
                Token::Cmp(CmpOperation::Geq),
                Token::Num(F64(1000.0)),
                Token::EOL,
                Token::RParen('}'),
                Token::EOL,
            ];
            assert!(tokens == truth);
            assert!(tokens.len() == truth.len());
        }
    }
}
