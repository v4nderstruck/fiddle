use std::io::Read;

use super::{tokens::Token, Tokenable};

/// Tokenizer; construct from other modules
pub struct Tokenizer<R: Read> {
    reader: R,
}

pub struct TokenizerIterator<R: Read> {
    tokenizer: Tokenizer<R>,
    state: Vec<Box<dyn Tokenable>>,
}

impl<R: Read> Iterator for TokenizerIterator<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
