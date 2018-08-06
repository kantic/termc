
use std::str::Chars;
use std::iter::Peekable;

/// The input stream operates on an input string and provides character-wise access.
pub struct InputStream<'a> {
    /// The input string.
    input: &'a str,
    /// The position of the next character in the input string.
    pos: usize,
    /// The iterator over the input string.
    iterator: Peekable<Chars<'a>>
}

impl<'a> InputStream<'a> {

    /// Generates a new InputStream instance.
    pub fn new(input: &'a str) -> InputStream<'a> {
        InputStream{input: input, pos: 0, iterator: input.chars().peekable()}
    }

    /// Returns the character of the next position of the stream without discarding it from the stream.
    pub fn peek(& mut self) -> Option<char> {
        self.iterator.peek().map(|x| *x)
    }

    /// Returns the character of the next position of the stream and advances the stream position.
    pub fn next(& mut self) -> Option<char> {
        match self.iterator.next() {
            Some(x) => {
                self.pos += 1;
                Some(x)
            },
            None => None
        }
    }

    /// Returns true if there are no more characters to read. Returns false otherwise.
    pub fn eof(& mut self) -> bool {
        self.iterator.peek().is_none()
    }

    /// Returns the current position of the input string.
    pub fn get_pos(& self) -> usize {
        self.pos
    }

    /// Returns the input string.
    pub fn get_input(& self) -> & str {
        & self.input
    }
}
