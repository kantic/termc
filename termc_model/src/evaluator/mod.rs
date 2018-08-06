
extern crate num;

use std::f64;
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use std::collections::HashSet;
use error_templates::ExpectedErrorTemplate;
use num::complex::Complex;
use math_context::{MathContext, OperationType, FunctionType};
use token::{Token, TokenType, SymbolicTokenType, NumberType};
use math_result::MathResult;
use tree::TreeNode;

/// Defines the errors that may occur in the evaluation process.
#[derive(Clone, Debug)]
pub enum EvaluationError {
    /// Error if a token occurs that is not of the expected type.
    /// Arguments: ExpectedErrorTemplate instance.
    ExpectedError(ExpectedErrorTemplate),
    /// General evaluation errors.
    /// Arguments: error message.
    GeneralError(String)
}

impl fmt::Display for EvaluationError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvaluationError::ExpectedError(ref tmpl) => write!(f, "{0}", tmpl),
            EvaluationError::GeneralError(ref m) => write!(f, "{0}", m)
        }
    }
}

impl From<ExpectedErrorTemplate> for EvaluationError {

    /// Converts an ExpectedErrorTemplate into an EvaluationError.
    fn from(tmpl: ExpectedErrorTemplate) -> EvaluationError {
        EvaluationError::ExpectedError(tmpl)
    }
}

impl From<String> for EvaluationError {

    /// Converts a String into an EvaluationError.
    fn from(err: String) -> EvaluationError {
        EvaluationError::GeneralError(err)
    }
}

impl<'a> From<&'a str> for EvaluationError {

    /// Converts a String into an EvaluationError.
    fn from(err: & str) -> EvaluationError {
        EvaluationError::GeneralError(String::from(err))
    }
}

impl Error for EvaluationError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            EvaluationError::ExpectedError(_) => "Expected a symbol.",
            EvaluationError::GeneralError(_) => "An error occurred in the evaluation process."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            EvaluationError::ExpectedError(_) => None,
            EvaluationError::GeneralError(_) => None
        }
    }
}

/// Represents a numerical or symbolical evaluation result.
pub enum EvaluationResult {
    Numerical(MathResult),
    Symbolical(TreeNode<Token>)
}

impl<'a> From<MathResult> for EvaluationResult {

    /// Converts a MathResult into an EvaluationResult.
    fn from(res: MathResult) -> EvaluationResult {

        // Check if a complex MathResult object can be reduced to a real MathResult object
        if res.result_type == NumberType::Complex && res.value.im == 0.0_f64 {
            EvaluationResult::Numerical(MathResult::from(res.value.re))
        }
        else {   
            EvaluationResult::Numerical(res)
        }
    }
}

impl<'a> From<&'a TreeNode<Token>> for EvaluationResult {

    /// Converts a TreeNode reference to an EvaluationResult.
    fn from(node: & TreeNode<Token>) -> EvaluationResult {
        EvaluationResult::Symbolical(node.clone())
    }
}

impl<'a> From<f64> for EvaluationResult {

    /// Converts a float to an EvaluationResult.
    fn from(f: f64) -> EvaluationResult {
        EvaluationResult::Numerical(MathResult::from(f))
    }
}

impl<'a> From<Complex<f64>> for EvaluationResult {

    /// Converts a complex number to an EvaluationResult.
    fn from(c: Complex<f64>) -> EvaluationResult {
        EvaluationResult::Numerical(MathResult::from(c))
    }
}

/// The evaluator.
pub struct Evaluator<'a> {
    /// The math context defining the mathematical environment.
    context: &'a mut MathContext
}

/// Provides parse-interface from strings.
trait RadixParse {
    /// The output type (Self for most purposes).
    type Output;
    /// Parses the specified string. If the parsing-process succeeds, a value of type Output is returned.
    /// Otherwise, an EvaluationError is returned which marks the specified position end_pos.
    fn parse_float(s: String, end_pos: usize) -> Result<Self::Output, EvaluationError>;
}

macro_rules! parse_radix {
    ($s:ident, $base:expr, $end_pos:ident) => {{
    // s: the string to be parsed
    // base: the base of the number system (e.g. 2 (binary), 8 (octal) or 16 (hexadecimal))
    // end_pos: the end position of s in the user input string (for error message generation)

        // remove the format prefix ("0b", "0x" or "0o")
        let mut counter = 2;
        let s_clean : String = $s.chars().skip_while(|_| {let ret = counter > 0; counter -= 1; ret} ).collect();
        let mut v : Vec<&str> = s_clean.split('.').collect();

        // initialise parsed result with 0
        let mut result : f64 = 0.0_f64;
        let mut is_err = false;

        if v.len() > 2 {
            // a valid number string can contain only one ".", e.g. "15.75",
            // and therefore the split string should at most contain two elements
            is_err = true;
        }
        else {
            if v.len() == 2 {
                // len == 2 => the number string is of the form <xxx>.<xxx>, here we parse the part after the "."
                let post = v.pop().unwrap();
                if post.chars().count() > 0 {
                    let count = post.chars().count() as i32;
                    let f = i64::from_str_radix(post, $base);
                    match f {
                        Ok(n) => result += (n as f64) * ($base as f64).powf(-count as f64),
                        Err(_) => is_err = true
                    }
                }
            }

            if v.len() > 0 {
                // here, we parse the parse the part before the "."
                let pre = v.pop().unwrap();
                if pre.chars().count() > 0 {
                    let f = i64::from_str_radix(pre, $base);
                    match f {
                        Ok(n) => result += n as f64,
                        Err(_) => is_err = true
                    }
                }
            }
        }

        if is_err {
            Err(EvaluationError::from(ExpectedErrorTemplate::new($s.clone(), "literal number", Some("Invalid literal symbol(s)".to_string()),
                                                                 $end_pos)))
        }
        else {
            Ok(result)
        }
    }}
}

impl RadixParse for f64 {
    type Output = Self;
    /// Implements the RadixParse trait for the f64 type.
    fn parse_float(s: String, end_pos: usize) -> Result<Self::Output, EvaluationError> {
        if s.starts_with("0x") {
            parse_radix!(s, 16_u32, end_pos)
        }
        else if s.starts_with("0o") {
            parse_radix!(s, 8_u32, end_pos)
        }
        else if s.starts_with("0b") {
            parse_radix!(s, 2_u32, end_pos)
        }
        else {
            match f64::from_str(&s) {
                Ok(f) => Ok(f),
                Err(_) => Err(EvaluationError::from(ExpectedErrorTemplate::new(s, "literal number", Some("Invalid literal symbol(s)".to_string()),
                                                                                   end_pos)))
            }
        }
    }
}

impl<'a> Evaluator<'a> {

    /// Creates a new Evaluator instance.
    pub fn new(context: &'a mut MathContext) -> Evaluator {
        Evaluator {context: context}
    }

    /// Evaluates the specified expression tree.
    /// The result is None if the evaluated expression is an assignment which returns no numerical value.
    pub fn evaluate(&'a mut self, tree: & TreeNode<Token>, input: &'a str) -> Result<Option<MathResult>, EvaluationError> { // Option<MathResult>: if none, then no result (e.g. assignment)
        let result = self.recursive_evaluate(tree, &input)?;
        match result {
            EvaluationResult::Numerical(x) => {
                self.context.add_user_constant("ans", x.clone());
                Ok(Some(x))
            },
            EvaluationResult::Symbolical(sym) => {
                match sym.content.get_type() {
                    TokenType::Operation => {
                        Ok(None)
                    },

                    TokenType::Symbol(SymbolicTokenType::UnknownConstant) => {
                        Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "built-in or user defined constant", Some(
                            format!("unknown constant \"{0}\"", sym.content)), sym.content.get_end_pos())))
                    },

                    TokenType::Symbol(SymbolicTokenType::UnknownFunction) => {
                        Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "built-in or user defined function", Some(
                            format!("unknown function \"{0}(...)\"", sym.content)), sym.content.get_end_pos())))
                    },

                    _ => {
                        Err(EvaluationError::from(format!("Error: The evaluation result is neither numerical nor an assignment.\nFound symbolical expression \"{}\".", sym.content)))
                    }
                }
            }
        }
    }

    /// Evaluates the specified subtree recursively by further splitting it into subtrees.
    /// Returns a numerical or symbolical evaluation result.
    pub fn recursive_evaluate(& mut self, subtree: & TreeNode<Token>, input: & str) -> Result<EvaluationResult, EvaluationError> {

        let token_type = subtree.content.get_type();

        match token_type {
            TokenType::Number(num_type) => {
                let x = f64::parse_float(subtree.content.get_value().to_string(), subtree.content.get_end_pos())?;
                match num_type {
                            NumberType::Real => Ok(EvaluationResult::from(x)),
                            NumberType::Complex => Ok(EvaluationResult::from(x * self.context.get_constant_value("i").unwrap().value))
                }
            },

            TokenType::Constant | TokenType::UserConstant => {
                let c_val = self.context.get_constant_value(subtree.content.get_value()).ok_or(
                    EvaluationError::from(ExpectedErrorTemplate::new(input, "constant", Some(subtree.content.get_value().to_string()), subtree.content.get_end_pos())))?;
                Ok(EvaluationResult::from(c_val))
            },

            TokenType::Operation => {
                let op_type = self.context.get_operation_type(subtree.content.get_value().as_ref());
                let op_type = op_type.unwrap(); // the parser ensures that this is a valid operation type

                if !(subtree.successors.len() > 0) {
                    // this operation has no operands => error
                    return Err(EvaluationError::from(ExpectedErrorTemplate::new(
                        input, "operands", Some(format!("operation \"{0}\" without any operands", subtree.content)), subtree.content.get_end_pos())))
                }

                if op_type == OperationType::Assign {
                    if subtree.successors.len() != 2 {
                        return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "2 arguments", Some(
                            format!("{0} arguments", subtree.successors.len())), subtree.content.get_end_pos())))
                    }

                    let left_val_sym = self.error_if_built_in(subtree.successors[0].as_ref(), input)?;
                    match left_val_sym.content.get_type() {
                        TokenType::Symbol(SymbolicTokenType::UnknownConstant) | TokenType::UserConstant => {
                            self.context.remove_user_constant(left_val_sym.content.get_value());
                            let right_val = self.recursive_evaluate(subtree.successors[1].as_ref(), input)?;
                            let right_val_num = Evaluator::error_if_symbolic(right_val, input)?;
                            self.context.add_user_constant(left_val_sym.content.get_value(), right_val_num);
                            Ok(EvaluationResult::from(subtree))
                        },

                        TokenType::Symbol(SymbolicTokenType::UnknownFunction) | TokenType::UserFunction => {
                            let f_name = left_val_sym.content.get_value();
                            self.context.remove_user_function(f_name);
                            let f_args = Evaluator::get_function_args(left_val_sym, input)?;
                            self.check_function_definition(subtree.successors[1].as_ref(), & f_args, input)?;
                            self.context.add_user_function(f_name, subtree.successors[1].as_ref().clone(), f_args, input);
                            Ok(EvaluationResult::from(subtree))
                        },

                        _ => {
                            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "constant or function definition", Some(
                            format!("expression \"{0}\"", left_val_sym.content)), left_val_sym.content.get_end_pos())))
                        }
                    }
                }
                else {
                    let left_val = self.recursive_evaluate(subtree.successors[0].as_ref(), input)?;
                    let left_val_num = Evaluator::error_if_symbolic(left_val, input)?;
                    if subtree.successors.len() == 2 {
                        // binary operation
                        let right_val = self.recursive_evaluate(subtree.successors[1].as_ref(), input)?;
                        let right_val_num = Evaluator::error_if_symbolic(right_val, input)?;
                        match op_type {
                            OperationType::Add => Ok(EvaluationResult::from(MathContext::operation_add(& left_val_num, & right_val_num))),
                            OperationType::Sub => Ok(EvaluationResult::from(MathContext::operation_sub(& left_val_num, & right_val_num))),
                            OperationType::Mul => Ok(EvaluationResult::from(MathContext::operation_mul(& left_val_num, & right_val_num))),
                            OperationType::Div => Ok(EvaluationResult::from(MathContext::operation_div(& left_val_num, & right_val_num))),
                            OperationType::Pow => Ok(EvaluationResult::from(MathContext::operation_pow(& left_val_num, & right_val_num))),
                            OperationType::Mod => Ok(EvaluationResult::from(MathContext::operation_mod(& left_val_num, & right_val_num))),
                            _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "binary mathematical operation",
                                                                                      Some(format!("operation \"{0}\"", subtree.content)),
                                                                                      subtree.content.get_end_pos())))
                        }
                    }
                    else {
                        match op_type {
                        OperationType::Add => Ok(EvaluationResult::from(MathContext::operation_add(& MathResult::from(0.0), & left_val_num))),
                        OperationType::Sub => Ok(EvaluationResult::from(MathContext::operation_sub(& MathResult::from(0.0), & left_val_num))),
                        _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "unary operation",
                                                                                  Some(format!("non-unary operation \"{0}\"", subtree.content)),
                                                                                  subtree.content.get_end_pos())))
                        }
                    }
                }
            },

            TokenType::Function | TokenType::UserFunction => {

                // get type of function (cos, sin, exp,..., or a user defined function)
                let f_type = self.context.get_function_type(subtree.content.get_value().as_ref());
                let f_type = f_type.unwrap();

                // get arguments of the function and check if the number of provided arguments matches the number of needed arguments
                let n_successors = subtree.successors.len() as u32;
                let n_args = self.context.get_function_arg_num(subtree.content.get_value()).unwrap();
                if n_successors != n_args {
                    return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, format!("{0} argument(s)", n_args),
                                                                                Some(format!("{0} argument(s)", n_successors)),
                                                                                subtree.content.get_end_pos())));
                }

                // evaluate the provided arguments
                let mut args : Vec<MathResult> = Vec::new();
                for s in subtree.successors.iter() {
                    let x = self.recursive_evaluate(s.as_ref(), input)?;
                    let x_num = Evaluator::error_if_symbolic(x, input)?;
                    args.push(x_num);
                }

                // call the correct function (regarding the function type) with the evaluated arguments
                match f_type {
                    FunctionType::Cos => Ok(EvaluationResult::from(MathContext::function_cos(& args[0]))),
                    FunctionType::Sin => Ok(EvaluationResult::from(MathContext::function_sin(& args[0]))),
                    FunctionType::Tan => Ok(EvaluationResult::from(MathContext::function_tan(& args[0]))),
                    FunctionType::Cot => Ok(EvaluationResult::from(MathContext::function_cot(& args[0]))),
                    FunctionType::Exp => Ok(EvaluationResult::from(MathContext::function_exp(& args[0]))),
                    FunctionType::Cosh => Ok(EvaluationResult::from(MathContext::function_cosh(& args[0]))),
                    FunctionType::Sinh => Ok(EvaluationResult::from(MathContext::function_sinh(& args[0]))),
                    FunctionType::Tanh => Ok(EvaluationResult::from(MathContext::function_tanh(& args[0]))),
                    FunctionType::Coth => Ok(EvaluationResult::from(MathContext::function_coth(& args[0]))),
                    FunctionType::ArcCosh => Ok(EvaluationResult::from(MathContext::function_arccosh(& args[0]))),
                    FunctionType::ArcSinh => Ok(EvaluationResult::from(MathContext::function_arcsinh(& args[0]))),
                    FunctionType::ArcTanh => Ok(EvaluationResult::from(MathContext::function_arctanh(& args[0]))),
                    FunctionType::ArcCoth => Ok(EvaluationResult::from(MathContext::function_arccoth(& args[0]))),
                    FunctionType::Sqrt => Ok(EvaluationResult::from(MathContext::function_sqrt(& args[0]))),
                    FunctionType::Ln => Ok(EvaluationResult::from(MathContext::function_ln(& args[0]))),
                    FunctionType::Pow => Ok(EvaluationResult::from(MathContext::operation_pow(& args[0], & args[1]))),
                    FunctionType::Root => Ok(EvaluationResult::from(MathContext::operation_root(& args[0], & args[1]))),
                    FunctionType::ArcCos => Ok(EvaluationResult::from(MathContext::function_arccos(& args[0]))),
                    FunctionType::ArcSin => Ok(EvaluationResult::from(MathContext::function_arcsin(& args[0]))),
                    FunctionType::ArcTan => Ok(EvaluationResult::from(MathContext::function_arctan(& args[0]))),
                    FunctionType::ArcCot => Ok(EvaluationResult::from(MathContext::function_arccot(& args[0]))),
                    FunctionType::Im => Ok(EvaluationResult::from(MathContext::function_im(& args[0]))),
                    FunctionType::Re => Ok(EvaluationResult::from(MathContext::function_re(& args[0]))),
                    FunctionType::UserFunction => {
                        let slice = subtree.successors.as_slice();
                        let mut args_token : Vec<& TreeNode<Token>> = Vec::new();
                        for succ in slice {
                            args_token.push(succ);
                        }
                        let f_tree = self.context.substitute_user_function_tree(subtree.content.get_value(), args_token);
                        match f_tree {
                            Some(x) => {
                                let f_input = self.context.get_user_function_input(subtree.content.get_value()).unwrap_or(String::new());
                                self.recursive_evaluate(& x, & f_input)
                            },
                            None => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "function call of user defined function", Some(
                                format!("expression {0}", subtree.content)), subtree.content.get_end_pos())))
                        }
                    }
                }
            },

            TokenType::Symbol(sym) => {
                match sym {
                    SymbolicTokenType::UnknownConstant | SymbolicTokenType::UnknownFunction => {
                        Ok(EvaluationResult::from(subtree))
                    }
                }
            }

            _ => {  // punctuation and unknown tokens should not occur in the evaluation method
                Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "function or operation", Some(
                    format!("symbol {0}", subtree.content)), subtree.content.get_end_pos())))
            }
        }
    }

    /// Checks whether the specified EvaluationResult is of symbolic type.
    /// If so, then an EvaluationError is returned, otherwise the numerical MathResult is returned.
    fn error_if_symbolic(res: EvaluationResult, input: & str) -> Result<MathResult, EvaluationError> {
        match res {
            EvaluationResult::Numerical(x) => Ok(x),
            EvaluationResult::Symbolical(n) => {

                match n.content.get_type() {

                    TokenType::Symbol(SymbolicTokenType::UnknownConstant) => Err(EvaluationError::from(ExpectedErrorTemplate::new(
                        input, "built-in or user defined constant", Some(format!("unknown constant \"{0}\"", n.content)), n.content.get_end_pos()))),

                    TokenType::Symbol(SymbolicTokenType::UnknownFunction) => Err(EvaluationError::from(ExpectedErrorTemplate::new(
                        input, "built-in or user defined function", Some(format!("unknown function \"{0}(...)\"", n.content)), n.content.get_end_pos()))),

                    _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(
                        input, "non-symbolic expression", Some(format!("symbolic expression \"{0}\"", n.content)),n.content.get_end_pos())))
                }
            }
        }
    }

    /// Checks whether the specified TreeNode represents a built-in constant or function.
    /// If so, then an EvaluationError is returned, otherwise the TreeNode is returned.
    fn error_if_built_in<'b>(& self, n: &'b TreeNode<Token>, input: & str) -> Result<&'b TreeNode<Token>, EvaluationError> {

        if self.context.is_built_in_function(n.content.get_value()) || self.context.is_built_in_constant(n.content.get_value()) ||
            n.content.get_type() == TokenType::Constant {
            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "new constant name or function name", Some(
                format!("built-in expression \"{0}\"", n.content)), n.content.get_end_pos())))
        }
        else {
            Ok(n)
        }
    }

    /// Returns the list of arguments of the specified function call tree.
    fn get_function_args(n: & TreeNode<Token>, input: & str) -> Result<Vec<String>, EvaluationError> {
        let mut args_set : HashSet<String> = HashSet::new();
        let mut args : Vec<String> = Vec::new();
        for succ in &n.successors {
            if succ.successors.len() != 0 {
                return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "function definition", Some(
                    format!("expression \"{0}\"", n.content)), n.content.get_end_pos())))
            }

            if succ.content.get_type() == TokenType::Number(NumberType::Real) || succ.content.get_type() == TokenType::Number(NumberType::Complex) ||
                succ.content.get_type() == TokenType::Function || succ.content.get_type() == TokenType::UserFunction ||
                succ.content.get_type() == TokenType::Symbol(SymbolicTokenType::UnknownFunction){
                return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "symbolic function argument", Some(
                    format!("expression \"{0}\"", succ.content)), succ.content.get_end_pos())))
            }

            args.push(String::from(succ.content.get_value()));
            args_set.insert(String::from(succ.content.get_value()));
        }

        if args.len() != args_set.len() {
            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "distinct arguments", Some(
                String::from("function definition with partly equal arguments")), n.content.get_end_pos())))
        }
        else {
            Ok(args)
        }
    }

    /// Checks a user function definition tree.
    /// Checks if every symbol is defined.
    fn check_function_definition(& self, n: & TreeNode<Token>, args: & Vec<String>, input: & str) -> Result<(), EvaluationError> {
        if !(n.content.get_type() == TokenType::Number(NumberType::Real) || n.content.get_type() == TokenType::Number(NumberType::Complex)
            || self.context.is_constant(n.content.get_value()) || self.context.is_function(n.content.get_value()) || self.context.is_operation(n.content.get_value())
            || args.iter().any(|x| x == n.content.get_value())) {
            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "non-symbolic expression", Some(
                    format!("symbolic expression \"{0}\"", n.content)), n.content.get_end_pos())))
        }
        else {
            for succ in  &n.successors {
                self.check_function_definition(succ, args, input)?;
            }

            Ok(())
        }
    }
}
