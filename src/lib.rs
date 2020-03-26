use std::marker::PhantomData;

/// A token with a kind (usually an enum representing distinct token types) and its source text
#[derive(Debug)]
pub struct Token<'a, T> {
    kind: T,
    text: &'a str,
}

impl<'a, T> Token<'a, T> {
    /// Create a new token with the given kind and text
    pub fn new(kind: T, text: &'a str) -> Token<'a, T> {
        Token { kind, text }
    }

    /// Return the token's kind (usually an enum)
    pub fn kind(&self) -> &T {
        &self.kind
    }

    /// Return the token's text (a reference to the original input passed to the lexer)
    pub fn text(&self) -> &'a str {
        self.text
    }
}

pub struct LexerInput<'a> {
    pos: usize,
    bytes: &'a [u8],
}

impl<'a> LexerInput<'a> {
    fn new(input: &'a str) -> LexerInput<'a> {
        LexerInput {
            pos: 0,
            bytes: input.as_bytes(),
        }
    }

    /// Retrieve the current input position (where 0 is the first character)
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Peek at the next character
    pub fn peek(&self) -> Option<u8> {
        if self.pos < self.bytes.len() {
            Some(self.bytes[self.pos])
        } else {
            None
        }
    }

    /// Retrieve the next character and increment the input position
    pub fn next(&mut self) -> Option<u8> {
        if let Some(b) = self.peek() {
            self.pos += 1;
            Some(b)
        } else {
            None
        }
    }

    /// Call `self.next()` if the peeked character is identical to `b`
    pub fn accept(&mut self, b: u8) -> bool {
        if let Some(actual) = self.peek() {
            if actual == b {
                self.next();
                return true;
            }
        }
        false
    }

    /// Call `self.next()` while the peeked character fulfils `predicate`
    pub fn next_while<P>(&mut self, predicate: P)
        where P: Fn(&u8) -> bool {
        while let Some(byte) = self.peek() {
            if predicate(&byte) {
                self.next().unwrap();
            } else {
                return;
            }
        }
    }
}

/// The result returned from a `MatcherFn`
pub type MatcherResult<T> = Result<T, String>;

/// A matcher fn matches a character (and and any following characters) and returns a `T`
/// to indicate the kind of token (see `Token`)
pub trait MatcherFn<T> {
    fn try_match(&mut self, first_char: u8, input: &mut LexerInput) -> MatcherResult<T>;
}

impl<T, F> MatcherFn<T> for F
    where F: Fn(u8, &mut LexerInput) -> MatcherResult<T> {

    fn try_match(&mut self, first_char: u8, input: &mut LexerInput) -> MatcherResult<T> {
        (*self)(first_char, input)
    }
}

/// A lexer splits a source string into tokens using the given `MatcherFn`
pub struct Lexer<'a, F, T>
    where F: MatcherFn<T> {
    input: LexerInput<'a>,
    matcher: F,
    skip_whitespace: bool,
    matcher_t: PhantomData<T>,
}

impl<'a, M, T> Lexer<'a, M, T>
    where M: MatcherFn<T> {
    pub fn new(input: &'a str, matcher: M, skip_whitespace: bool) -> Lexer<'a, M, T> {
        Lexer {
            input: LexerInput::new(input),
            matcher,
            skip_whitespace,
            matcher_t: PhantomData::default(),
        }
    }
}

impl<'a, M, T> Iterator  for Lexer<'a, M, T>
    where M: MatcherFn<T> {

    type Item = MatcherResult<Token<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        if self.skip_whitespace {
            self.input.next_while(u8::is_ascii_whitespace);
        }

        // get start position and first character
        let start = self.input.pos;
        let byte = match self.input.next() {
            Some(b) => b,
            None => return None,
        };

        // match a token kind and mark the end of the token
        let kind = match self.matcher.try_match(byte, &mut self.input) {
            Ok(k) => k,
            Err(e) => return Some(Err(e)),
        };
        let end = self.input.pos;

        // fetch the token bytes from `self.input` and convert to `&str`
        let text = match ::std::str::from_utf8(&self.input.bytes[start..end]) {
            Ok(t) => t,
            Err(e) => return Some(Err(format!("Error: Invalid UTF-8; error = {:?}", e)))
        };

        // create a `Token` wrapper and return it
        Some(Ok(Token::new(kind, text)))
    }
}

