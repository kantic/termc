
extern crate serde_json;

use serde_json::Value;
use std::error::Error;
use std::fmt;

/// Defines the errors that may occur while serializing and deserializing.
#[derive(Debug)]
pub enum SerializationError {

    /// A wrong value has been read (the wrong value).
    ValueError(String),
    /// A wrong value type (expected type).
    ValueTypeError(String),
    /// A value is missing (the missing value name).
    MissingValueError(String),
    /// The serialization string could not be build (type of the object).
    StringGenerationError(serde_json::error::Error),
    /// The deserialized map could not be build (type of the object).
    MapGenerationError(serde_json::error::Error)
}

impl fmt::Display for SerializationError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializationError::ValueError(ref val) => write!(f, "Error: Wrong value found: \"{}\".", val),
            SerializationError::ValueTypeError(ref t) => write!(f, "Error: Wrong value type found. Expected: \"{}\"", t),
            SerializationError::MissingValueError(ref name) => write!(f, "Error: Missing value: \"{}\".", name),
            SerializationError::StringGenerationError(ref err) => write!(f, "Error: Serialization failed due to the following error:\n{}.", err),
            SerializationError::MapGenerationError(ref err) => write!(f, "Error: Deserialization failed due to the following error:\n{}", err)
        }
    }
}

impl Error for SerializationError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            SerializationError::ValueError(_) => "Found an unexpected value.",
            SerializationError::ValueTypeError(_) => "Found a wrong value type.",
            SerializationError::MissingValueError(_) => "A value is missing.",
            SerializationError::StringGenerationError(_) => "A serialization string could not be generated.",
            SerializationError::MapGenerationError(_) => "A deserialization map could not be generated."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            SerializationError::ValueError(_) => None,
            SerializationError::ValueTypeError(_) => None,
            SerializationError::MissingValueError(_) => None,
            SerializationError::StringGenerationError(ref err) => Some(err),
            SerializationError::MapGenerationError(ref err) => Some(err)
        }
    }
}

/// Defines the serialization trait for serialization and deserialization.
pub trait Serialization {

    /// Generates the JSON object for serialization.
    fn build_value(& self) -> Value;
    /// Generates a JSON serialization as pretty string.
    fn pretty_serialize(& self) -> Result<String, SerializationError> {
        let value = self.build_value();
        match serde_json::to_string_pretty(& value) {
            Ok(s) => Ok(s),
            Err(err) => Err(SerializationError::StringGenerationError(err))
        }
    }
    /// Generates a deserialized instance from the specified JSON serialization string.
    fn deserialize(s: & str) -> Result<Self, SerializationError> where Self: Sized {
        let val : Value = match serde_json::from_str(s) {
            Ok(v) => v,
            Err(err) => return Err(SerializationError::MapGenerationError(err))
        };
        Ok(Self::build_instance(val)?)
    }
    /// Generates a deserialized instance from the specified JSON object.
    fn build_instance(v: Value) -> Result<Self, SerializationError> where Self: Sized;
}
