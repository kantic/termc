extern crate num;

pub mod input_stream;

use std::fmt;
use std::error::Error;
use error_templates::create_location_string;
use token::{Token, TokenType, SymbolicTokenType, NumberType};
use parser::tokenizer::input_stream::InputStream;
use math_context::MathContext;

/// Defines the error if an invalid / unknown token has been read.
#[derive(Clone, Debug)]
pub struct TokenError {
    /// The invalid / unknown token.
    token: String,
    /// The location mark string.
    location: String
}

impl TokenError {
    pub fn new(token: String, location: String) -> Self {
        TokenError {token: token, location: location}
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

}

impl fmt::Display for TokenError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: Unknown token found: \"{}\".\n{}", self.token, self.location)
    }
}

impl Error for TokenError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        "An unknown token has been read."
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        None
    }
}

/// The Tokenizer that groups the characters of the input streams into tokens.
pub struct Tokenizer<'a> {
    /// The mathematical environment.
    context: &'a MathContext,
    /// The character input stream.
    input_stream: InputStream<'a>,
    /// The current token.
    token: Option<Result<Token, TokenError>>
}

impl<'a> Tokenizer<'a> {

    /// Creates a new Tokenizer instance.
    pub fn new(context: &'a MathContext, input: &'a str) -> Tokenizer<'a> {
        let mut t = Tokenizer{context: context, input_stream: InputStream::new(input), token: None};
        t.token = t.read_dispatcher();
        t
    }

    /// Returns the current token from the token input stream without discarding it.
    pub fn peek(&self) -> Option<Result<Token, TokenError>> {
        self.token.clone()
    }

    /// Returns the current token from the input stream and reads the next token.
    pub fn next(& mut self) -> Option<Result<Token, TokenError>> {
        let token = self.token.clone();
        self.token = self.read_dispatcher();
        token
    }

    /// Returns the position of the current token (the last character) in the input string of the
    /// input stream.
    pub fn get_pos(& self) -> usize {
        self.input_stream.get_pos() - 1
    }

    /// Returns the input string.
    pub fn get_input(& self) -> & str {
        self.input_stream.get_input()
    }

    /// Returns true if there are no more tokens to read. Returns false otherwise.
    pub fn eof(& self) -> bool {
       match self.peek() {
           Some(_) => false,
           None => true
       }
    }

    /// Calls the correct reading method regarding the current token.
    fn read_dispatcher(& mut self) -> Option<Result<Token, TokenError>> {
        self.ignore_while(Tokenizer::is_whitespace);
        let peeked_char = match self.input_stream.peek() {
           Some(c) => c,
           None => return None
        };

        if self.context.is_literal_symbol(& peeked_char) {
            Some(Ok(self.read_char_sequence()))
        }
        else if self.context.is_number_symbol(& peeked_char) || peeked_char == '.' {
            Some(Ok(self.read_number()))
        }
        else if self.context.is_operation(& peeked_char.to_string()) {
            Some(Ok(self.read_operation()))
        }
        else if self.context.is_punctuation_symbol(& peeked_char) {
            Some(Ok(self.read_punctuation()))
        }
        else {
            // this case is executed e.g. if an input character is unusual, e.g. "§"
            Some(Err(TokenError::new(peeked_char.to_string(), create_location_string(
                self.input_stream.get_input(), self.input_stream.get_pos()))))
        }
    }

    /// Discards all characters of the input stream until the specified closure returns false.
    fn ignore_while<F>(& mut self, closure: F) -> () where F : Fn(char) -> bool {

        let mut peeked = self.input_stream.peek();
        while peeked.is_some() && closure(peeked.unwrap()) {

            self.input_stream.next();
            peeked = self.input_stream.peek();
        }
    }

    /// Reads a number token from the input stream.
    fn read_number(& mut self) -> Token {

        let mut value = String::new();
        let mut is_first_digit = true;
        let mut last_was_e = false;
        let mut formatting_zero = false;
        let mut num_type = NumberType::Real;

        while !self.input_stream.eof() {

            let peeked_char = self.input_stream.peek().unwrap();
            if self.context.is_number_symbol(& peeked_char) {
                if peeked_char == '0' && is_first_digit {
                    formatting_zero = true;
                }
                else {
                    formatting_zero = false;
                }
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == '.' && is_first_digit {
                formatting_zero = false;
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == '.' {
                formatting_zero = false;
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == 'i' && !is_first_digit {
                num_type = NumberType::Complex;
                self.input_stream.next().unwrap();
                break;
            }
            else if peeked_char == 'E' {
                formatting_zero = false;
                last_was_e = true;
                value.push(self.input_stream.next().unwrap());
            }
            else if (peeked_char == '+' || peeked_char == '-') && last_was_e {
                formatting_zero = false;
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if (peeked_char == 'x' || peeked_char == 'o' || peeked_char == 'b') && formatting_zero  {
                // formatting characters for hexadecimal, octal and binary numbers
                formatting_zero = false;
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if peeked_char == 'a' || peeked_char == 'b' || peeked_char == 'c' || peeked_char == 'd' || peeked_char == 'e' || peeked_char == 'f' {
                // digits of hexadecimal numbers (note: the 'b' is tested for in the previous else-if branch)
                formatting_zero = false;
                last_was_e = false;
                value.push(self.input_stream.next().unwrap());
            }
            else if self.context.is_literal_symbol(&peeked_char) {
                // We are adding the literal symbols to the value string although they are no number symbols, so the parsing of the number will fail.
                // So why do we do this? => To provide better error output for the user.
                // If we would not add the literal symbol, the error for the input ">>> 5h" would be: "Error: Unexpected end of input reached.".
                // If we add this literal symbol, the user will get the (much better) error message:
                // Error: Expected literal number.
                // 5h
                //  ^~~~ Found: Invalid literal symbol(s).

                value.push(self.input_stream.next().unwrap());
            }
            else {
                break;
            }

            is_first_digit = false;
        }

        Token::new(TokenType::Number(num_type), value, self.get_pos())
    }

    /// Reads a constant or a function token from the input stream.
    fn read_char_sequence(& mut self) -> Token {

        let mut value = String::new();

        while !self.input_stream.eof() {

            let peeked_char = self.input_stream.peek().unwrap();
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
            let peeked_char = self.input_stream.peek().unwrap();
            if peeked_char == '(' {
                next_is_paren = true;
            }
            else {
                next_is_paren = false;
            }
        }
        if self.context.is_built_in_constant(& value) && !next_is_paren {
            token = Token::new(TokenType::Constant, value, self.get_pos());
        }
        else if self.context.is_user_constant(& value) && !next_is_paren {
            token = Token::new(TokenType::UserConstant, value, self.get_pos());
        }
        else if self.context.is_built_in_function(& value) && next_is_paren {
            token = Token::new(TokenType::Function, value, self.get_pos());
        }
        else if self.context.is_user_function(& value) && next_is_paren {
            token = Token::new(TokenType::UserFunction, value, self.get_pos());
        }
        else if next_is_paren {
            // unknown function

            // every unknown char sequence that is followed by an open parenthesis is
            // assumed to be an unknown function

            // an unknown function is a function that is neither a built in nor a user defined
            // function; it may be the left hand side of an assignment
            token = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), value,
                               self.get_pos());
        }
        else {
            // !next_is_paren => it must be an unknown constant

            // every unknown char sequence that is **not** followed by an open parenthesis is
            // assumed to be an unknown constant

            // an unknown constant is a constant that is neither a built in nor a user defined
            // constant; it may be the left hand side of an assignment
            token = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), value,
                               self.get_pos());
        }

        token
    }

    /// Reads an operation token from the input stream.
    fn read_operation(& mut self) -> Token {

        let mut value = String::new();

        if !self.input_stream.eof() {
            value.push(self.input_stream.next().unwrap());
        }

        Token::new(TokenType::Operation, value, self.get_pos())
    }

    /// Reads a punctuation token from the input stream.
    fn read_punctuation(& mut self) -> Token {

        let mut value = String::new();

        if !self.input_stream.eof() {

            value.push(self.input_stream.next().unwrap());
        }

        Token::new(TokenType::Punctuation, value, self.get_pos())
    }

    /// Returns true if the specified character is a whitespace character, false otherwise.
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }
}
