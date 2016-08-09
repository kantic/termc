extern crate num;

use num::complex::Complex;
use std::fmt;

/// Defines the number type of the result of a mathematical expression.
#[derive(Clone, PartialEq)]
pub enum MathResultType {
    Real,
    Complex
}

/// Defines the result of a mathematical expression.
/// The result can be a real or a complex number.
#[derive(Clone)]
pub struct MathResult {
    pub result_type: MathResultType,
    pub value: Complex<f64>
}

impl MathResult {
    pub fn new(t: MathResultType, val: Complex<f64>) -> MathResult {
        MathResult {result_type: t, value: val}
    }
}

impl fmt::Display for MathResult {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        match self.result_type {
            MathResultType::Real => write!(f, "{}", self.value.re),
            MathResultType::Complex => write!(f, "{}", self.value)
        }
    }
}

impl From<Complex<f64>> for MathResult {
    /// Converts a complex number into a MathResult.
    fn from(cmplx: Complex<f64>) -> Self {
        MathResult {result_type: MathResultType::Complex, value: cmplx}
    }
}

impl<'a> From<&'a Complex<f64>> for MathResult {
    /// Converts a complex number reference into a MathResult.
    fn from(cmplx: &'a Complex<f64>) -> Self {
        MathResult {result_type: MathResultType::Complex, value: cmplx.clone()}
    }
}

impl From<(f64, f64)> for MathResult {
    /// Converts a tuple of two floats into a MathResult (complex type).
    fn from(tpl: (f64, f64)) -> Self {
        MathResult {result_type: MathResultType::Complex, value: Complex::new(tpl.0, tpl.1)}
    }
}

impl<'a> From<&'a (f64, f64)> for MathResult {
    /// Converts a tuple reference of two floats into a MathResult (complex type).
    fn from(tpl: &'a (f64, f64)) -> Self {
        MathResult {result_type: MathResultType::Complex, value: Complex::new(tpl.0, tpl.1)}
    }
}

impl From<f64> for MathResult {
    /// Converts a real number into a MathResult.
    fn from(real: f64) -> Self {
        MathResult {result_type: MathResultType::Real, value: Complex::from(real)}
    }
}

impl<'a> From<&'a f64> for MathResult {
    /// Converts a real number reference into a MathResult.
    fn from(real: & f64) -> Self {
        MathResult {result_type: MathResultType::Real, value: Complex::from(real.clone())}
    }
}