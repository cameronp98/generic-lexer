use std::iter::Peekable;
use std::str::Chars;

pub struct BufferedInput<'a> {
    buffer: String,
    chars: Peekable<Chars<'a>>,
}

impl<'a> BufferedInput<'a> {
    /// Create a new buffered lexer input
    pub(crate) fn new(input: &'a str) -> BufferedInput<'a> {
        BufferedInput {
            buffer: String::new(),
            chars: input.chars().peekable(),
        }
    }

    /// Copy out the buffer and clear it
    pub fn take_buffer(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.buffer.clear();
        buffer
    }

    /// Peek at the next character
    #[inline(always)]
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&c| c)
    }

    /// Get the next character but don't push it to the buffer
    #[inline(always)]
    pub fn skip(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Skip if the given predicate is true
    pub fn skip_if<P: Fn(&char) -> bool>(&mut self, predicate: P) -> Option<char> {
        if let Some(c) = self.peek() {
            if predicate(&c) {
                self.skip();
                return Some(c);
            }
        }
        None
    }

    /// Skip while the given predicate is true
    pub fn skip_while<P: Fn(&char) -> bool>(&mut self, predicate: P) {
        while let Some(_) = self.skip_if(&predicate) {}
    }

    /// Retrieve the next character and increment the input position
    pub fn accept(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.buffer.push(c);
            Some(c)
        } else {
            None
        }
    }

    /// Call `self.next()` if the peeked character is identical to `b`
    pub fn accept_if<P: Fn(&char) -> bool>(&mut self, predicate: P) -> Option<char> {
        if let Some(c) = self.peek() {
            if predicate(&c) {
                self.accept().unwrap();
                return Some(c)
            }
        }
        None
    }

    /// Call `self.next()` while the peeked character fulfils `predicate`
    pub fn accept_while<P: Fn(&char) -> bool>(&mut self, predicate: P) {
        while let Some(_) = self.accept_if(&predicate) {}
    }

    /// Accept the given byte and return `ok`, or else return `default`
    ///
    /// This is useful for matching multi-character tokens:
    /// ```rust
    /// match c {
    ///     '=' => input.accept_or(|&c| c == '=', TokenKind::DoubleEquals, TokenKind::Equals),
    ///     _ => {},
    /// }
    /// ```
    pub fn accept_or<P: Fn(&char) -> bool, T>(&mut self, predicate: P, ok: T, default: T) -> T {
        if let Some(_) = self.accept_if(predicate) {
            ok
        } else {
            default
        }
    }

    /// Skip whitespace, preserving the original buffer before any whitespace was encountered
    #[inline]
    pub fn skip_whitespace(&mut self) {
        self.skip_while(char::is_ascii_whitespace);
    }
}