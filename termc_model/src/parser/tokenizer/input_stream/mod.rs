
use std::fmt;
use std::error::Error;
use std::str::Chars;

/// Error that occurs when characters are accessed while the stream has no more characters to read.
#[derive(Clone, Debug)]
pub struct StreamEndError {
}

impl fmt::Display for StreamEndError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: End of character input stream reached!")
    }
}

impl Error for StreamEndError {

    /// Returns the description of the error.
    fn description(& self) -> &str {
        "There are no more characters available to read."
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        None
    }
}

/// The input stream returns operates on an input string and provides character-wise access.
pub struct InputStream<'a> {
    /// The current position of the current character in the input string.
    pos : u32,
    /// The input string.
    input: String,
    /// The character iterator over the characters of the input string.
    s : Chars<'a>,
    /// The current character.
    c: Option<char>
}

impl<'a> InputStream<'a> {

    /// Generates a new InputStream instance.
    pub fn new(input: & str) -> InputStream {
        let mut char_iterator = input.chars();
        let c = char_iterator.next();
        InputStream{pos: 0, input: String::from(input), s: char_iterator, c: c}
    }

    /// Returns the character of the current position of the stream without discarding it from the stream.
    pub fn peek(& self) -> Result<char, StreamEndError> {
        match self.c {
            Some(x) => Ok(x),
            None => Err(StreamEndError{})
        }
    }

    /// Returns the character of the current position of the stream and goes to the next character position.
    pub fn next(& mut self) -> Option<char> {

        let ret = self.c;
        self.c = self.s.next();
        self.pos += 1;
        ret
    }

    /// Returns true if there are no more characters to read. Returns false otherwise.
    pub fn eof(& self) -> bool {
        self.c.is_none()
    }

    /// Returns the current position of the input string.
    pub fn get_pos(& self) -> u32 {
        self.pos
    }

    pub fn get_input(& self) -> & str {
        & self.input
    }

    /// Returns an error string marking the specified position in the input string and appends
    /// the specified message.
    pub fn get_err_string(& self, pos: u32, msg: & str) -> String {
        let mut err_str = self.input.clone();
        err_str.push('\n');
        for _ in 0..pos {
            err_str.push(' ');
        }
        err_str.push_str("^~~~ ");
        err_str.push_str(msg);

        err_str
    }
}