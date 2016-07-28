pub mod math_context;
mod tree;
mod parser;
mod evaluator;

#[cfg(test)]
mod test;

use std::f64;
use parser::{Parser, ParseError};
use parser::tokenizer::Token;
use math_context::MathContext;
use tree::TreeNode;
use evaluator::Evaluator;

/// Creates an expression tree from the specified input string.
fn parse(s: & str, context: & MathContext) -> Result<TreeNode<Token>, ParseError> {

    let mut p = Parser::new(context, s);
    p.parse_toplevel()
}

/// Evaluates the specified expression tree.
fn evaluate(tree: & TreeNode<Token>, context: & MathContext) -> Option<f64> {

    let e = Evaluator::new(context);
    e.evaluate(tree)
}

/// Computes the result of the specified input string containing an mathematical expression.
pub fn get_result(s: & str, context: & MathContext) -> Result<f64, ParseError> {

    match parse(s, context) {
        Ok(ref x) => match evaluate(x, context) {
            Some(x) => Ok(x),
            None => Ok(f64::NAN)
        },
        Err(err) => Err(err)
    }
}