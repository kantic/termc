
extern crate num;

use std::f64;
use std::collections::{HashMap, HashSet};
use num::complex::Complex;
use math_result::{MathResult, MathResultType};

/// Defines the sets of numbers.
#[derive(Clone, PartialEq, Debug)]
pub enum NumberType {
    Real,
    Complex
}

/// Defines the types of supported operations.
#[derive(Clone, PartialEq)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// Defines the types of supported built-in functions.
#[derive(Clone, PartialEq)]
pub enum FunctionType {
    Cos,
    Sin,
    Tan,
    Cot,
    Exp,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Ln,
    Pow,
    Root,
    ArcCos,
    ArcSin,
    ArcTan,
    ArcCot,
    ArcCosh,
    ArcSinh,
    ArcTanh
}

/// Defines the mathematical context.
pub struct MathContext {
    /// Map of supported operations (operation type and precedence).
    operations: HashMap<String, (OperationType, u32)>,
    /// Set of symbols representing numbers.
    number_symbols: HashSet<char>,
    /// Set of symbols representing words.
    literals : HashSet<char>,
    /// Set of functions (function type and number of arguments).
    functions: HashMap<String, (FunctionType, u32)>,
    /// Map of supported constants (constant representation and value).
    constants : HashMap<String, MathResult>,
    /// Set of punctuation symbols.
    punctuation : HashSet<char>
}

impl<'a> MathContext {

    /// Creates a new instance of type MathContext.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// ```
    pub fn new() -> MathContext {
        let number_symbols: HashSet<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
            .into_iter().collect();

        // all literal symbols with which function names or constant names can start with
        // e.g. "pi" or "c0", but now allowed is starting with a number like "0c"
        let literals: HashSet<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
        'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C',
        'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z', '_'].into_iter().collect();

        // define the operation types associated with their string representation
        let mut operations: HashMap<String, (OperationType, u32)> = HashMap::new();
        operations.insert(String::from("+"), (OperationType::Add, 1));
        operations.insert(String::from("-"), (OperationType::Sub, 1));
        operations.insert(String::from("*"), (OperationType::Mul, 2));
        operations.insert(String::from("/"), (OperationType::Div, 2));
        operations.insert(String::from("^"), (OperationType::Pow, 3));

        // defines functions types with associated with their string representation
        let mut functions: HashMap<String, (FunctionType, u32)> = HashMap::new();
        functions.insert(String::from("cos"), (FunctionType::Cos, 1));
        functions.insert(String::from("sin"), (FunctionType::Sin, 1));
        functions.insert(String::from("tan"), (FunctionType::Tan, 1));
        functions.insert(String::from("cot"), (FunctionType::Cot, 1));

        functions.insert(String::from("cosh"), (FunctionType::Cosh, 1));
        functions.insert(String::from("sinh"), (FunctionType::Sinh, 1));
        functions.insert(String::from("tanh"), (FunctionType::Tanh, 1));

        functions.insert(String::from("arccos"), (FunctionType::ArcCos, 1));
        functions.insert(String::from("acos"), (FunctionType::ArcCos, 1));
        functions.insert(String::from("arcsin"), (FunctionType::ArcSin, 1));
        functions.insert(String::from("asin"), (FunctionType::ArcSin, 1));
        functions.insert(String::from("arctan"), (FunctionType::ArcTan, 1));
        functions.insert(String::from("atan"), (FunctionType::ArcTan, 1));
        functions.insert(String::from("arccot"), (FunctionType::ArcCot, 1));
        functions.insert(String::from("acot"), (FunctionType::ArcCot, 1));

        functions.insert(String::from("acosh"), (FunctionType::ArcCosh, 1));
        functions.insert(String::from("arccosh"), (FunctionType::ArcCosh, 1));
        functions.insert(String::from("asinh"), (FunctionType::ArcSinh, 1));
        functions.insert(String::from("arcsinh"), (FunctionType::ArcSinh, 1));
        functions.insert(String::from("atanh"), (FunctionType::ArcTanh, 1));
        functions.insert(String::from("arctanh"), (FunctionType::ArcTanh, 1));

        functions.insert(String::from("exp"), (FunctionType::Exp, 1));
        functions.insert(String::from("sqrt"), (FunctionType::Sqrt, 1));
        functions.insert(String::from("ln"), (FunctionType::Ln, 1));
        functions.insert(String::from("pow"), (FunctionType::Pow, 2));
        functions.insert(String::from("root"), (FunctionType::Root, 2));

        // defines constants
        let mut constants: HashMap<String, MathResult> = HashMap::new();
        constants.insert(String::from("pi"), MathResult::from(f64::consts::PI));
        constants.insert(String::from("e"), MathResult::from(f64::consts::E));
        constants.insert(String::from("i"), MathResult::from(Complex::i()));  // the imaginary unit

        let mut punctuation: HashSet<char> = HashSet::new();
        punctuation.insert('(');
        punctuation.insert(')');
        punctuation.insert(',');

        MathContext {
            operations: operations, number_symbols: number_symbols, literals: literals,
            functions: functions, constants: constants, punctuation: punctuation
        }
    }

    /// Checks whether the specified string is an operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_op = context.is_operation("+");
    /// assert!(is_op == true);
    /// ```
    pub fn is_operation(&self, s: &'a str) -> bool {
        self.operations.contains_key(s)
    }

    /// Checks whether the specified string is an unary operation.
    /// An unary operation is an operation that may take only one operand, e.g. "-3", where the
    /// "-" has only one operand "3".
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_op = context.is_unary_operation("-");
    /// assert!(is_op == true);
    /// let is_op = context.is_unary_operation("*");
    /// assert!(is_op == false);
    /// ```
    pub fn is_unary_operation(&self, s: &'a str) -> bool {
        match self.get_operation_type(s) {
            Some(x) => {
                if x == OperationType::Add || x == OperationType::Sub {
                    true
                }
                else {
                    false
                }
            },
            None => false
        }
    }

    /// Checks whether the specified string is a function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_func = context.is_function("cos");
    /// assert!(is_func == true);
    /// ```
    pub fn is_function(&self, s: &'a str) -> bool {
        self.functions.contains_key(s)
    }

    /// Checks whether the specified character is a number symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_num = context.is_number_symbol(& '3');
    /// assert!(is_num == true);
    /// ```
    pub fn is_number_symbol(&self, c: & char) -> bool {
        self.number_symbols.contains(c)
    }

    /// Checks whether the specified character is a literal symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_literal = context.is_literal_symbol(& 'f');
    /// assert!(is_literal == true);
    /// ```
    pub fn is_literal_symbol(&self, c: & char) -> bool {
        self.literals.contains(c)
    }

    /// Check whether the specified string is a constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_constant = context.is_constant("pi");
    /// assert!(is_constant == true);
    /// ```
    pub fn is_constant(&self, s: &'a str) -> bool {
        self.constants.contains_key(s)
    }

    /// Checks whether the specified character is a punctuation symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_punc = context.is_punctuation_symbol(& '(');
    /// assert!(is_punc == true);
    /// ```
    pub fn is_punctuation_symbol(&self, c: & char) -> bool {
        self.punctuation.contains(c)
    }

    /// Returns the value of the specified constant.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResultType;
    /// use std::f64;
    ///
    /// fn main() {
    ///     let context = MathContext::new();
    ///
    ///     let const_val = context.get_constant_value("pi");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == MathResultType::Real);
    ///     assert!(const_val.value.re - f64::consts::PI < 10e-10);
    ///
    ///     let const_val = context.get_constant_value("e");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == MathResultType::Real);
    ///     assert!(const_val.value.re - f64::consts::E < 10e-10);
    ///
    ///     let const_val = context.get_constant_value("i");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == MathResultType::Complex);
    ///     assert!(const_val.value.re < 10e-10);
    ///     assert!(const_val.value.im - 1.0 < 10e-10);
    /// }
    /// ```
    pub fn get_constant_value(&self, s: & str) -> Option<MathResult> {
        self.constants.get(s).cloned()
    }

    /// Gets the operation type of the specified operation string.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::{MathContext, OperationType};
    ///
    /// let context = MathContext::new();
    /// let op_type = context.get_operation_type("+");
    /// assert!(op_type == Some(OperationType::Add));
    /// ```
    pub fn get_operation_type(&self, s: &'a str) -> Option<OperationType> {
        match self.operations.get(s) {
            Some(x) => Some(x.0.clone()),
            None => None
        }
    }

    /// Returns the precedence of the specified operation string.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let op_prec = context.get_operation_precedence("+");
    /// assert!(op_prec == Some(1 as u32));
    /// ```
    pub fn get_operation_precedence(&self, s: &'a str) -> Option<u32> {
        match self.operations.get(s) {
            Some(x) => Some(x.1),
            None => None
        }
    }

    /// Returns the function type of the specified function string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::{MathContext, FunctionType};
    ///
    /// let context = MathContext::new();
    /// let func_type = context.get_function_type("cosh");
    /// assert!(func_type == Some(FunctionType::Cosh));
    /// ```
    pub fn get_function_type(&self, s: &'a str) -> Option<FunctionType> {
        match self.functions.get(s) {
            Some(x) => Some(x.0.clone()),
            None => None
        }
    }

    /// Returns the number of arguments for the specified function
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let n_args = context.get_function_arg_num("pow");
    /// assert!(n_args == Some(2));
    /// ```
    pub fn get_function_arg_num(&self, s: &'a str) -> Option<u32> {
        match self.functions.get(s) {
            Some(ref x) => Some(x.1),
            None => None
        }
    }

    /// Implements the mathematical "+" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0);
    /// let rhs = MathResult::from(4.0);
    /// assert!(MathContext::operation_add(& lhs, & rhs).value.re - 9.0 < 10e-10);
    /// ```
    pub fn operation_add(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        MathResult::new(t, lhs.value + rhs.value)
    }

    /// Implements the mathematical "-" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0);
    /// let rhs = MathResult::from(4.0);
    /// assert!(MathContext::operation_sub(& lhs, & rhs).value.re - 1.0 < 10e-10);
    /// ```
    pub fn operation_sub(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        MathResult::new(t, lhs.value - rhs.value)
    }

    /// Implements the mathematical "*" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0);
    /// let rhs = MathResult::from(4.0);
    /// assert!(MathContext::operation_mul(& lhs, & rhs).value.re - 20.0 < 10e-10);
    /// ```
    pub fn operation_mul(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        MathResult::new(t, lhs.value * rhs.value)
    }

    /// Implements the mathematical "/" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0);
    /// let rhs = MathResult::from(4.0);
    /// assert!(MathContext::operation_div(& lhs, & rhs).value.re - 5.0/4.0 < 10e-10);
    /// ```
    pub fn operation_div(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        MathResult::new(t, lhs.value / rhs.value)
    }

    /// Implements the mathematical "^" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0);
    /// let rhs = MathResult::from(4.0);
    /// assert!(MathContext::operation_pow(& lhs, & rhs).value.re - 625.0 < 10e-10);
    /// ```
    pub fn operation_pow(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        match lhs.result_type {
            MathResultType::Real => {
                match rhs.result_type {
                    MathResultType::Real => {
                        // ordinary pow, e.g. "a^b"
                        MathResult::new(t, Complex::from(lhs.value.re.powf(rhs.value.re)))
                    },

                    MathResultType::Complex => {
                        // exponent is complex, e.g. "a^(b+ci)" = "exp(ln(a) * (b+ci))"
                        MathResult::new(t, (rhs.value * lhs.value.re.ln()).exp())
                    }
                }
            },

            MathResultType::Complex =>  {
                // base is complex, e.g. "(a+bi)^c" = "exp(ln(a+bi) * c)" or
                // base and exponent are complex, e.g. "(a+bi)^(c+di)" = "exp(ln(a+bi) * (c+di))"
                MathResult::new(t, (lhs.value.ln() * rhs.value).exp())
            }
        }
    }

    pub fn function_cos(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cos())
    }

    pub fn function_sin(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.sin())
    }

    pub fn function_tan(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.tan())
    }

    pub fn function_cot(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cos() / arg.value.sin())
    }

    pub fn function_arccos(arg: & MathResult) -> MathResult {
        let mut t = arg.result_type.clone();
        if t == MathResultType::Real {
            if !(arg.value.re <= 1.0 && arg.value.re >= -1.0) {
                t = MathResultType::Complex;
            }
        }
        MathResult::new(t, arg.value.acos())
    }

    pub fn function_arcsin(arg: & MathResult) -> MathResult {
        let mut t = arg.result_type.clone();
        if t == MathResultType::Real {
            if !(arg.value.re <= 1.0 && arg.value.re >= -1.0) {
                t = MathResultType::Complex;
            }
        }
        MathResult::new(t, arg.value.asin())
    }

    pub fn function_arctan(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.atan())
    }

    pub fn function_arccot(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), f64::consts::FRAC_PI_2 - arg.value.atan())
    }

    pub fn function_cosh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cosh())
    }

    pub fn function_sinh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.sinh())
    }

    pub fn function_tanh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.tanh())
    }

    pub fn function_arccosh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.acosh())
    }

    pub fn function_arcsinh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.asinh())
    }

    pub fn function_arctanh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.atanh())
    }

    pub fn function_exp(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.exp())
    }

    pub fn function_ln(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.ln())
    }

    pub fn function_sqrt(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.sqrt())
    }

    /// Returns the result type for a mathematical expression with the given operands.
    /// The result type is complex, if any of the specified operands is complex.
    /// Otherwise, the result type is real.
    fn get_result_type(args: &Vec<& MathResult>) -> MathResultType {
        for arg in args {
            if arg.result_type == MathResultType::Complex {
                return MathResultType::Complex;
            }
        }

        MathResultType::Real
    }
}