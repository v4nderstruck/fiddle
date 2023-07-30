use std::io::{BufRead, Read};

use super::{
    token_automata::{TAArithOp, TAEq, TAFun, TALParan, TANum, TARParan, TAVariable},
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
        let ta_eq: Box<dyn Tokenable> = Box::new(TAEq::new());

        Self {
            tokenizer,
            current_line_string: None,
            current_line_number: 1,
            pointer: 0,
            state_machine: vec![ta_var, ta_num, ta_op, ta_lparen, ta_rparen, ta_func, ta_eq],
        }
    }

    fn reset_state_machine(&mut self) {
        for sm in self.state_machine.iter_mut() {
            sm.reset();
        }
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
                        return None;
                    }
                    self.current_line_string = Some(current_line_string);
                    self.pointer = 0;
                }
                Some(current_line_string) => match current_line_string.chars().nth(self.pointer) {
                    Some(c) => {
                        if c == '\n' {
                            self.current_line_number += 1;
                        }

                        let current_states = self.step(c);
                        if !states.is_empty() {
                            for (i, (current_matcher, _t)) in current_states.iter().enumerate() {
                                let (prev_matcher, prev_t) = &states[i];

                                if *current_matcher == LexState::NoMatch
                                    && (*prev_matcher == LexState::Match
                                        || *prev_matcher == LexState::Final)
                                {
                                    // skip whitespace etc
                                    if c == '\r' || c == '\n' || c == ' ' || c == '\t' {
                                        self.pointer += 1;
                                    }
                                    return prev_t.clone();
                                }
                            }
                        }
                        states = current_states;
                        self.pointer += 1;
                    }
                    None => {
                        self.current_line_string = None;
                        self.pointer = 0;

                        for (matcher, token) in &states {
                            if LexState::Final == *matcher {
                                return token.clone();
                            }
                        }
                        return None;
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokens::{ArithOperation, Token, F64};

    #[test]
    fn test_tokenizer() {
        let tokenizer = super::Tokenizer::new("1 + 2".as_bytes());
        let tokens = tokenizer.into_iter().collect::<Vec<_>>();
        assert!(tokens.len() == 3);
        assert!(tokens[0] == Token::Num(F64(1.0)));
        assert!(tokens[1] == Token::ArithOp(ArithOperation::Add));
        assert!(tokens[2] == Token::Num(F64(2.0)));
    }
}
