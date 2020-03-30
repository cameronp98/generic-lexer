mod input;

pub use input::BufferedInput;

use std::error::Error;
use std::fmt;

/// A token with a kind (usually an enum representing distinct token types) and its source text

#[derive(Debug)]
pub struct Token<K> {
    kind: K,
    text: String,
}

impl<K> Token<K> {
    /// Create a new token with the given kind and text
    pub fn new(kind: K, text: String) -> Token<K> {
        Token { kind, text }
    }

    /// Return the token's kind (usually an enum)
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// Return the token's text
    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn into_text(self) -> String {
        self.text
    }
}

#[derive(Debug)]
pub enum MatchError {
    Unexpected(char),
    Custom(String),
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MatchError::Unexpected(c) => writeln!(f, "Unexpected '{}'", c),
            MatchError::Custom(ref msg) => msg.fmt(f),
        }
    }
}

impl Error for MatchError {}

pub type MatchResult<T> = Result<T, MatchError>;

/// A matcher fn matches a character (and and any following characters) and returns a `T`
/// to indicate the kind of token (see `Token`)
///
/// `input` is always fresh (i.e. its buffer is empty)
pub trait Matcher<K> {
    fn try_match(&self, first_char: char, input: &mut BufferedInput) -> MatchResult<K>;
}

impl<F, K> Matcher<K> for F
    where F: Fn(char, &mut BufferedInput) -> MatchResult<K> {
    fn try_match(&self, first_char: char, input: &mut BufferedInput) -> MatchResult<K> {
        (*self)(first_char, input)
    }
}

/// A lexer splits a source string into tokens using the given `MatcherFn`
pub struct Lexer<'a, K> {
    input: BufferedInput<'a>,
    matcher: &'a dyn Matcher<K>,
    skip_whitespace: bool,
}

impl<'a, K> Lexer<'a, K> {
    pub fn new(input: &'a str, matcher: &'a dyn Matcher<K>, skip_whitespace: bool) -> Lexer<'a, K> {
        Lexer {
            input: BufferedInput::new(input),
            matcher,
            skip_whitespace,
        }
    }
}

impl<'a, K> Iterator  for Lexer<'a, K> {
    type Item = MatchResult<Token<K>>;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        if self.skip_whitespace {
            self.input.skip_whitespace();
        }

        // get first character
        let first_char = match self.input.accept() {
            Some(byte) => byte,
            None => return None,
        };

        // match a token kind and mark the end of the token
        let kind = match self.matcher.try_match(first_char, &mut self.input) {
            Ok(kind) => kind,
            Err(err) => return Some(Err(err)),
        };

        // create a `Token` wrapper and return it
        Some(Ok(Token::new(kind, self.input.take_buffer())))
    }
}

