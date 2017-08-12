use std::fmt;
use num::complex::Complex;
use token::NumberType;
use f64formatter::F64Formatter;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::{SerializeStruct};
use serde::de;

/// Defines the result of a mathematical expression.
/// The result can be a real or a complex number and thus, be only numerical.
#[derive(Clone)]
pub struct MathResult {
    pub result_type: NumberType,
    pub value: Complex<f64>
}

impl Serialize for MathResult {

    /// Serializes the MathResult struct.
    fn serialize<S>(&self, serializer: S) -> Result<(S::Ok), S::Error> where
        S: Serializer
    {
        let mut struc = serializer.serialize_struct("MathResult", 3)?;
        struc.serialize_field("result_type", &self.result_type)?;
        struc.serialize_field("re", &self.value.re)?;
        struc.serialize_field("im", &self.value.im)?;
        struc.end()
    }
}

impl Deserialize for MathResult
{
    /// Deserializes the MathResult struct.
    /// See https://serde.rs/impl-deserialize.html for reference and examples.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer,
    {

        enum Field {ResultType, Re, Im};

        impl Deserialize for Field
        {

            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
                D: Deserializer,
            {
                struct FieldVisitor;

                impl de::Visitor for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`result_type (NumberType)`, `re (f64)` or `im (f64)`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where E: de::Error
                    {
                        match value {
                            "result_type" => Ok(Field::ResultType),
                            "re" => Ok(Field::Re),
                            "im" => Ok(Field::Im),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct MathResultVisitor;

        impl de::Visitor for MathResultVisitor {
            type Value = MathResult;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MathResult")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<MathResult, V::Error>
                where V: de::MapVisitor
            {
                let mut result_type = None;
                let mut re = None;
                let mut im = None;
                while let Some(key) = visitor.visit_key()? {
                    match key {
                        Field::ResultType => {
                            if result_type.is_some() {
                                return Err(de::Error::duplicate_field("result_type"));
                            }
                            result_type = Some(visitor.visit_value()?);
                        }
                        Field::Re => {
                            if re.is_some() {
                                return Err(de::Error::duplicate_field("re"));
                            }
                            re = Some(visitor.visit_value()?);
                        }
                        Field::Im => {
                            if im.is_some() {
                                return Err(de::Error::duplicate_field("im"));
                            }
                            im = Some(visitor.visit_value()?);
                        }
                    }
                }
                let result_type = match result_type {
                    Some(result_type) => result_type,
                    None => return Err(de::Error::missing_field("result_type")),
                };
                let re = match re {
                    Some(re) => re,
                    None => return Err(de::Error::missing_field("re")),
                };
                let im = match im {
                    Some(im) => im,
                    None => return Err(de::Error::missing_field("im")),
                };
                Ok(MathResult {result_type: result_type, value: Complex::new(re, im)})
            }
        }

        const FIELDS: &'static [&'static str] = &["result_type", "re", "im"];
        deserializer.deserialize_struct("MathResult", FIELDS, MathResultVisitor)
    }
}

impl MathResult {
    /// Creates a new instance of the MathResult struct.
    pub fn new(t: NumberType, val: Complex<f64>) -> MathResult {
        MathResult {result_type: t, value: val}
    }
}

impl fmt::Display for MathResult {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        match self.result_type {
            NumberType::Real => write!(f, "{0}", self.value.re),
            NumberType::Complex => write!(f, "{0}", self.value)
        }
    }
}

macro_rules! write_result {
    ($f:ident, $t:expr, $val:expr) => {{
    // f: fmt::Formatter
    // t: format type (e.g "b" for binary)
    // val: MathResult instance to print
        let tmp : Complex<i64> = Complex::new($val.value.re as i64, $val.value.im as i64);
        match $val.result_type {
            NumberType::Real => write!($f, concat!("{0:#", $t, "}"), tmp.re),
            NumberType::Complex => write!($f, concat!("{0:#", $t, "}"), tmp)
        }
    }}
}

macro_rules! fmt_impl {
    ($f:ident, $obj:ident, $fmt_type:tt) => {{
    // f: the fmt::Formatter instance
    // obj: the MathResult instance to be formatted
    // fmt_type: the formatting type (e.g. 'b' (binary), 'o' (octal) or 'x' (hexadecimal))

        match $obj.result_type {
            NumberType::Real => {
                write!($f, concat!("{0:#" ,$fmt_type, "}"), F64Formatter($obj.value.re))
            },
            NumberType::Complex => {
                let tmp : Complex<F64Formatter> = Complex::new(F64Formatter($obj.value.re), F64Formatter($obj.value.im));
                write!($f, concat!("{0:#", $fmt_type, "}"), tmp)
            }
        }
    }}
}

impl fmt::Binary for MathResult {
    /// Implements the formatted binary output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        fmt_impl!(f, self, "b")
    }
}

impl fmt::LowerHex for MathResult {
    /// Implements the formatted lower hexadecimal output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        fmt_impl!(f, self, "x")
    }
}

impl fmt::UpperHex for MathResult {
    /// Implements the formatted upper hexadecimal output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        fmt_impl!(f, self, "X")
    }
}

impl fmt::Octal for MathResult {
    /// Implements the formatted octal output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        fmt_impl!(f, self, "o")
    }
}

impl fmt::LowerExp for MathResult {
    /// Implements the formatted lower exponential output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match self.result_type {
            NumberType::Real => write!(f, "{0:#e}", self.value.re),
            NumberType::Complex => write!(f, "{0:#e}", self.value)
        }
    }
}

impl fmt::UpperExp for MathResult {
    /// Implements the formatted upper exponential output for MathResult.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match self.result_type {
            NumberType::Real => write!(f, "{0:#E}", self.value.re),
            NumberType::Complex => write!(f, "{0:#E}", self.value)
        }
    }
}

impl From<Complex<f64>> for MathResult {
    /// Converts a complex number into a MathResult.
    fn from(cmplx: Complex<f64>) -> Self {
        MathResult {result_type: NumberType::Complex, value: Complex::from(cmplx)}
    }
}

impl<'a> From<&'a Complex<f64>> for MathResult {
    /// Converts a complex number reference into a MathResult.
    fn from(cmplx: &'a Complex<f64>) -> Self {
        if cmplx.im == 0.0_f64 {
            MathResult {result_type: NumberType::Real, value: Complex::from(cmplx.re)}
        }
        else {
            MathResult {result_type: NumberType::Complex, value: Complex::from(cmplx.clone())}
        }
    }
}

impl From<(f64, f64)> for MathResult {
    /// Converts a tuple of two floats into a MathResult (complex type).
    fn from(tpl: (f64, f64)) -> Self {
        if tpl.1 == 0.0_f64 {
            MathResult {result_type: NumberType::Real, value: Complex::from(tpl.0)}
        }
        else {
            MathResult {result_type: NumberType::Complex, value: Complex::new(tpl.0, tpl.1)}
        }
    }
}

impl<'a> From<&'a (f64, f64)> for MathResult {
    /// Converts a tuple reference of two floats into a MathResult (complex type).
    fn from(tpl: &'a (f64, f64)) -> Self {
        if tpl.1 == 0.0_f64 {
            MathResult {result_type: NumberType::Real, value: Complex::from(tpl.0)}
        }
        else {
            MathResult {result_type: NumberType::Complex, value: Complex::new(tpl.0, tpl.1)}
        }
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
