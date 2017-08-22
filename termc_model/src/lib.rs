#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate serde_json;
extern crate serde;
extern crate num;

pub mod math_context;
pub mod math_result;
pub mod token;
pub mod tree;

mod result_error;
mod parser;
mod evaluator;
mod error_templates;
mod f64formatter;

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
///
/// # Examples
///
/// ```
/// use termc_model::math_context::MathContext;
/// use termc_model::math_result::MathResult;
/// use termc_model::get_result;
///
/// fn main() {
///     let mut context = MathContext::new();
///     let input_str = "5+7";
///     let result = get_result(input_str, &mut context);
///     assert!(result.ok().unwrap().unwrap() == MathResult::from((12.0, 0.0)));
/// }
/// ```
pub fn get_result(s: & str, context: & mut MathContext) -> Result<Option<MathResult>, ResultError> {
    match parse(s, context) {
        Ok(ref x) => Ok(try!(evaluate(x, context, s))),
        Err(err) => Err(ResultError::from(err))
    }
}
