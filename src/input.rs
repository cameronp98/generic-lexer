use std::iter::Peekable;
use std::str::Chars;

pub struct LexerInput<'a> {
    buffer: String,
    chars: Peekable<Chars<'a>>,
}

impl<'a> LexerInput<'a> {
    /// Create a new LexerInput
    pub(crate) fn new(input: &'a str) -> LexerInput<'a> {
        LexerInput {
            buffer: String::new(),
            chars: input.chars().peekable(),
        }
    }

    /// Copy out the current buffer and reset it to an empty string
    pub(crate) fn take_buffer(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.buffer.clear();
        buffer
    }

    /// Peek at the next character
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&c| c)
    }

    /// Retrieve the next character and increment the input position
    pub fn next(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.buffer.push(c);
            Some(c)
        } else {
            None
        }
    }

    /// Call `self.next()` if the peeked character is identical to `b`
    pub fn accept<P: Fn(&char) -> bool>(&mut self, predicate: P) -> Option<char> {
        if let Some(c) = self.peek() {
            if predicate(&c) {
                self.next().unwrap();
                return Some(c)
            }
        }
        None
    }

    /// Call `self.next()` while the peeked character fulfils `predicate`
    pub fn accept_while<P: Fn(&char) -> bool>(&mut self, predicate: P) {
        while let Some(_) = self.accept(&predicate) {}
    }

    /// Accept the given byte and return `ok`, or else return `default`
    ///
    /// This is useful for matching multi-character tokens:
    /// ```rust
    /// match c {
    ///     b'=' => input.accept_or(b'=', TokenKind::DoubleEquals, TokenKind::Equals),
    ///     _ => {},
    /// }
    /// ```
    pub fn accept_or<P: Fn(&char) -> bool, T>(&mut self, predicate: P, ok: T, default: T) -> T {
        if let Some(_) = self.accept(predicate) {
            ok
        } else {
            default
        }
    }

    /// Skip whitespace, preserving the original buffer before any whitespace was encountered
    #[inline]
    pub fn skip_whitespace(&mut self) {
        let buffer = self.take_buffer();
        self.accept_while(char::is_ascii_whitespace);
        self.buffer = buffer
    }
}