use std::f64;
use std::collections::{HashMap, HashSet};
use num::complex::Complex;
use token::{Token, TokenType, SymbolicTokenType};
use token::NumberType;
use math_result::MathResult;
use tree::TreeNode;

/// Defines the types of supported operations.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Assign
}

/// Defines the types of supported built-in functions.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionType {
    Cos,
    Sin,
    Tan,
    Cot,
    Exp,
    Sinh,
    Cosh,
    Tanh,
    Coth,
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
    ArcTanh,
    ArcCoth,
    Im,
    Re,
    UserFunction
}

/// Defines the mathematical context.
#[derive(Serialize, Deserialize)]
pub struct MathContext {
    /// Map of supported operations (operation type and precedence).
    #[serde(skip_serializing, skip_deserializing)]
    operations: HashMap<String, (OperationType, u32)>,

    /// Set of symbols representing numbers.
    #[serde(skip_serializing, skip_deserializing)]
    number_symbols: HashSet<char>,

    /// Set of symbols representing words.
    #[serde(skip_serializing, skip_deserializing)]
    literals : HashSet<char>,

    /// Set of functions (function type and number of arguments).
    #[serde(skip_serializing, skip_deserializing)]
    functions: HashMap<String, (FunctionType, u32)>,

    /// Set of user defined functions (the function expression tree and it's variables).
    user_functions: HashMap<String, (TreeNode<Token>, Vec<String>)>,

    /// The user inputs that define user functions.
    user_function_inputs: HashMap<String, String>,

    /// Map of built-in constants (constant representation and value).
    #[serde(skip_serializing, skip_deserializing)]
    constants : HashMap<String, MathResult>,

    /// Map of user defined constants (constant representation and value).
    user_constants: HashMap<String, MathResult>,

    /// Set of punctuation symbols.
    #[serde(skip_serializing, skip_deserializing)]
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

        let (number_symbols, literals, operations, functions, constants,
            punctuation) = MathContext::get_init_values();
        MathContext {
            operations: operations, number_symbols: number_symbols, literals: literals,
            functions: functions, user_functions: HashMap::new(), user_function_inputs: HashMap::new(),
            constants: constants, user_constants: HashMap::new(), punctuation: punctuation
        }
    }

    fn get_init_values() -> (HashSet<char>, HashSet<char>, HashMap<String, (OperationType, u32)>,
                        HashMap<String, (FunctionType, u32)>, HashMap<String, MathResult>,
                        HashSet<char>) {

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
        operations.insert(String::from("="), (OperationType::Assign, 1));
        operations.insert(String::from("+"), (OperationType::Add, 2));
        operations.insert(String::from("-"), (OperationType::Sub, 2));
        operations.insert(String::from("*"), (OperationType::Mul, 3));
        operations.insert(String::from("/"), (OperationType::Div, 3));
        operations.insert(String::from("%"), (OperationType::Mod, 3));
        operations.insert(String::from("^"), (OperationType::Pow, 4));

        // defines functions types with associated with their string representation
        let mut functions: HashMap<String, (FunctionType, u32)> = HashMap::new();
        functions.insert(String::from("cos"), (FunctionType::Cos, 1));
        functions.insert(String::from("sin"), (FunctionType::Sin, 1));
        functions.insert(String::from("tan"), (FunctionType::Tan, 1));
        functions.insert(String::from("cot"), (FunctionType::Cot, 1));

        functions.insert(String::from("cosh"), (FunctionType::Cosh, 1));
        functions.insert(String::from("sinh"), (FunctionType::Sinh, 1));
        functions.insert(String::from("tanh"), (FunctionType::Tanh, 1));
        functions.insert(String::from("coth"), (FunctionType::Coth, 1));

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
        functions.insert(String::from("arccoth"), (FunctionType::ArcCoth, 1));

        functions.insert(String::from("exp"), (FunctionType::Exp, 1));
        functions.insert(String::from("sqrt"), (FunctionType::Sqrt, 1));
        functions.insert(String::from("ln"), (FunctionType::Ln, 1));
        functions.insert(String::from("im"), (FunctionType::Im, 1));
        functions.insert(String::from("re"), (FunctionType::Re, 1));

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

        (number_symbols, literals, operations, functions, constants, punctuation)
    }

    pub fn initialize(& mut self) {
        let (number_symbols, literals, operations, functions,
            constants, punctuation) = MathContext::get_init_values();

        self.number_symbols = number_symbols;
        self.literals = literals;
        self.operations = operations;
        self.functions = functions;
        self.constants = constants;
        self.punctuation = punctuation;
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
    pub fn is_operation(&self, s: & str) -> bool {
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
    pub fn is_unary_operation(&self, s: & str) -> bool {
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
    pub fn is_function(& self, s: & str) -> bool {
        self.functions.contains_key(s) || self.user_functions.contains_key(s)
    }

    /// Checks whether the specified string is a built-in function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_built_in_func = context.is_built_in_function("arctan");
    /// assert!(is_built_in_func == true);
    /// ```
    pub fn is_built_in_function(& self, s: & str) -> bool {
        self.functions.contains_key(s)
    }

    /// Checks whether the specified string is a user defined function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_built_in_func = context.is_user_function("arctan");
    /// assert!(is_built_in_func == false);
    /// ```
    pub fn is_user_function(& self, s: & str) -> bool {
        self.user_functions.contains_key(s)
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
    pub fn is_number_symbol(& self, c: & char) -> bool {
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
    pub fn is_literal_symbol(& self, c: & char) -> bool {
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
    pub fn is_constant(& self, s: & str) -> bool {
        self.constants.contains_key(s) || self.user_constants.contains_key(s)
    }

    /// Checks whether the specified string is a built-in constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_built_in_const = context.is_built_in_constant("pi");
    /// assert!(is_built_in_const == true);
    /// ```
    pub fn is_built_in_constant(& self, s: & str) -> bool {
        self.constants.contains_key(s)
    }

    /// Checks whether the specified string is a user defined constant.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::NumberType;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///     let is_built_in_const = context.is_user_constant("pi");
    ///     assert!(is_built_in_const == false);
    ///
    ///     context.add_user_constant("custom_constr", MathResult::new(NumberType::Real, Complex::new(4.1, 0.0)));
    ///
    ///     let is_built_in_const = context.is_user_constant("custom_constr");
    ///     assert!(is_built_in_const == true);
    /// }
    /// ```
    pub fn is_user_constant(& self, s: & str) -> bool {
        self.user_constants.contains_key(s)
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
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::NumberType;
    /// use std::f64;
    ///
    /// fn main() {
    ///     let context = MathContext::new();
    ///
    ///     let const_val = context.get_constant_value("pi");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == NumberType::Real);
    ///     assert!(const_val.value.re - f64::consts::PI < 10e-10);
    ///
    ///     let const_val = context.get_constant_value("e");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == NumberType::Real);
    ///     assert!(const_val.value.re - f64::consts::E < 10e-10);
    ///
    ///     let const_val = context.get_constant_value("i");
    ///     assert!(const_val.is_some());
    ///     let const_val = const_val.unwrap();
    ///     assert!(const_val.result_type == NumberType::Complex);
    ///     assert!(const_val.value.re < 10e-10);
    ///     assert!(const_val.value.im - 1.0 < 10e-10);
    /// }
    /// ```
    pub fn get_constant_value(&self, s: & str) -> Option<MathResult> {
        match self.constants.get(s) {
            Some(x) => Some(x.clone()),
            None => {
                self.user_constants.get(s).cloned()
            }
        }
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
    pub fn get_operation_type(&self, s: & str) -> Option<OperationType> {
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
    /// assert!(op_prec == Some(2 as u32));
    /// ```
    pub fn get_operation_precedence(& self, s: & str) -> Option<u32> {
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
    pub fn get_function_type(& self, s: & str) -> Option<FunctionType> {
        match self.functions.get(s) {
            Some(x) => Some(x.0.clone()),
            None => {
                match self.user_functions.get(s) {
                    Some(_) => Some(FunctionType::UserFunction),
                    None => None
                }
            }
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
    pub fn get_function_arg_num(& self, s: & str) -> Option<u32> {
        match self.functions.get(s) {
            Some(ref x) => Some(x.1),
            None => {
                match self.user_functions.get(s) {
                    Some(ref x) => Some(x.1.len() as u32),
                    None => None
                }
            }
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
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(4.0_f64);
    /// assert!(MathContext::operation_add(& lhs, & rhs).value.re - 9.0_f64 < 10e-10_f64);
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
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(4.0_f64);
    /// assert!(MathContext::operation_sub(& lhs, & rhs).value.re - 1.0_f64 < 10e-10_f64);
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
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(4.0_f64);
    /// assert!(MathContext::operation_mul(& lhs, & rhs).value.re - 20.0_f64 < 10e-10_f64);
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
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(4.0_f64);
    /// assert!(MathContext::operation_div(& lhs, & rhs).value.re - 5.0_f64/4.0_f64 < 10e-10_f64);
    /// ```
    pub fn operation_div(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        MathResult::new(t, lhs.value / rhs.value)
    }

    /// Implements the mathematical "/" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(3.0_f64);
    /// assert!(MathContext::operation_mod(& lhs, & rhs).value.re - 2.0 < 10e-10_f64);
    /// ```
    pub fn operation_mod(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);

        // check if the input was no float
        if MathContext::has_decimal_places(lhs.value.re)
            || MathContext::has_decimal_places(rhs.value.re) {

            MathResult::from(f64::NAN)
        }
        else {
            let lhs_i = match lhs.result_type {
                NumberType::Complex => return MathResult::from(f64::NAN),
                NumberType::Real => lhs.value.re as i64
            };
            let rhs_i = match lhs.result_type {
                NumberType::Complex => return MathResult::from(f64::NAN),
                NumberType::Real => rhs.value.re as i64
            };

            MathResult::new(t, Complex::from((lhs_i % rhs_i) as f64))
        }
    }

    /// Checks whether the specified float has decimal_places.
    fn has_decimal_places(f: f64) -> bool {
        let i = f as i64;
        f.abs() - (i.abs() as f64) > 0.0_f64
    }

    /// Implements the mathematical "^" operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let lhs = MathResult::from(5.0_f64);
    /// let rhs = MathResult::from(4.0_f64);
    /// assert!(MathContext::operation_pow(& lhs, & rhs).value.re - 625.0_f64 < 10e-10_f64);
    /// ```
    pub fn operation_pow(lhs: & MathResult, rhs: & MathResult) -> MathResult {
        let t = MathContext::get_result_type(& vec![lhs, rhs]);
        match lhs.result_type {
            NumberType::Real => {
                match rhs.result_type {
                    NumberType::Real => {
                        // ordinary pow, e.g. "a^b"
                        MathResult::new(t, Complex::from(lhs.value.re.powf(rhs.value.re)))
                    },

                    NumberType::Complex => {
                        // exponent is complex, e.g. "a^(b+ci)" = "exp(ln(a) * (b+ci))"
                        MathResult::new(t, (rhs.value * lhs.value.re.ln()).exp())
                    }
                }
            },

            NumberType::Complex =>  {
                // base is complex, e.g. "(a+bi)^c" = "exp(ln(a+bi) * c)" or
                // base and exponent are complex, e.g. "(a+bi)^(c+di)" = "exp(ln(a+bi) * (c+di))"
                MathResult::new(t, (lhs.value.ln() * rhs.value).exp())
            }
        }
    }

    /// Implements the mathematical cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(0.0_f64);
    /// assert!(MathContext::function_cos(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_cos(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cos())
    }

    /// Implements the mathematical sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use std::f64;
    ///
    /// let arg = MathResult::from(f64::consts::FRAC_PI_2);
    /// assert!(MathContext::function_sin(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_sin(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.sin())
    }

    /// Implements the mathematical tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use std::f64;
    ///
    /// let arg = MathResult::from(f64::consts::FRAC_PI_4);
    /// assert!(MathContext::function_tan(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_tan(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.tan())
    }

    /// Implements the mathematical cotangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use std::f64;
    ///
    /// let arg = MathResult::from(f64::consts::FRAC_PI_4);
    /// assert!(MathContext::function_cot(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_cot(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cos() / arg.value.sin())
    }

    /// Implements the mathematical inverse cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.cos());
    /// assert!(MathContext::function_arccos(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arccos(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if !(arg.value.re <= 1.0_f64 && arg.value.re >= -1.0_f64) {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.acos())
    }

    /// Implements the mathematical inverse sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.sin());
    /// assert!(MathContext::function_arcsin(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arcsin(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if !(arg.value.re <= 1.0_f64 && arg.value.re >= -1.0_f64) {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.asin())
    }

    /// Implements the mathematical inverse tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.tan());
    /// assert!(MathContext::function_arctan(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arctan(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.atan())
    }

    /// Implements the mathematical inverse cotangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.cos() / 1.0_f64.sin());
    /// assert!(MathContext::function_arccot(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arccot(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), f64::consts::FRAC_PI_2 - arg.value.atan())
    }

    /// Implements the mathematical hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(0.0_f64);
    /// assert!(MathContext::function_cosh(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_cosh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cosh())
    }

    /// Implements the mathematical hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(0.5_f64.sinh());
    /// assert!(MathContext::function_arctan(& arg).value.re - 0.5_f64 < 10e-10_f64);
    /// ```
    pub fn function_sinh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.sinh())
    }

    /// Implements the mathematical hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(0.7_f64.tanh());
    /// assert!(MathContext::function_arctanh(& arg).value.re - 0.7_f64 < 10e-10_f64);
    /// ```
    pub fn function_tanh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.tanh())
    }

    /// Implements the mathematical hyperbolic cotangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.cosh() / 1.0_f64.sinh());
    /// assert!(MathContext::function_arccoth(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_coth(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.cosh() / arg.value.sinh())
    }

    /// Implements the mathematical inverse hyperbolic cosine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.cosh());
    /// assert!(MathContext::function_arccosh(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arccosh(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if !(arg.value.re >= 1.0_f64) {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.acosh())
    }

    /// Implements the mathematical inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.sinh());
    /// assert!(MathContext::function_arcsinh(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arcsinh(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.asinh())
    }

    /// Implements the mathematical inverse hyperbolic tangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(1.0_f64.tanh());
    /// assert!(MathContext::function_arctanh(& arg).value.re - 1.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_arctanh(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if !(arg.value.re > -1.0_f64 && arg.value.re < 1.0_f64) {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.atanh())
    }

    /// Implements the mathematical inverse hyperbolic cotangent function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(0.5_f64.tanh());
    /// assert!(MathContext::function_arccoth(& arg).value.re - 0.549306144_f64 < 10e-10_f64);
    /// ```
    pub fn function_arccoth(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if !(arg.value.re > 1.0_f64 || arg.value.re < -1.0_f64) {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        let temp = MathResult::new(NumberType::Complex, -Complex::i() * arg.value);
        MathResult::new(t, 1.0_f64 / Complex::i() * MathContext::function_arccot(& temp).value)
    }

    /// Implements the mathematical exponential function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use std::f64;
    ///
    /// let arg = MathResult::from(2.0_f64);
    /// assert!(MathContext::function_exp(& arg).value.re - f64::consts::E * f64::consts::E < 10e-10_f64);
    /// ```
    pub fn function_exp(arg: & MathResult) -> MathResult {
        MathResult::new(arg.result_type.clone(), arg.value.exp())
    }

    /// Implements the mathematical inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(5.0_f64.exp());
    /// assert!(MathContext::function_ln(& arg).value.re - 5.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_ln(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if arg.value.re < 0.0_f64 {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.ln())
    }

    /// Implements the mathematical inverse hyperbolic sine function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from(25.0_f64);
    /// assert!(MathContext::function_sqrt(& arg).value.re - 5.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_sqrt(arg: & MathResult) -> MathResult {
        let t : NumberType = match arg.result_type {
            NumberType::Real => {
                if arg.value.re < 0.0_f64 {
                    NumberType::Complex
                }
                else {
                    NumberType::Real
                }
            },

            NumberType::Complex => NumberType::Complex
        };

        MathResult::new(t, arg.value.sqrt())
    }

    /// Implements the mathematical imaginary-part function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from((25.7, 89.224));
    /// assert!(MathContext::function_im(& arg).value.im - 89.224_f64 < 10e-10_f64);
    /// assert!(MathContext::function_im(& arg).value.re - 0.0_f64 < 10e-10_f64);
    /// ```
    pub fn function_im(arg: & MathResult) -> MathResult {
        MathResult::new(NumberType::Complex, Complex::new(0.0_f64, arg.value.im))
    }

    /// Implements the mathematical imaginary-part function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    ///
    /// let arg = MathResult::from((25.7, 89.224));
    /// assert!(MathContext::function_re(& arg).value.im - 0.0_f64 < 10e-10_f64);
    /// assert!(MathContext::function_re(& arg).value.re - 25.7_f64 < 10e-10_f64);
    /// ```
    pub fn function_re(arg: & MathResult) -> MathResult {
        MathResult::new(NumberType::Real, Complex::new(arg.value.re, 0.0_f64))
    }

    /// Returns the result type for a mathematical expression with the given operands.
    /// The result type is complex, if any of the specified operands is complex.
    /// Otherwise, the result type is real.
    fn get_result_type(args: & Vec<& MathResult>) -> NumberType {
        for arg in args {
            if arg.result_type == NumberType::Complex {
                return NumberType::Complex;
            }
        }

        NumberType::Real
    }

    /// Adds the specified user constant to the mathematical context.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::NumberType;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///     context.add_user_constant("c", MathResult::new(NumberType::Real, Complex::new(4.1, 0.0)));
    ///
    ///     let is_built_in_const = context.is_user_constant("c");
    ///     assert!(is_built_in_const == true);
    ///     let constr = context.get_constant_value("c").unwrap();
    ///     assert!(constr.value.re - 4.1 < 10e-10);
    /// }
    /// ```
    pub fn add_user_constant<S>(& mut self, repr: S, value: MathResult) where S: Into<String> {
        self.user_constants.insert(repr.into(), value);
    }

    /// Adds the specified user constant to the mathematical context.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::NumberType;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///     context.add_user_constant("c", MathResult::new(NumberType::Real, Complex::new(4.1, 0.0)));
    ///
    ///     let is_built_in_const = context.is_user_constant("c");
    ///     assert!(is_built_in_const == true);
    ///     let constr = context.get_constant_value("c").unwrap();
    ///     assert!(constr.value.re - 4.1 < 10e-10);
    ///
    ///     context.remove_user_constant("c");
    ///     let is_user_const = context.is_user_constant("c");
    ///     assert!(is_user_const == false);
    /// }
    /// ```
    pub fn remove_user_constant<S>(& mut self, repr: S) where S: Into<String> {
        let repr_string = repr.into();
        self.user_constants.remove(& repr_string);
    }

    /// Adds the specified user function to the mathematical context.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::{Token, TokenType, SymbolicTokenType, NumberType};
    /// use termc_model::tree::TreeNode;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///
    ///     let mut input = "f(x) = x";
    ///     let mut f = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), String::from("f"), 0);
    ///     let mut f_node: TreeNode<Token> = TreeNode::new(f);
    ///     let mut x = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), String::from("x"), 2);
    ///     let mut x_node: TreeNode<Token> = TreeNode::new(x);
    ///     f_node.successors.push(Box::new(x_node));
    ///     context.add_user_function("f", f_node, vec![String::from("x")], input);
    ///
    ///     let is_built_in_fun = context.is_user_function("f");
    ///     assert!(is_built_in_fun == true);
    /// }
    /// ```
    pub fn add_user_function<S1, S2>(& mut self, repr: S1, t: TreeNode<Token>, vars: Vec<String>,
                                     input: S2) where S1: Into<String>, S2: Into<String> {
        let repr_string : String = repr.into();
        self.user_functions.insert(repr_string.clone(), (t, vars));
        self.user_function_inputs.insert(repr_string, input.into());
    }

    /// Removes the specified user function to the mathematical context.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::{Token, TokenType, SymbolicTokenType, NumberType};
    /// use termc_model::tree::TreeNode;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///
    ///     let mut input = "f(x) = x";
    ///     let mut f = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), String::from("f"), 0);
    ///     let mut f_node: TreeNode<Token> = TreeNode::new(f);
    ///     let mut x = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), String::from("x"), 2);
    ///     let mut x_node: TreeNode<Token> = TreeNode::new(x);
    ///     f_node.successors.push(Box::new(x_node));
    ///     context.add_user_function("f", f_node, vec![String::from("x")], input);
    ///
    ///     let is_built_in_fun = context.is_user_function("f");
    ///     assert!(is_built_in_fun == true);
    ///
    ///     context.remove_user_function("f");
    ///     let is_built_in_fun = context.is_user_function("f");
    ///     assert!(is_built_in_fun == false);
    /// }
    /// ```
    pub fn remove_user_function<S1>(& mut self, repr: S1) where S1: Into<String> {
        let repr_string: String = repr.into();
        self.user_functions.remove(& repr_string);
        self.user_function_inputs.remove(& repr_string);
    }

    /// Substitutes the arguments of the specified user function with the specified tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::{Token, TokenType, SymbolicTokenType, NumberType};
    /// use termc_model::tree::TreeNode;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///     let mut input = "f(x) = x";
    ///     let mut f = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), String::from("f"), 0);
    ///     let mut f_node: TreeNode<Token> = TreeNode::new(f);
    ///     let mut x = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), String::from("x"), 2);
    ///     let mut x_node: TreeNode<Token> = TreeNode::new(x);
    ///     f_node.successors.push(Box::new(x_node));
    ///     context.add_user_function("f", f_node, vec![String::from("x")], input);
    ///
    ///     let is_built_in_fun = context.is_user_function("f");
    ///     assert!(is_built_in_fun == true);
    ///
    ///     let input2 = "f(0.5)";
    ///     let val_t = Token::new(TokenType::Number(NumberType::Real), String::from("0.5"), 4);
    ///     let val_t_node: TreeNode<Token> = TreeNode::new(val_t);
    ///     let substituted = context.substitute_user_function_tree("f", vec![& val_t_node]).unwrap();
    ///     assert!(substituted.content.get_value() == "f");
    ///     assert!(substituted.successors[0].content.get_value() == "0.5");
    /// }
    /// ```
    pub fn substitute_user_function_tree(& self, repr: & str, args: Vec<& TreeNode<Token>>) -> Option<TreeNode<Token>> {

        let f_entry = self.user_functions.get(repr);
        if f_entry.is_none() {
            return None;
        }
        let f_entry = f_entry.unwrap();
        let mut f_tree = f_entry.0.clone();
        let f_args = &f_entry.1;
        if f_args.len() != args.len() {
            return None;
        }

        let mut args_map : HashMap<String, & TreeNode<Token>> = HashMap::new();
        for i in 0..args.len() {
            args_map.insert(f_args[i].clone(), args[i]);
        }

        MathContext::substitute_user_function_args(& mut f_tree, & args_map);
        Some(f_tree)
    }

    /// Substitutes all types of constant tokens of the specified tree with the tokens mapped by the specified map.
    fn substitute_user_function_args(t: & mut TreeNode<Token>, m: & HashMap<String, & TreeNode<Token>>) {

        match t.content.get_type() {
            TokenType::Constant | TokenType::UserConstant | TokenType::Symbol(SymbolicTokenType::UnknownConstant) => {
                let sub = m.get(t.content.get_value());
                if sub.is_some() {
                    let sub = sub.cloned().unwrap();
                    let sub = sub.clone();
                    *t = sub;
                }
            },
            _ => {
                for succ in t.successors.as_mut_slice() {
                    match succ.content.get_type() {
                        TokenType::Constant | TokenType::UserConstant | TokenType::Symbol(SymbolicTokenType::UnknownConstant) => {
                            let sub = m.get(succ.content.get_value());
                            if sub.is_some() {
                                let sub = sub.cloned().unwrap();
                                let sub = sub.clone();
                                let succ_ref = succ.as_mut();
                                *succ_ref = sub;
                            }
                            else {
                                MathContext::substitute_user_function_args(succ, m);
                            }
                        },

                        _ => {
                            MathContext::substitute_user_function_args(succ, m);
                        }
                    }
                }
            }
        }
    }

    /// Gets the user input that defined the specified user function.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::{Token, TokenType, SymbolicTokenType, NumberType};
    /// use termc_model::tree::TreeNode;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///
    ///     let mut input = "f(x) = x";
    ///     let mut f = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), String::from("f"), 0);
    ///     let mut f_node: TreeNode<Token> = TreeNode::new(f);
    ///     let mut x = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), String::from("x"), 2);
    ///     let mut x_node: TreeNode<Token> = TreeNode::new(x);
    ///     f_node.successors.push(Box::new(x_node));
    ///     context.add_user_function("f", f_node, vec![String::from("x")], input);
    ///
    ///     let f_input = context.get_user_function_input("f").unwrap();
    ///     assert!(f_input == "f(x) = x");
    /// }
    /// ```
    pub fn get_user_function_input(& self, repr: & str) -> Option<String> {
        self.user_function_inputs.get(repr).cloned()
    }

    /// Gets all user defined constants.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::NumberType;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///     context.add_user_constant("c", MathResult::new(NumberType::Real, Complex::new(4.1, 0.0)));
    ///
    ///     let constants = context.get_user_constants();
    ///     assert!(constants.len() == 1);
    ///     assert!(constants.get("c").unwrap().value.re == 4.1);
    /// }
    /// ```
    pub fn get_user_constants(&self) -> HashMap<String, MathResult> {
        self.user_constants.clone()
    }

    /// Gets all user defined function definitions.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate num;
    /// extern crate termc_model;
    ///
    /// use num::complex::Complex;
    /// use termc_model::math_context::MathContext;
    /// use termc_model::math_result::MathResult;
    /// use termc_model::token::{Token, TokenType, SymbolicTokenType, NumberType};
    /// use termc_model::tree::TreeNode;
    ///
    /// fn main() {
    ///     let mut context = MathContext::new();
    ///
    ///     let mut input = "f(x) = x";
    ///     let mut f = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownFunction), String::from("f"), 0);
    ///     let mut f_node: TreeNode<Token> = TreeNode::new(f);
    ///     let mut x = Token::new(TokenType::Symbol(SymbolicTokenType::UnknownConstant), String::from("x"), 2);
    ///     let mut x_node: TreeNode<Token> = TreeNode::new(x);
    ///     f_node.successors.push(Box::new(x_node));
    ///     context.add_user_function("f", f_node, vec![String::from("x")], input);
    ///
    ///     let user_functions = context.get_user_function_definitions();
    ///     assert!(user_functions.len() == 1);
    ///     assert!(user_functions[0] == "f(x) = x");
    /// }
    /// ```
    pub fn get_user_function_definitions(&self) -> Vec<String> {
        let mut result = Vec::new();
        for (_, input) in &self.user_function_inputs {
            result.push(input.clone())
        }
        result
    }
}
