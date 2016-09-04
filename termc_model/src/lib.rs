extern crate num;

pub mod math_context;
pub mod math_result;
pub mod token;
pub mod result_error;
pub mod tree;

mod parser;
mod evaluator;
mod error_templates;

#[cfg(test)]
mod test;

use parser::{Parser, ParseError};
use token::Token;
use math_context::MathContext;
use tree::TreeNode;
use evaluator::{Evaluator, EvaluationError};
use math_result::MathResult;
use result_error::ResultError;

/// Creates an expression tree from the specified input string.
fn parse(s: & str, context: & MathContext) -> Result<TreeNode<Token>, ParseError> {

    let mut p = Parser::new(context, s);
    p.parse_toplevel()
}

/// Evaluates the specified expression tree.
fn evaluate(tree: & TreeNode<Token>, context: & mut MathContext, s: & str) -> Result<Option<MathResult>, EvaluationError> {

    let mut e = Evaluator::new(context);
    e.evaluate(tree, s)
}

/// Computes the result of the specified input string containing an mathematical expression.
pub fn get_result(s: & str, context: & mut MathContext) -> Result<Option<MathResult>, ResultError> {
    match parse(s, context) {
        Ok(ref x) => Ok(try!(evaluate(x, context, s))),
        Err(err) => Err(ResultError::from(err))
    }
}