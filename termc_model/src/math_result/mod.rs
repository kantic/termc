extern crate num;
extern crate serde_json;

use num::complex::Complex;
use serde_json::{Value, Map};
use std::fmt;
use token::NumberType;
use serialization::{Serialization, SerializationError};

/// Defines the result of a mathematical expression.
/// The result can be a real or a complex number and thus, be only numerical.
#[derive(Clone)]
pub struct MathResult {
    pub result_type: NumberType,
    pub value: Complex<f64>
}

/// Defines the serialization trait for serialization and deserialization.
impl Serialization for MathResult {

    /// Generates the JSON object for serialization.
    fn build_value(& self) -> Value {
        let mut m : Map<String, Value> = Map::new();
        m.insert(String::from("resultType"), self.result_type.build_value());
        let mut value_val : Map<String, Value> = Map::new();
        value_val.insert(String::from("re"), Value::F64(self.value.re));
        value_val.insert(String::from("im"), Value::F64(self.value.im));
        m.insert(String::from("value"), Value::Object(value_val));

        Value::Object(m)
    }

    /// Generates a deserialized instance from the specified JSON object.
    fn build_instance(v: Value) -> Result<MathResult, SerializationError> {
        let mut m = match v {
            Value::Object(map) => map,
            _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
        };

        let result_val = match m.remove("resultType") {
            Some(v) => try!(NumberType::build_instance(v)),
            None => return Err(SerializationError::MissingValueError(String::from("MathResult: resultType")))
        };

        let value_val = match m.remove("value") {
            Some(v) => {
                match v {
                    Value::Object(mut map) => {
                        let re_val = match map.remove("re") {
                            Some(re) => {
                                match re {
                                    Value::F64(f) => f,
                                    _ => return Err(SerializationError::ValueTypeError(String::from("f64")))
                                }
                            },
                            None => return Err(SerializationError::MissingValueError(String::from("ComplexNumber: re")))
                        };

                        let im_val = match map.remove("im") {
                            Some(im) => {
                                match im {
                                    Value::F64(f) => f,
                                    _ => return Err(SerializationError::ValueTypeError(String::from("f64")))
                                }
                            },
                            None => return Err(SerializationError::MissingValueError(String::from("ComplexNumber: im")))
                        };

                        Complex {re: re_val, im: im_val}
                    },

                    _ => return Err(SerializationError::ValueTypeError(String::from("Object")))
                }
            },

            None => return Err(SerializationError::MissingValueError(String::from("MathResult: value")))
        };

        Ok(MathResult {result_type: result_val, value: value_val})
    }
}

impl MathResult {
    pub fn new(t: NumberType, val: Complex<f64>) -> MathResult {
        MathResult {result_type: t, value: val}
    }
}

impl fmt::Display for MathResult {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        match self.result_type {
            NumberType::Real => write!(f, "{}", self.value.re),
            NumberType::Complex => write!(f, "{}", self.value)
        }
    }
}

impl From<Complex<f64>> for MathResult {
    /// Converts a complex number into a MathResult.
    fn from(cmplx: Complex<f64>) -> Self {
        MathResult {result_type: NumberType::Complex, value: cmplx}
    }
}

impl<'a> From<&'a Complex<f64>> for MathResult {
    /// Converts a complex number reference into a MathResult.
    fn from(cmplx: &'a Complex<f64>) -> Self {
        MathResult {result_type: NumberType::Complex, value: cmplx.clone()}
    }
}

impl From<(f64, f64)> for MathResult {
    /// Converts a tuple of two floats into a MathResult (complex type).
    fn from(tpl: (f64, f64)) -> Self {
        MathResult {result_type: NumberType::Complex, value: Complex::new(tpl.0, tpl.1)}
    }
}

impl<'a> From<&'a (f64, f64)> for MathResult {
    /// Converts a tuple reference of two floats into a MathResult (complex type).
    fn from(tpl: &'a (f64, f64)) -> Self {
        MathResult {result_type: NumberType::Complex, value: Complex::new(tpl.0, tpl.1)}
    }
}

impl From<f64> for MathResult {
    /// Converts a real number into a MathResult.
    fn from(real: f64) -> Self {
        MathResult {result_type: NumberType::Real, value: Complex::from(real)}
    }
}

impl<'a> From<&'a f64> for MathResult {
    /// Converts a real number reference into a MathResult.
    fn from(real: & f64) -> Self {
        MathResult {result_type: NumberType::Real, value: Complex::from(real.clone())}
    }
}