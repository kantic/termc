
extern crate serde_json;

use std::fmt;
use serde_json::{Value, Map};
use serialization::{Serialization, SerializationError};

/// Defines the sets of numbers.
#[derive(Clone, PartialEq, Debug)]
pub enum NumberType {
    Real,
    Complex
}

impl Serialization for NumberType {

    fn build_value(& self) -> Value {

        let mut m : Map<String, Value> = Map::new();
        match *self {
            NumberType::Real => m.insert(String::from("value"), Value::String(String::from("Real"))),
            NumberType::Complex => m.insert(String::from("value"), Value::String(String::from("Complex")))
        };

        Value::Object(m)
    }

    fn build_instance(v: Value) -> Result<NumberType, SerializationError> {
        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };
        let val = match m.remove("value") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("NumberType: value")))
        };
        match val {
            Value::String(str) => {
                match str.as_ref() {
                    "Real" => Ok(NumberType::Real),
                    "Complex" => Ok(NumberType::Complex),
                    _ => Err(SerializationError::ValueError(str))
                }
            },
            _ => Err(SerializationError::ValueTypeError(String::from("String")))
        }
    }
}

/// Defines the types of symbols that may occur.
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolicTokenType {
    UnknownConstant,
    UnknownFunction
}

impl Serialization for SymbolicTokenType {

    fn build_value(& self) -> Value {
        let mut m : Map<String, Value> = Map::new();
        match *self {
            SymbolicTokenType::UnknownConstant => m.insert(String::from("value"), Value::String(String::from("UnknownConstant"))),
            SymbolicTokenType::UnknownFunction => m.insert(String::from("value"), Value::String(String::from("UnknownFunction")))
        };

        Value::Object(m)
    }

    fn build_instance(v: Value) -> Result<SymbolicTokenType, SerializationError> {
        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };
        let val = match m.remove("value") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("SymbolicTokenType: value")))
        };
        match val {
            Value::String(str) => {
                match str.as_ref() {
                    "UnknownConstant" => Ok(SymbolicTokenType::UnknownConstant),
                    "UnknownFunction" => Ok(SymbolicTokenType::UnknownFunction),
                    _ => Err(SerializationError::ValueError(str))
                }
            },
            _ => Err(SerializationError::ValueTypeError(String::from("String")))
        }
    }
}

/// Defines the types of tokens.
#[derive(Clone, PartialEq, Debug)]
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

impl Serialization for TokenType {

    fn build_value(& self) -> Value {
        let mut m : Map<String, Value> = Map::new();
        match *self {
            TokenType::Number(ref typ) => {
                m.insert(String::from("type"), Value::String(String::from("Number")));
                let num_val = typ.build_value();
                m.insert(String::from("value"), num_val);
            },

            TokenType::Constant => {
                m.insert(String::from("type"), Value::String(String::from("Constant")));
                m.insert(String::from("value"), Value::String(String::from("Constant")));
            },

            TokenType::UserConstant => {
                m.insert(String::from("type"), Value::String(String::from("UserConstant")));
                m.insert(String::from("value"), Value::String(String::from("UserConstant")));
            },

            TokenType::Function => {
                m.insert(String::from("type"), Value::String(String::from("Function")));
                m.insert(String::from("value"), Value::String(String::from("Function")));
            },

            TokenType::UserFunction => {
                m.insert(String::from("type"), Value::String(String::from("UserFunction")));
                m.insert(String::from("value"), Value::String(String::from("UserFunction")));
            },

            TokenType::Operation => {
                m.insert(String::from("type"), Value::String(String::from("Operation")));
                m.insert(String::from("value"), Value::String(String::from("Operation")));
            },

            TokenType::Punctuation => {
                m.insert(String::from("type"), Value::String(String::from("Punctuation")));
                m.insert(String::from("value"), Value::String(String::from("Punctuation")));
            },

            TokenType::Symbol(ref sym) => {
                m.insert(String::from("type"), Value::String(String::from("Symbol")));
                let sym_val = sym.build_value();
                m.insert(String::from("value"), sym_val);
            },

            TokenType::FunctionArg => {
                m.insert(String::from("type"), Value::String(String::from("FunctionArg")));
                m.insert(String::from("value"), Value::String(String::from("FunctionArg")));
            }
        };

        Value::Object(m)
    }

    fn build_instance(v: Value) -> Result<TokenType, SerializationError> {
        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };

        let t = match m.remove("type") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("TokenType: type")))
        };

        let val = match m.remove("value") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("TokenType: value")))
        };

        match t {
            Value::String(ref str) => {
                match str.as_ref() {
                    "Number" => {
                        let num_type = try!(NumberType::build_instance(val));
                        Ok(TokenType::Number(num_type))
                    },

                    "Constant" => Ok(TokenType::Constant),
                    "UserConstant" => Ok(TokenType::UserConstant),
                    "Function" => Ok(TokenType::Function),
                    "UserFunction" => Ok(TokenType::UserFunction),
                    "Operation" => Ok(TokenType::Operation),
                    "Punctuation" => Ok(TokenType::Punctuation),
                    "Symbol" => {
                        let sym_type = try!(SymbolicTokenType::build_instance(val));
                        Ok(TokenType::Symbol(sym_type))
                    },

                    "FunctionArg" => Ok(TokenType::FunctionArg),
                    _ => Err(SerializationError::ValueError(String::from(str.as_ref())))
                }
            },

            _ => {
                Err(SerializationError::ValueTypeError(String::from("String")))
            }
        }
    }
}

/// Defines the Token structure.
#[derive(Clone, Debug)]
pub struct Token {
    /// The type of the token.
    token_type: TokenType,
    /// The string representation of the token.
    value: String,
    /// The position of the first character of the token in the user input string.
    /// (Useful for marking the character in the input string when printing error messages.)
    end_pos: u32
}

impl Serialization for Token {

    /// Generates the JSON object for serialization.
    fn build_value(& self) -> Value {
        let mut m : Map<String, Value> = Map::new();
        let t_type_val = self.token_type.build_value();
        let value_val = Value::String(self.value.clone());
        let end_pos_val = Value::U64(self.end_pos as u64);
        m.insert(String::from("tokenType"), t_type_val);
        m.insert(String::from("value"), value_val);
        m.insert(String::from("endPos"), end_pos_val);

        Value::Object(m)
    }

    /// Generates a deserialized instance from the specified JSON object.
    fn build_instance(v: Value) -> Result<Token, SerializationError> {
        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };

        let t_type_val = match m.remove("tokenType") {
            Some(v) => try!(TokenType::build_instance(v)),
            None => return Err(SerializationError::MissingValueError(String::from("Token: tokenType")))
        };
        let value_val = match m.remove("value") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("Token: value")))
        };
        let end_pos_val = match m.remove("endPos") {
            Some(v) => v,
            None => return Err(SerializationError::MissingValueError(String::from("Token: endPos")))
        };


        let value_val = match value_val {
            Value::String(str) => str,
            _ => return Err(SerializationError::ValueTypeError(String::from("String")))
        };
        let end_pos_val = match end_pos_val {
            Value::U64(u) => u,
            _ => return Err(SerializationError::ValueTypeError(String::from("U64")))
        };

        Ok(Token {token_type: t_type_val, value: value_val, end_pos: end_pos_val as u32})
    }
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

impl fmt::Display for Token {

    /// Returns a formatted representation of the token.
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}