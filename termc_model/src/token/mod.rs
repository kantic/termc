use std::fmt;

/// Defines the sets of numbers.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum NumberType {
    Real,
    Complex
}

/// Defines the types of symbols that may occur.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SymbolicTokenType {
    UnknownConstant,
    UnknownFunction
}

/// Defines the types of tokens.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TokenType {
    Number(NumberType),
    Constant,
    UserConstant,
    Function,
    UserFunction,
    Operation,
    Punctuation,
    Symbol(SymbolicTokenType),
    FunctionArg
}

/// Defines the Token structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    /// The type of the token.
    token_type: TokenType,
    /// The string representation of the token.
    value: String,
    /// The position of the first character of the token in the user input string.
    /// (Useful for marking the character in the input string when printing error messages.)
    end_pos: u32
}

impl<'a> Token {

    /// Creates a new Token instance.
    pub fn new(token_type: TokenType, value: String, end_pos: u32) -> Token {
        Token {token_type: token_type, value: value, end_pos: end_pos}
    }

    /// Return the type of the token.
    pub fn get_type(& self) -> TokenType {
        self.token_type.clone()
    }

    /// returns the string representation of the token.
    pub fn get_value(& self) -> &str {
        & self.value
    }

    /// Returns the position of the last character of the token in the user input string.
    pub fn get_end_pos(& self) -> u32 {
        self.end_pos
    }
}

impl Into<String> for Token {
    /// Implements the Into trait for the Token struct.
    fn into(self) -> String {
        match self.token_type {
            TokenType::Number(NumberType::Complex) => format!("{0}i", self.value),
            _ => format!("{0}", self.value)
        }
    }
}

impl<'a> Into<String> for &'a Token {
    /// Implements the Into trait for Token reference type.
    fn into(self) -> String {
        match self.token_type {
            TokenType::Number(NumberType::Complex) => format!("{0}i", self.value),
            _ => format!("{0}", self.value)
        }
    }
}

impl fmt::Display for Token {

    /// Returns a formatted representation of the token.
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
