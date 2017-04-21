extern crate termc_model;
extern crate num;

use std::char;
use std::error::Error;
use std::fmt;
use std::mem::transmute;
use num::complex::Complex;
#[cfg(unix)]
use unix::TerminalHandle;
#[cfg(windows)]
use windows::TerminalHandle;
use termc_model::math_result::MathResult;
use termc_model::token::NumberType;

/// Defines the prompt.
static PROMPT : &'static str = ">>> ";
/// Defines the length of the prompt (number of characters)
static PROMPT_LEN : u16 = 4;

/// Defines the answer prefix
static ANS_PREFIX : &'static str = "ans = ";

/// Defines the ASCII number for the "Enter" key.
const ENTER_KEY : char = '\x0A';

#[derive(Clone)]
pub enum FormatType {
    Dec,
    Oct,
    Hex,
    Bin,
    IEEE754,
    Exp,
    Unknown
}

impl<'a> From<&'a str> for FormatType {
    fn from(s: &'a str) -> FormatType {
        if s == "bin" {
            FormatType::Bin
        }
        else if s == "oct" {
            FormatType::Oct
        }
        else if s == "hex" {
            FormatType::Hex
        }
        else if s == "ieee754" {
            FormatType::IEEE754
        }
        else if s == "exp" {
            FormatType::Exp
        }
        else if s == "dec" {
            FormatType::Dec
        }
        else {
            FormatType::Unknown
        }
    }
}

pub enum TerminalMode {
    Interactive,
    Call
}

pub trait FormatIEEE754 {
    fn ieee754_fmt(&self) -> String;
}

impl FormatIEEE754 for f64 {
    fn ieee754_fmt(&self) -> String {
        let pattern : u64 = unsafe {transmute::<f64, u64>(*self)};
        format!("{0:#b}", pattern)
    }
}

impl FormatIEEE754 for f32 {
    fn ieee754_fmt(&self) -> String {
        let pattern : u32 = unsafe {transmute::<f32, u32>(*self)};
        format!("{0:#b}", pattern)
    }
}

impl FormatIEEE754 for Complex<f32> {
    fn ieee754_fmt(&self) -> String {
        let new_re : u32 = unsafe {transmute::<f32, u32>(self.re)};
        let new_im : u32 = unsafe {transmute::<f32, u32>(self.im)};
        let tmp : Complex<u32> = Complex::new(new_re, new_im);
        format!("{0:#b}", tmp)
    }
}

impl FormatIEEE754 for Complex<f64> {
    fn ieee754_fmt(&self) -> String {
        let new_re : u64 = unsafe {transmute::<f64, u64>(self.re)};
        let new_im : u64 = unsafe {transmute::<f64, u64>(self.im)};
        let tmp : Complex<u64> = Complex::new(new_re, new_im);
        format!("{0:#b}", tmp)
    }
}

impl FormatIEEE754 for MathResult {
    fn ieee754_fmt(&self) -> String {
        match self.result_type {
            NumberType::Real => format!("{0}", self.value.re.ieee754_fmt()),
            NumberType::Complex => {
                format!("{0}", self.value.ieee754_fmt())
            }
        }
    }
}

#[macro_export]
macro_rules! format_result {
    ($typ:expr, $res:expr) => {{
        match $typ {
            FormatType::Dec | FormatType::Unknown => format!("{0}", $res),
            FormatType::Bin => format!("{0:#b}", $res),
            FormatType::Hex => format!("{0:#x}", $res),
            FormatType::Oct => format!("{0:#o}", $res),
            FormatType::Exp => format!("{0:E}", $res),
            FormatType::IEEE754 => format!("{0}", $res.ieee754_fmt()),
        }
    }};
    ($typ:expr, $res:ident, $ans_prefix:ident) => {{
        match $typ {
            FormatType::Dec | FormatType::Unknown => format!("{0}{1}", $ans_prefix, $res),
            FormatType::Bin => format!("{0}{1:#b}", $ans_prefix, $res),
            FormatType::Hex => format!("{0}{1:#x}", $ans_prefix, $res),
            FormatType::Oct => format!("{0}{1:#o}", $ans_prefix, $res),
            FormatType::Exp => format!("{0}{1:E}", $ans_prefix, $res),
            FormatType::IEEE754 => format!("{0}{1}", $ans_prefix, $res.ieee754_fmt())
        }
    }}
}

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

/// Defines the interface with which the computation results and errors can be printed on the
/// terminal and with which the user input is fetched.
pub trait TerminalUI {
    /// Creates a new TerminalUI trait object instance.
    fn new(mode: TerminalMode) -> Self;
    /// Initializes the terminal.
    fn init(& mut self);
    /// Finalize the terminal.
    fn end(& mut self);
    /// Gets the user input and manages the manipulation of the terminal.
    fn get_user_input(& mut self) -> String;
    /// Prints the specified result on the terminal.
    fn print_result<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(& mut self, result: Option<&T>);
    /// Prints the specified results on the terminal, separated with ';'.
    fn print_results<T: fmt::Display + fmt::Binary + fmt::LowerHex + fmt::UpperHex + fmt::Octal +
                        FormatIEEE754 + fmt::LowerExp + fmt::UpperExp>(& mut self, results: &Vec<T>);
    /// Prints the specified error on the terminal.
    fn print_error<T: Error>(& mut self, err: T);
    /// Prints the specified string.
    fn print_str(& mut self, s: &str);
    /// Prints a newline.
    fn print_newline(& mut self);
    fn set_format_type(& mut self, t: FormatType);
}

/// Creates a new terminal handle that implements the TerminalUI trait.
pub fn create_terminal_handle(mode: TerminalMode) -> TerminalHandle {
    TerminalUI::new(mode)
}