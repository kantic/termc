
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
                let f_type = f_type.unwrap();

                let arg = self.recursive_evaluate(subtree.successors[0].as_ref());
                if arg.is_none() {
                    return None;
                }
                let arg = arg.unwrap();

                match f_type {
                    FunctionType::Cos => {
                        Some(arg.cos())
                    },
                    FunctionType::Sin => {
                        Some(arg.sin())
                    },
                    FunctionType::Tan => {
                        Some(arg.tan())
                    },
                    FunctionType::Exp => {
                        Some(arg.exp())
                    },
                    FunctionType::Cosh => {
                        Some(arg.cosh())
                    },
                    FunctionType::Sinh => {
                        Some(arg.sinh())
                    },
                    FunctionType::Tanh => {
                        Some(arg.tanh())
                    },
                    FunctionType::Sqrt => {
                        Some(arg.sqrt())
                    },
                    FunctionType::Ln => {
                        Some(arg.ln())
                    }
                }
            },

            _ => {  // punctuation and unknown tokens should not occur in the evaluation method
                None
            }
        }
    }
}