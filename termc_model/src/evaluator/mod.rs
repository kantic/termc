
extern crate num;

use std::f64;
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use std::collections::HashSet;
use error_templates::ExpectedErrorTemplate;
use std::num::ParseFloatError;
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
    /// Error if a string could not be parsed to a float.
    /// Arguments: ParseFloatError instance.
    NumberError(ParseFloatError),
    /// General evaluation errors.
    /// Arguments: error message.
    GeneralError(String)
} // Todo: add GeneralError for all the cases where ExpectedError does not really fit

impl fmt::Display for EvaluationError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvaluationError::ExpectedError(ref tmpl) => write!(f, "{}", tmpl),
            EvaluationError::NumberError(ref e) => write!(f, "{}", e),
            EvaluationError::GeneralError(ref m) => write!(f, "{}", m)
        }
    }
}

impl From<ExpectedErrorTemplate> for EvaluationError {

    /// Converts an ExpectedErrorTemplate into an EvaluationError.
    fn from(tmpl: ExpectedErrorTemplate) -> EvaluationError {
        EvaluationError::ExpectedError(tmpl)
    }
}

impl From<ParseFloatError> for EvaluationError {

    /// Converts a ParseFloatError into an EvaluationError.
    fn from(err: ParseFloatError) -> EvaluationError {
        EvaluationError::NumberError(err)
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
            EvaluationError::NumberError(_) => "A number could not be parsed.",
            EvaluationError::GeneralError(_) => "An error occurred in the evaluation process."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            EvaluationError::ExpectedError(_) => None,
            EvaluationError::NumberError(ref err) => Some(err),
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

    /// Concerts a MathResult into an EvaluationResult.
    fn from(res: MathResult) -> EvaluationResult {
        EvaluationResult::Numerical(res)
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

impl<'a> Evaluator<'a> {

    /// Creates a new Evaluator instance.
    pub fn new(context: &'a mut MathContext) -> Evaluator {
        Evaluator {context: context}
    }

    /// Evaluates the specified expression tree.
    /// The result is None if the evaluated expression is an assignment which returns no numerical value.
    pub fn evaluate(&'a mut self, tree: & TreeNode<Token>, input: & str) -> Result<Option<MathResult>, EvaluationError> { // Option<MathResult>: if none, then no result (e.g. assignment)
        let result = try!(self.recursive_evaluate(tree, input));
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

                    _ => {
                        Err(EvaluationError::from(format!("Error: The evaluation result is neither numerical nor an assignment.\nFound symbolical expression \"{}\".", sym.content))) // FixMe: GeneralError
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
                let x = try!(f64::from_str(subtree.content.get_value()));
                match num_type {
                            NumberType::Real => Ok(EvaluationResult::from(x)),
                            NumberType::Complex => Ok(EvaluationResult::from(x * self.context.get_constant_value("i").unwrap().value))
                }
            },

            TokenType::Constant | TokenType::UserConstant => {
                let c_val = try!(self.context.get_constant_value(subtree.content.get_value()).ok_or(
                    EvaluationError::from(ExpectedErrorTemplate::new(input, "constant", Some(subtree.content.get_value().to_string()), subtree.content.get_end_pos()))));
                Ok(EvaluationResult::from(c_val))
            },

            TokenType::Operation => {
                let op_type = self.context.get_operation_type(subtree.content.get_value().as_ref());
                let op_type = op_type.unwrap(); // the parser ensures that this is a valid operation type

                if !(subtree.successors.len() > 0) {
                    // this operation has no operands => error
                    return Err(EvaluationError::from(ExpectedErrorTemplate::new(
                        input, "operands", Some(format!("operation \"{}\" without any operands", subtree.content)), subtree.content.get_end_pos())))
                }

                if op_type == OperationType::Assign {
                    if subtree.successors.len() != 2 {
                        return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "2 arguments", Some(
                            format!("{} arguments", subtree.successors.len())), subtree.content.get_end_pos())))
                    }

                    let left_val_sym = try!(self.error_if_built_in(subtree.successors[0].as_ref(), input));
                    match left_val_sym.content.get_type() {
                        TokenType::Symbol(SymbolicTokenType::UnknownConstant) | TokenType::UserConstant => {
                            let right_val = try!(self.recursive_evaluate(subtree.successors[1].as_ref(), input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            self.context.add_user_constant(left_val_sym.content.get_value(), right_val_num);
                            Ok(EvaluationResult::from(subtree))
                        },

                        TokenType::Symbol(SymbolicTokenType::UnknownFunction) | TokenType::UserFunction => {
                            let f_name = left_val_sym.content.get_value();
                            let f_args = try!(Evaluator::get_function_args(left_val_sym, input));
                            self.context.add_user_function(f_name, subtree.successors[1].as_ref().clone(), f_args, input);
                            Ok(EvaluationResult::from(subtree))
                        },

                        _ => {
                            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "constant or function definition", Some(
                            format!("expression \"{}\"", left_val_sym.content)), left_val_sym.content.get_end_pos())))
                        }
                    }
                }
                else {
                    let left_val = try!(self.recursive_evaluate(subtree.successors[0].as_ref(), input));
                    let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                    if subtree.successors.len() == 2 {
                        // binary operation
                        let right_val = try!(self.recursive_evaluate(subtree.successors[1].as_ref(), input));
                        let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                        match op_type {
                            OperationType::Add => Ok(EvaluationResult::from(MathContext::operation_add(& left_val_num, & right_val_num))),
                            OperationType::Sub => Ok(EvaluationResult::from(MathContext::operation_sub(& left_val_num, & right_val_num))),
                            OperationType::Mul => Ok(EvaluationResult::from(MathContext::operation_mul(& left_val_num, & right_val_num))),
                            OperationType::Div => Ok(EvaluationResult::from(MathContext::operation_div(& left_val_num, & right_val_num))),
                            OperationType::Pow => Ok(EvaluationResult::from(MathContext::operation_pow(& left_val_num, & right_val_num))),
                            _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "binary mathematical operation",
                                                                                      Some(format!("operation \"{}\"", subtree.content)),
                                                                                      subtree.content.get_end_pos())))
                        }
                    }
                    else {
                        match op_type {
                        OperationType::Add => Ok(EvaluationResult::from(MathContext::operation_add(& MathResult::from(0.0), & left_val_num))),
                        OperationType::Sub => Ok(EvaluationResult::from(MathContext::operation_sub(& MathResult::from(0.0), & left_val_num))),
                        _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "unary operation",
                                                                                  Some(format!("non-unary operation \"{}\"", subtree.content)),
                                                                                  subtree.content.get_end_pos())))
                        }
                    }
                }
            },

            TokenType::Function | TokenType::UserFunction => {
                let f_type = self.context.get_function_type(subtree.content.get_value().as_ref());
                let f_type = f_type.unwrap();

                let n_successors = subtree.successors.len() as u32;
                let n_args = self.context.get_function_arg_num(subtree.content.get_value()).unwrap();
                if n_successors != n_args {
                    return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, format!("{} argument(s)", n_args),
                                                                                Some(format!("{} argument(s)", n_successors)),
                                                                                subtree.content.get_end_pos())));
                }

                let mut args : Vec<MathResult> = Vec::new();
                for s in subtree.successors.iter() {
                    let x = try!(self.recursive_evaluate(s.as_ref(), input));
                    let x_num = try!(Evaluator::error_if_symbolic(x, input));
                    args.push(x_num);
                }

                match f_type {
                    FunctionType::Cos => Ok(EvaluationResult::from(MathContext::function_cos(& args[0]))),
                    FunctionType::Sin => Ok(EvaluationResult::from(MathContext::function_sin(& args[0]))),
                    FunctionType::Tan => Ok(EvaluationResult::from(MathContext::function_tan(& args[0]))),
                    FunctionType::Cot => Ok(EvaluationResult::from(MathContext::function_cot(& args[0]))),
                    FunctionType::Exp => Ok(EvaluationResult::from(MathContext::function_exp(& args[0]))),
                    FunctionType::Cosh => Ok(EvaluationResult::from(MathContext::function_cosh(& args[0]))),
                    FunctionType::Sinh => Ok(EvaluationResult::from(MathContext::function_sinh(& args[0]))),
                    FunctionType::Tanh => Ok(EvaluationResult::from(MathContext::function_tanh(& args[0]))),
                    FunctionType::ArcCosh => Ok(EvaluationResult::from(MathContext::function_arccosh(& args[0]))),
                    FunctionType::ArcSinh => Ok(EvaluationResult::from(MathContext::function_arcsinh(& args[0]))),
                    FunctionType::ArcTanh => Ok(EvaluationResult::from(MathContext::function_arctanh(& args[0]))),
                    FunctionType::Sqrt => Ok(EvaluationResult::from(MathContext::function_sqrt(& args[0]))),
                    FunctionType::Ln => Ok(EvaluationResult::from(MathContext::function_ln(& args[0]))),
                    FunctionType::Pow => Ok(EvaluationResult::from(MathContext::operation_pow(& args[0], & args[1]))),
                    FunctionType::Root => Ok(EvaluationResult::from(MathContext::operation_pow(& args[0], & MathResult::new(
                        args[1].result_type.clone(), 1.0 / args[1].value)))),
                    FunctionType::ArcCos => Ok(EvaluationResult::from(MathContext::function_arccos(& args[0]))),
                    FunctionType::ArcSin => Ok(EvaluationResult::from(MathContext::function_arcsin(& args[0]))),
                    FunctionType::ArcTan => Ok(EvaluationResult::from(MathContext::function_arctan(& args[0]))),
                    FunctionType::ArcCot => Ok(EvaluationResult::from(MathContext::function_arccot(& args[0]))),
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
                                format!("expression {}", subtree.content)), subtree.content.get_end_pos())))
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
                    format!("symbol {}", subtree.content)), subtree.content.get_end_pos())))
            }
        }
    }

    /// Checks whether the specified EvaluationResult is of symbolic type.
    /// If so, then an EvaluationError is returned, otherwise the numerical MathResult is returned.
    fn error_if_symbolic(res: EvaluationResult, input: & str) -> Result<MathResult, EvaluationError> {
        match res {
            EvaluationResult::Numerical(x) => Ok(x),
            EvaluationResult::Symbolical(n) => Err(EvaluationError::from(
                ExpectedErrorTemplate::new(input, "non-symbolic expression", Some(format!("symbolic expression \"{}\"", n.content)),
                                           n.content.get_end_pos())))
        }
    }

    /// Checks whether the specified TreeNode represents a built-in constant or function.
    /// If so, then an EvaluationError is returned, otherwise the TreeNode is returned.
    fn error_if_built_in<'b>(& self, n: &'b TreeNode<Token>, input: & str) -> Result<&'b TreeNode<Token>, EvaluationError> {

        if self.context.is_built_in_function(n.content.get_value()) || self.context.is_built_in_constant(n.content.get_value()) ||
            n.content.get_type() == TokenType::Constant {
            Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "new constant name or function name", Some(
                format!("built-in expression \"{}\"", n.content)), n.content.get_end_pos())))
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
                    format!("expression \"{}\"", n.content)), n.content.get_end_pos())))
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
}