use self::tokens::LexToken;

pub mod token_automata;
pub mod tokenizer;
pub mod tokens;

/// Trait for "token automata"
/// We compose the Backtrack automata using a "token automata" which only
/// task is to match their respective token.
pub trait Tokenable {
    /// Automata matching the current char.
    /// Return LexToken::Token(Token) if match is full token
    /// Return LexToken::NoMatch if the current char is not matched.
    fn consume_char(&mut self, c: char) -> LexToken;
    /// Reset the automata to its initial state.
    fn reset(&mut self);
    /// Get the token if the automata is in a final state.
    fn tokenize(&self) -> Option<tokens::Token>;
}
