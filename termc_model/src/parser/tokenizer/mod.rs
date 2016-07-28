pub mod input_stream;

use std::fmt;
use std::error::Error;
use parser::tokenizer::input_stream::InputStream;
use parser::tokenizer::input_stream::StreamEndError;
use math_context::MathContext;

/// Defines the types of tokens.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    Number,
    Constant,
    Function,
    Operation,
    Punctuation,
    UnknownToken,
}

/// Defines the Token structure.
#[derive(Clone, Debug)]
pub struct Token {
    /// The type of the token.
    token_type: TokenType,
    /// The string representation of the token.
    value: String
}

impl<'a> Token {

    /// Creates a new Token instance.
    pub fn new(token_type: TokenType, value: &str) -> Token {
        Token {token_type: token_type, value: String::from(value)}
    }

    /// Return the type of the token.
    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    /// returns the string representation of the token.
    pub fn get_value(&self) -> &str {
        & self.value
    }
}

impl fmt::Display for Token {

    /// Returns a formatted representation of the token.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

/// Defines the errors that can occur while tokenizing the input stream.
#[derive(Clone, Debug)]
pub enum TokenError {
    /// The input stream has reached it's end but a new token has been requested.
    StreamEndError(StreamEndError),
    /// An unknown token has been read.
    UnknownTokenError(Token, String)
}

impl From<StreamEndError> for TokenError {

    /// Converts an StreamEndError into a TokenError.
    fn from(err: StreamEndError) -> TokenError {
        TokenError::StreamEndError(err)
    }
}

impl fmt::Display for TokenError {

    /// Returns the formatted error message.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::StreamEndError(_) => write!(f, "Error: End of token input stream reached!."),
            TokenError::UnknownTokenError(ref t, ref l) => write!(f, "Error: Unknown token found: \"{}\".\n{}", t, l)
        }
    }
}

impl Error for TokenError {

    /// Returns the description of the error.
    fn description(&self) -> &str {
        match *self {
            TokenError::StreamEndError(_) => "There are no more tokens available to read.",
            TokenError::UnknownTokenError(_, _) => "A token could not be interpreted."
        }
    }

    /// Returns the preceding error.
    fn cause(&self) -> Option<&Error> {
        match *self {
            TokenError::StreamEndError(ref err) => Some(err),
            TokenError::UnknownTokenError(_, _) => None
        }
    }
}

/// The Tokenizer that groups the characters of the input streams into tokens.
pub struct Tokenizer<'a> {
    /// The mathematical environment.
    context: &'a MathContext,
    /// The character input stream.
    input_stream: InputStream<'a>,
    /// The current token.
    token: Result<Token, TokenError>
}

impl<'a> Tokenizer<'a> {

    /// Creates a new Tokenizer instance.
    pub fn new(context: &'a MathContext, input: &'a str) -> Tokenizer<'a> {
        let mut t = Tokenizer{context: context, input_stream: InputStream::new(input), token: Err(TokenError::StreamEndError(StreamEndError{}))};
        t.token = t.read_dispatcher();
        t
    }

    /// Returns the current token from the token input stream without discarding it.
    pub fn peek(&self) -> Result<Token, TokenError> {

        match self.token {
            Ok(ref x) => Ok(x.clone()),
            Err(ref e) => match e {
                &TokenError::StreamEndError(ref err) => Err(TokenError::StreamEndError(err.clone())),
                &TokenError::UnknownTokenError(ref t, ref l) => Err(TokenError::UnknownTokenError(t.clone(), l.clone()))
            }
        }
    }

    /// Returns the current token from the input stream and reads the next token.
    pub fn next(&mut self) -> Result<Token, TokenError> {
        let token : Result<Token, TokenError> = self.token.clone();
        self.token = self.read_dispatcher();
        token
    }

    /// Returns the position of the current token (the last character) in the input string of the
    /// input stream.
    pub fn get_pos(&self) -> u32 {
        match self.token {
            Ok(ref x) => self.input_stream.get_pos() - (x.get_value().len() as u32),
            Err(_) => self.input_stream.get_pos() - 1
        }
    }

    /// Returns an error string marking the specified position in the input string of the input stream
    /// and appends the specified message.
    pub fn get_err_string(&self, pos: u32, msg: & str) -> String {
        self.input_stream.get_err_string(pos, msg)
    }

    /// Returns true if there are no more tokens to read. Returns false otherwise.
    pub fn eof(&self) -> bool {
        match self.token {
            Ok(_) => false,
            Err(ref e) => match e {
                &TokenError::StreamEndError(_) => {
                    true
                },
                _ => {
                    false
                }
            }
        }
    }

    /// Calls the correct reading method regarding the current token.
    fn read_dispatcher(&mut self) -> Result<Token, TokenError> {
        self.ignore_while(Tokenizer::is_whitespace);
        let peeked_char = try!(self.input_stream.peek());

        if self.context.is_literal_symbol(& peeked_char) {
            return self.read_char_sequence();
        }
        else if self.context.is_number_symbol(& peeked_char) || peeked_char == '.' {
            return Ok(self.read_number());
        }
        else if self.context.is_operation(& peeked_char.to_string()) {
            return Ok(self.read_operation());
        }
        else if self.context.is_punctuation_symbol(& peeked_char) {
            return Ok(self.read_punctuation());
        }
        else {
            return Err(TokenError::UnknownTokenError(Token::new(TokenType::UnknownToken,
                                                                & peeked_char.to_string()), self.get_err_string(self.get_pos() + 1, "")));
        }
    }

    /// Discards all characters of the input stream until the specified closure returns false.
    fn ignore_while<F>(&mut self, closure: F) -> () where F : Fn(char) -> bool {

        let mut peeked = self.input_stream.peek();
        while peeked.is_ok() && closure(peeked.unwrap()) {

            self.input_stream.next();
            peeked = self.input_stream.peek();
        }
    }

    /// Reads a number token from the input stream.
    fn read_number(&mut self) -> Token {

        let mut value = String::new();
        let mut is_first_digit : bool = true;

        while !self.input_stream.eof() {

            let peeked_char = self.input_stream.peek().unwrap();
            if self.context.is_number_symbol(& peeked_char) {
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == '.' && is_first_digit {
                value.push('0');
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == '.' {
                value.push(self.input_stream.next().unwrap());
            }
            else {
                break;
            }

            is_first_digit = false;
        }

        Token::new(TokenType::Number, &value)
    }

    /// Reads a constant or a function token from the input stream.
    fn read_char_sequence(&mut self) -> Result<Token, TokenError> {

        let mut value = String::new();

        while !self.input_stream.eof() {

            let peeked_char = try!(self.input_stream.peek());
            if self.context.is_literal_symbol(& peeked_char) || self.context.is_number_symbol(& peeked_char) {
                value.push(self.input_stream.next().unwrap());
            }
            else {
                break;
            }
        }

        let token : Token;
        let mut next_is_paren = false;
        if !self.input_stream.eof() {
            let peeked_char = try!(self.input_stream.peek());
            if peeked_char == '(' {
                next_is_paren = true;
            }
            else {
                next_is_paren = false;
            }
        }
        if self.context.is_constant(& value) && !next_is_paren {
            token = Token::new(TokenType::Constant, & value);
        }
        else if self.context.is_function(& value) && next_is_paren {
            token = Token::new(TokenType::Function, & value);
        }
        else {
            if next_is_paren {
                value.push_str("(...)");
            }
            return Err(TokenError::UnknownTokenError(Token::new(TokenType::UnknownToken, & value), self.get_err_string(self.get_pos(), "")));
        }

        Ok(token)
    }

    /// Reads an operation token from the input stream.
    fn read_operation(&mut self) -> Token {

        let mut value = String::new();

        if !self.input_stream.eof() {

            value.push(self.input_stream.next().unwrap());
        }

        Token::new(TokenType::Operation, & value)
    }

    /// Reads a punctuation token from the input stream.
    fn read_punctuation(&mut self) -> Token {

        let mut value = String::new();

        if !self.input_stream.eof() {

            value.push(self.input_stream.next().unwrap());
        }

        Token::new(TokenType::Punctuation, & value)
    }

    /// Returns true if the specified character is a whitespace character.
    /// Returns false otherwise.
    fn is_whitespace(c: char) -> bool {
        " \t\n".contains(c)
    }
}