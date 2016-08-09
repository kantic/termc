
extern crate num;

use std::f64;
use std::str::FromStr;
use math_context::{MathContext, OperationType, FunctionType, NumberType};
use parser::tokenizer::{Token, TokenType};
use math_result::MathResult;
use tree::TreeNode;

/// The evaluator.
pub struct Evaluator<'a> {
    /// The math context defining the mathematical environment.
    context: &'a MathContext
}

impl<'a> Evaluator<'a> {

    /// Creates a new Evaluator instance.
    pub fn new(context: &'a MathContext) -> Evaluator {
        Evaluator {context: context}
    }

    /// Evaluates the specified expression tree.
    pub fn evaluate(&self, tree: & TreeNode<Token>) -> Option<MathResult> {
        self.recursive_evaluate(tree)
    }

    /// Evaluates the specified subtree recursively by further splitting it into subtrees.
    pub fn recursive_evaluate(&self, subtree: & TreeNode<Token>) -> Option<MathResult> {

        let token_type = subtree.content.get_type();

        match token_type {
            TokenType::Number(num_type) => {
                match f64::from_str(subtree.content.get_value()) {
                    Ok(x) => {
                        match num_type {
                            NumberType::Real => Some(MathResult::from(x)),
                            NumberType::Complex => Some(MathResult::from(x * self.context.get_constant_value("i").unwrap().value))
                        }
                    },
                    Err(_) => None
                }
            },

            TokenType::Constant => {
                self.context.get_constant_value(subtree.content.get_value())
            },

            TokenType::Operation => {
                let op_type = self.context.get_operation_type(subtree.content.get_value().as_ref());
                if op_type.is_none() {
                    return None;
                }
                let op_type = op_type.unwrap();

                let left_val = self.recursive_evaluate(subtree.successors[0].as_ref());
                if left_val.is_none() {
                    return None;
                }
                let left_val = left_val.unwrap();

                if subtree.successors.len() == 2 {
                    // binary operation
                    let right_val = self.recursive_evaluate(subtree.successors[1].as_ref());
                    if right_val.is_none() {
                        return None;
                    }
                    let right_val = right_val.unwrap();

                    match op_type {
                        OperationType::Add => {
                            Some(MathContext::operation_add(& left_val, & right_val))
                        },
                        OperationType::Sub => {
                            Some(MathContext::operation_sub(& left_val, & right_val))
                        },
                        OperationType::Mul => {
                            Some(MathContext::operation_mul(& left_val, & right_val))
                        },
                        OperationType::Div => {
                            Some(MathContext::operation_div(& left_val, & right_val))
                        },
                        OperationType::Pow => {
                            Some(MathContext::operation_pow(& left_val, & right_val))
                        }
                    }
                }
                else {
                    // unary operation
                    match op_type {
                        OperationType::Add => {
                            Some(MathContext::operation_add(& MathResult::from(0.0), & left_val))
                        },
                        OperationType::Sub => {
                            Some(MathContext::operation_sub(& MathResult::from(0.0), & left_val))
                        },
                        _ => None
                    }
                }
            },

            TokenType::Function => {
                let f_type = self.context.get_function_type(subtree.content.get_value().as_ref());
                if f_type.is_none() {
                    return None;
                }

                let n_successors = subtree.successors.len() as u32;
                let n_args = self.context.get_function_arg_num(subtree.content.get_value()).unwrap_or(0);
                if n_successors != n_args {
                    return None;
                }

                let f_type = f_type.unwrap();

                let mut args : Vec<MathResult> = Vec::new();
                for s in subtree.successors.iter() {
                    match self.recursive_evaluate(s.as_ref()) {
                        Some(x) => {
                            args.push(x)
                        },
                        None => return None
                    }
                }

                match f_type {
                    FunctionType::Cos => {
                        Some(MathContext::function_cos(& args[0]))
                    },
                    FunctionType::Sin => {
                        Some(MathContext::function_sin(& args[0]))
                    },
                    FunctionType::Tan => {
                        Some(MathContext::function_tan(& args[0]))
                    },
                    FunctionType::Cot => {
                        Some(MathContext::function_cot(& args[0]))
                    }
                    FunctionType::Exp => {
                        Some(MathContext::function_exp(& args[0]))
                    },
                    FunctionType::Cosh => {
                        Some(MathContext::function_cosh(& args[0]))
                    },
                    FunctionType::Sinh => {
                        Some(MathContext::function_sinh(& args[0]))
                    },
                    FunctionType::Tanh => {
                        Some(MathContext::function_tanh(& args[0]))
                    },
                    FunctionType::ArcCosh => {
                        Some(MathContext::function_arccosh(& args[0]))
                    },
                    FunctionType::ArcSinh => {
                        Some(MathContext::function_arcsinh(& args[0]))
                    },
                    FunctionType::ArcTanh => {
                        Some(MathContext::function_arctanh(& args[0]))
                    },
                    FunctionType::Sqrt => {
                        Some(MathContext::function_sqrt(& args[0]))
                    },
                    FunctionType::Ln => {
                        Some(MathContext::function_ln(& args[0]))
                    },
                    FunctionType::Pow => {
                        Some(MathContext::operation_pow(& args[0], & args[1]))
                    },
                    FunctionType::Root => {
                        Some(MathContext::operation_pow(& args[0], & MathResult::new(args[1].result_type.clone(),
                                                                                     1.0 / args[1].value)))
                    },
                    FunctionType::ArcCos => {
                        Some(MathContext::function_arccos(& args[0]))
                    },
                    FunctionType::ArcSin => {
                        Some(MathContext::function_arcsin(& args[0]))
                    },
                    FunctionType::ArcTan => {
                        Some(MathContext::function_arctan(& args[0]))
                    },
                    FunctionType::ArcCot => {
                        Some(MathContext::function_arccot(& args[0]))
                    }
                }
            },

            _ => {  // punctuation and unknown tokens should not occur in the evaluation method
                None
            }
        }
    }
}