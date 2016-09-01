
extern crate num;

use std::f64;
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use error_templates::ExpectedErrorTemplate;
use std::num::ParseFloatError;
use num::complex::Complex;
use math_context::{MathContext, OperationType, FunctionType};
use token::{Token, TokenType, SymbolicTokenType, NumberType};
use math_result::MathResult;
use tree::TreeNode;

#[derive(Clone, Debug)]
pub enum EvaluationError {
    ExpectedError(ExpectedErrorTemplate),
    //InputError(ParseError),
    NumberError(ParseFloatError),
    //AlreadyExistsError(String, String)
}

impl fmt::Display for EvaluationError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvaluationError::ExpectedError(ref tmpl) => write!(f, "{}", tmpl),
            //EvaluationError::InputError(ref e) => write!(f, "{}", e),
            EvaluationError::NumberError(ref e) => write!(f, "{}", e)//,
            //EvaluationError::AlreadyExistsError(ref what, ref ex) => write!(f, "{} already exists: {}")
        }
    }
}

/*impl From<ParseError> for EvaluationError {

    /// Converts a TokenError into a ParseError.
    fn from(err: ParseError) -> EvaluationError {
        match err {
            ParseError::ExpectedError(ref symbol, ref location_msg) => EvaluationError::ExpectedError(symbol.clone(), location_msg.clone()),
            ParseError::InputError(ref e) => EvaluationError::InputError(ParseError::InputError(e.clone()))
        }
    }
}*/

impl From<ExpectedErrorTemplate> for EvaluationError {

    /// Converts a ExpectedErrorTemplate into a ParseError.
    fn from(tmpl: ExpectedErrorTemplate) -> EvaluationError {
        EvaluationError::ExpectedError(tmpl)
    }
}

impl From<ParseFloatError> for EvaluationError {

    fn from(err: ParseFloatError) -> EvaluationError {
        EvaluationError::NumberError(err)
    }
}

impl Error for EvaluationError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            EvaluationError::ExpectedError(_) => "Expected a symbol.",
            EvaluationError::NumberError(_) => "A number could not be parsed."
            //EvaluationError::InputError(ref err) => err.description()
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            EvaluationError::ExpectedError(_) => None,
            EvaluationError::NumberError(ref err) => Some(err)
            //EvaluationError::InputError(ref err) => Some(err)
        }
    }
}

/// Represents a numerical or symbolical evaluation result.
pub enum EvaluationResult {
    Numerical(MathResult),
    Symbolical(TreeNode<Token>)
}

impl<'a> From<MathResult> for EvaluationResult {
    fn from(res: MathResult) -> EvaluationResult {
        EvaluationResult::Numerical(res)
    }
}

impl<'a> From<&'a TreeNode<Token>> for EvaluationResult {
    fn from(node: & TreeNode<Token>) -> EvaluationResult {
        EvaluationResult::Symbolical(node.clone())
    }
}

impl<'a> From<f64> for EvaluationResult {
    fn from(f: f64) -> EvaluationResult {
        EvaluationResult::Numerical(MathResult::from(f))
    }
}

impl<'a> From<Complex<f64>> for EvaluationResult {
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
            EvaluationResult::Numerical(x) => Ok(Some(x)),
            EvaluationResult::Symbolical(_) => Ok(None)
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

            // Todo: TokenType::UserConstant and TokenType::UserFunction and support user constants and user functions at math_context

            TokenType::Operation => {
                let op_type = self.context.get_operation_type(subtree.content.get_value().as_ref());
                let op_type = op_type.unwrap(); // the parser ensures that this is a valid operation type

                let left_val = try!(self.recursive_evaluate(subtree.successors[0].as_ref(), input));

                if subtree.successors.len() == 2 {  // binary operation
                    let right_val = try!(self.recursive_evaluate(subtree.successors[1].as_ref(), input));
                    // let right_val = try!(right_val.ok_or(EvaluationError::from(ExpectedErrorTemplate::new(input, "non-symbolic expression"))));

                    match op_type {
                        OperationType::Add => {
                            let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            Ok(EvaluationResult::from(MathContext::operation_add(& left_val_num, & right_val_num)))
                        },
                        OperationType::Sub => {
                            let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            Ok(EvaluationResult::from(MathContext::operation_sub(& left_val_num, & right_val_num)))
                        },
                        OperationType::Mul => {
                            let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            Ok(EvaluationResult::from(MathContext::operation_mul(& left_val_num, & right_val_num)))
                        },
                        OperationType::Div => {
                            let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            Ok(EvaluationResult::from(MathContext::operation_div(& left_val_num, & right_val_num)))
                        },
                        OperationType::Pow => {
                            let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            Ok(EvaluationResult::from(MathContext::operation_pow(& left_val_num, & right_val_num)))
                        },
                        OperationType::Assign => {
                            // check left and right hand side
                            let left_val_sym = try!(Evaluator::error_if_built_in(subtree.successors[0].as_ref(), input));
                            let right_val_num = try!(Evaluator::error_if_symbolic(right_val, input));
                            match left_val_sym.content.get_type() {
                                TokenType::Symbol(SymbolicTokenType::UnknownConstant) | TokenType::UserConstant => {
                                    if self.context.is_built_in_constant(left_val_sym.content.get_value()) {
                                        Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "new constant name",
                                                                                             Some(format!("built in constant name \"{}\"", left_val_sym.content.get_value())),
                                                                                             left_val_sym.content.get_end_pos())))
                                    }
                                    else {
                                        self.context.add_user_constant(left_val_sym.content.get_value(), right_val_num);
                                        Ok(EvaluationResult::from(subtree))
                                    }
                                },
                                TokenType::Symbol(SymbolicTokenType::UnknownFunction) | TokenType::UserFunction => {
                                    Ok(EvaluationResult::from(0.0)) // dummy
                                    //TODO: Implement add_user_function function to math_context
                                },
                                _ => {
                                    // error: left hand side of an assignment must be a symbolical type
                                    Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "new or user defined constant or function",
                                                                                         Some(format!("expression {}", left_val_sym.content)),
                                                                                         left_val_sym.content.get_end_pos())))
                                }
                            }
                        }
                    }
                }
                else {
                    // unary operation
                    let left_val_num = try!(Evaluator::error_if_symbolic(left_val, input));
                    match op_type {
                        OperationType::Add => {
                            Ok(EvaluationResult::from(MathContext::operation_add(& MathResult::from(0.0), & left_val_num)))
                        },
                        OperationType::Sub => {
                            Ok(EvaluationResult::from(MathContext::operation_sub(& MathResult::from(0.0), & left_val_num)))
                        },
                        _ => Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "unary operation",
                                                                                  Some(format!("non-unary operation {}", subtree.content)),
                                                                                  subtree.content.get_end_pos())))
                    }
                }
            },

            TokenType::Function => {
                let f_type = self.context.get_function_type(subtree.content.get_value().as_ref());
                let f_type = f_type.unwrap();

                let n_successors = subtree.successors.len() as u32;
                let n_args = self.context.get_function_arg_num(subtree.content.get_value()).unwrap();
                if n_successors != n_args {
                    return Err(EvaluationError::from(ExpectedErrorTemplate::new(input, format!("{} arguments", n_args),
                                                                                Some(format!("{} arguments", n_successors)),
                                                                                subtree.content.get_end_pos())));
                }

                let mut args : Vec<MathResult> = Vec::new();
                for s in subtree.successors.iter() {
                    let x = try!(self.recursive_evaluate(s.as_ref(), input));
                    let x_num = try!(Evaluator::error_if_symbolic(x, input));
                    args.push(x_num);
                }

                match f_type {
                    FunctionType::Cos => {
                        Ok(EvaluationResult::from(MathContext::function_cos(& args[0])))
                    },
                    FunctionType::Sin => {
                        Ok(EvaluationResult::from(MathContext::function_sin(& args[0])))
                    },
                    FunctionType::Tan => {
                        Ok(EvaluationResult::from(MathContext::function_tan(& args[0])))
                    },
                    FunctionType::Cot => {
                        Ok(EvaluationResult::from(MathContext::function_cot(& args[0])))
                    }
                    FunctionType::Exp => {
                        Ok(EvaluationResult::from(MathContext::function_exp(& args[0])))
                    },
                    FunctionType::Cosh => {
                        Ok(EvaluationResult::from(MathContext::function_cosh(& args[0])))
                    },
                    FunctionType::Sinh => {
                        Ok(EvaluationResult::from(MathContext::function_sinh(& args[0])))
                    },
                    FunctionType::Tanh => {
                        Ok(EvaluationResult::from(MathContext::function_tanh(& args[0])))
                    },
                    FunctionType::ArcCosh => {
                        Ok(EvaluationResult::from(MathContext::function_arccosh(& args[0])))
                    },
                    FunctionType::ArcSinh => {
                        Ok(EvaluationResult::from(MathContext::function_arcsinh(& args[0])))
                    },
                    FunctionType::ArcTanh => {
                        Ok(EvaluationResult::from(MathContext::function_arctanh(& args[0])))
                    },
                    FunctionType::Sqrt => {
                        Ok(EvaluationResult::from(MathContext::function_sqrt(& args[0])))
                    },
                    FunctionType::Ln => {
                        Ok(EvaluationResult::from(MathContext::function_ln(& args[0])))
                    },
                    FunctionType::Pow => {
                        Ok(EvaluationResult::from(MathContext::operation_pow(& args[0], & args[1])))
                    },
                    FunctionType::Root => {
                        Ok(EvaluationResult::from(MathContext::operation_pow(& args[0], & MathResult::new(args[1].result_type.clone(),
                                                                                                          1.0 / args[1].value))))
                    },
                    FunctionType::ArcCos => {
                        Ok(EvaluationResult::from(MathContext::function_arccos(& args[0])))
                    },
                    FunctionType::ArcSin => {
                        Ok(EvaluationResult::from(MathContext::function_arcsin(& args[0])))
                    },
                    FunctionType::ArcTan => {
                        Ok(EvaluationResult::from(MathContext::function_arctan(& args[0])))
                    },
                    FunctionType::ArcCot => {
                        Ok(EvaluationResult::from(MathContext::function_arccot(& args[0])))
                    },
                    FunctionType::UserFunction => {
                        Ok(EvaluationResult::from(0.0)) // dummy
                        //TODO: Implement function substituion in math_context
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
                Err(EvaluationError::from(ExpectedErrorTemplate::new(input, "function or operation",
                                                                     Some(format!("{}", subtree.content)),
                                                                     subtree.content.get_end_pos())))
            }
        }
    }

    fn error_if_symbolic(res: EvaluationResult, input: & str) -> Result<MathResult, EvaluationError> {
        match res {
            EvaluationResult::Numerical(x) => Ok(x),
            EvaluationResult::Symbolical(n) => Err(EvaluationError::from(
                ExpectedErrorTemplate::new(input, "non-symbolic expression", Some(format!("symbolic expression {}", n.content.get_value())),
                                           n.content.get_end_pos())))
        }
    }

    fn error_if_built_in<'b>(n: &'b TreeNode<Token>, input: & str) -> Result<&'b TreeNode<Token>, EvaluationError> {

        match n.content.get_type() {
            TokenType::Constant | TokenType::Function => Err(EvaluationError::from(
                ExpectedErrorTemplate::new(input, "new constant or function", Some(format!("built-in expression {}", n.content.get_value())), n.content.get_end_pos()))),
            _ => Ok(n)
        }
    }
}