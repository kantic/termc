
extern crate num;

use std::f64;
use std::str::FromStr;
use num::complex::Complex;
use math_context::{MathContext, OperationType, FunctionType, NumberType};
use parser::tokenizer::{Token, TokenType};
use math_result::{MathResult, MathResultType};
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
                let mut result_type = left_val.result_type.clone();

                if subtree.successors.len() == 2 {
                    let right_val = self.recursive_evaluate(subtree.successors[1].as_ref());
                    if right_val.is_none() {
                        return None;
                    }
                    let right_val = right_val.unwrap();
                    if right_val.result_type == MathResultType::Complex {
                        result_type = MathResultType::Complex;
                    }

                    match op_type {
                        OperationType::Add => {
                            Some(MathResult::new(result_type, left_val.value + right_val.value))
                        },
                        OperationType::Sub => {
                            Some(MathResult::new(result_type, left_val.value - right_val.value))
                        },
                        OperationType::Mul => {
                            Some(MathResult::new(result_type, left_val.value * right_val.value))
                        },
                        OperationType::Div => {
                            Some(MathResult::new(result_type, left_val.value / right_val.value))
                        },
                        OperationType::Pow => {
                            Some(calc_pow(& left_val, & right_val, result_type))
                        }
                    }
                }
                else {
                    match op_type {
                        OperationType::Add => {
                            Some(MathResult::new(result_type, 0.0 + left_val.value))
                        },
                        OperationType::Sub => {
                            Some(MathResult::new(result_type, 0.0 - left_val.value))
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

                let mut result_type = MathResultType::Real;
                let mut args : Vec<MathResult> = Vec::new();
                for s in subtree.successors.iter() {
                    match self.recursive_evaluate(s.as_ref()) {
                        Some(x) => {
                            if x.result_type == MathResultType::Complex {
                                result_type = MathResultType::Complex;
                            }
                            args.push(x)
                        },
                        None => return None
                    }
                }

                match f_type {
                    FunctionType::Cos => {
                        Some(MathResult::new(result_type, args[0].value.cos()))
                    },
                    FunctionType::Sin => {
                        Some(MathResult::new(result_type, args[0].value.sin()))
                    },
                    FunctionType::Tan => {
                        Some(MathResult::new(result_type, args[0].value.tan()))
                    },
                    FunctionType::Cot => {
                        Some(MathResult::new(result_type, args[0].value.cos() / args[0].value.sin()))
                    }
                    FunctionType::Exp => {
                        Some(MathResult::new(result_type, args[0].value.exp()))
                    },
                    FunctionType::Cosh => {
                        Some(MathResult::new(result_type, args[0].value.cosh()))
                    },
                    FunctionType::Sinh => {
                        Some(MathResult::new(result_type, args[0].value.sinh()))
                    },
                    FunctionType::Tanh => {
                        Some(MathResult::new(result_type, args[0].value.tanh()))
                    },
                    FunctionType::Sqrt => {
                        Some(MathResult::new(result_type, args[0].value.sqrt()))
                    },
                    FunctionType::Ln => {
                        Some(MathResult::new(result_type, args[0].value.ln()))
                    },
                    FunctionType::Pow => {
                        Some(calc_pow(& args[0], & args[1], result_type))
                    },
                    FunctionType::Root => {
                        Some(calc_pow(& args[0], & MathResult::from(1.0 / args[1].value), result_type))
                    },
                    FunctionType::ArcCos => {
                        if result_type == MathResultType::Real {
                            if !(args[0].value.re <= 1.0 && args[0].value.re >= -1.0) {
                                result_type = MathResultType::Complex;
                            }
                        }
                        Some(MathResult::new(result_type, args[0].value.acos()))
                    },
                    FunctionType::ArcSin => {
                        if result_type == MathResultType::Real {
                            if !(args[0].value.re <= 1.0 && args[0].value.re >= -1.0) {
                                result_type = MathResultType::Complex;
                            }
                        }
                        Some(MathResult::new(result_type, args[0].value.asin()))
                    },
                    FunctionType::ArcTan => {
                        Some(MathResult::new(result_type, args[0].value.atan()))
                    },
                    FunctionType::ArcCot => {
                        Some(MathResult::new(result_type, f64::consts::PI / 2.0 - args[0].value.atan()))
                    }
                }
            },

            _ => {  // punctuation and unknown tokens should not occur in the evaluation method
                None
            }
        }
    }
}

/// Computes the pow of two MathResult instances.
fn calc_pow(left_val: & MathResult, right_val: & MathResult, result_type: MathResultType) -> MathResult {
    match left_val.result_type {
        MathResultType::Real => {
            match right_val.result_type {
                MathResultType::Real => {
                    // ordinary pow, e.g. "a^b"
                    MathResult::new(result_type, Complex::from(left_val.value.re.powf(right_val.value.re)))
                },

                MathResultType::Complex => {
                    // exponent is complex, e.g. "a^(b+ci)" = "exp(ln(a) * (b+ci))"
                    MathResult::new(result_type, (right_val.value * left_val.value.re.ln()).exp())
                }
            }
        },

        MathResultType::Complex =>  {
            // base is complex, e.g. "(a+bi)^c" = "exp(ln(a+bi) * c)" or
            // base and exponent are complex, e.g. "(a+bi)^(c+di)" = "exp(ln(a+bi) * (c+di))"
            MathResult::new(result_type, (left_val.value.ln() * right_val.value).exp())
        }
    }
}