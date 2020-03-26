/// A token with a kind (usually an enum representing distinct token types) and its source text
#[derive(Debug)]
pub struct Token<'a, K> {
    kind: K,
    text: &'a str,
}

impl<'a, K> Token<'a, K> {
    /// Create a new token with the given kind and text
    pub fn new(kind: K, text: &'a str) -> Token<'a, K> {
        Token { kind, text }
    }

    /// Return the token's kind (usually an enum)
    pub fn kind(&self) -> &K {
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
        if let Some(byte) = self.peek() {
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Call `self.next()` if the peeked character is identical to `b`
    pub fn accept(&mut self, byte: u8) -> bool {
        if let Some(actual) = self.peek() {
            if actual == byte {
                self.next();
                return true;
            }
        }
        false
    }

    /// Accept the given byte and return `ok`, or else return `default`
    ///
    /// This is useful for matching multi-character tokens:
    /// ```rust
    /// match byte {
    ///     b'=' => input.accept_or(b'=', TokenKind::DoubleEquals, TokenKind::Equals),
    ///     _ => {},
    /// }
    /// ```
    pub fn accept_or<T>(&mut self, byte: u8, ok: T, default: T) -> T {
        if self.accept(byte) {
            ok
        } else {
            default
        }
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
pub type MatcherResult<K> = Result<K, String>;

/// A matcher fn matches a character (and and any following characters) and returns a `T`
/// to indicate the kind of token (see `Token`)
pub trait MatcherFn<K> {
    fn try_match(&self, first_char: u8, input: &mut LexerInput) -> MatcherResult<K>;
}

impl<K, F> MatcherFn<K> for F
    where F: Fn(u8, &mut LexerInput) -> MatcherResult<K> {

    fn try_match(&self, first_char: u8, input: &mut LexerInput) -> MatcherResult<K> {
        (*self)(first_char, input)
    }
}

/// A lexer splits a source string into tokens using the given `MatcherFn`
pub struct Lexer<'a, K> {
    input: LexerInput<'a>,
    matcher: &'a dyn MatcherFn<K>,
    skip_whitespace: bool,
}

impl<'a, K> Lexer<'a, K> {
    pub fn new(input: &'a str, matcher: &'a dyn MatcherFn<K>, skip_whitespace: bool) -> Lexer<'a, K> {
        Lexer {
            input: LexerInput::new(input),
            matcher,
            skip_whitespace,
        }
    }
}

impl<'a, K> Iterator  for Lexer<'a, K> {
    type Item = MatcherResult<Token<'a, K>>;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        if self.skip_whitespace {
            self.input.next_while(u8::is_ascii_whitespace);
        }

        // get start position and first character
        let start = self.input.pos;
        let byte = match self.input.next() {
            Some(byte) => byte,
            None => return None,
        };

        // match a token kind and mark the end of the token
        let kind = match self.matcher.try_match(byte, &mut self.input) {
            Ok(kind) => kind,
            Err(err) => return Some(Err(err)),
        };
        let end = self.input.pos;

        // fetch the token bytes from `self.input` and convert to `&str`
        let text = match ::std::str::from_utf8(&self.input.bytes[start..end]) {
            Ok(text) => text,
            Err(err) => return Some(Err(format!("Error: Invalid UTF-8; error = {:?}", err)))
        };

        // create a `Token` wrapper and return it
        Some(Ok(Token::new(kind, text)))
    }
}

