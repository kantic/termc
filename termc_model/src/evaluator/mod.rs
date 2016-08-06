
use std::f64;
use math_context::{MathContext, OperationType, FunctionType};
use parser::tokenizer::{Token, TokenType};
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
    pub fn evaluate(&self, tree: & TreeNode<Token>) -> Option<f64> {
        self.recursive_evaluate(tree)
    }

    /// Evaluates the specified subtree recursively by further splitting it into subtrees.
    pub fn recursive_evaluate(&self, subtree: & TreeNode<Token>) -> Option<f64> {

        let token_type = subtree.content.get_type();

        match token_type {
            TokenType::Number => {
                match subtree.content.get_value().parse() {
                    Ok(x) => Some(x),
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
                    let right_val = self.recursive_evaluate(subtree.successors[1].as_ref());
                    if right_val.is_none() {
                        return None;
                    }
                    let right_val = right_val.unwrap();

                    match op_type {
                        OperationType::Add => {
                            Some(left_val + right_val)
                        },
                        OperationType::Sub => {
                            Some(left_val - right_val)
                        },
                        OperationType::Mul => {
                            Some(left_val * right_val)
                        },
                        OperationType::Div => {
                            Some(left_val / right_val)
                        },
                        OperationType::Pow => {
                            Some(left_val.powf(right_val))
                        }
                    }
                }
                else {
                    match op_type {
                        OperationType::Add => {
                            Some(0.0 + left_val)
                        },
                        OperationType::Sub => {
                            Some(0.0 - left_val)
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

                let mut args : Vec<f64> = Vec::new();
                for s in subtree.successors.iter() {
                    match self.recursive_evaluate(s.as_ref()) {
                        Some(x) => args.push(x),
                        None => return None
                    }
                }

                match f_type {
                    FunctionType::Cos => {
                        Some(args[0].cos())
                    },
                    FunctionType::Sin => {
                        Some(args[0].sin())
                    },
                    FunctionType::Tan => {
                        Some(args[0].tan())
                    },
                    FunctionType::Cot => {
                        Some(args[0].cos() / args[0].sin())
                    }
                    FunctionType::Exp => {
                        Some(args[0].exp())
                    },
                    FunctionType::Cosh => {
                        Some(args[0].cosh())
                    },
                    FunctionType::Sinh => {
                        Some(args[0].sinh())
                    },
                    FunctionType::Tanh => {
                        Some(args[0].tanh())
                    },
                    FunctionType::Sqrt => {
                        Some(args[0].sqrt())
                    },
                    FunctionType::Ln => {
                        Some(args[0].ln())
                    },
                    FunctionType::Pow => {
                        Some(args[0].powf(args[1]))
                    },
                    FunctionType::Root => {
                        Some(args[0].powf(1.0/args[1]))
                    },
                    FunctionType::ArcCos => {
                        Some(args[0].acos())
                    },
                    FunctionType::ArcSin => {
                        Some(args[0].asin())
                    },
                    FunctionType::ArcTan => {
                        Some(args[0].atan())
                    },
                    FunctionType::ArcCot => {
                        Some(f64::consts::PI / 2.0 - args[0].atan())
                    }
                }
            },

            _ => {  // punctuation and unknown tokens should not occur in the evaluation method
                None
            }
        }
    }
}