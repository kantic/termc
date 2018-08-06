pub mod tokenizer;

use std::fmt;
use std::error::Error;
use error_templates::ExpectedErrorTemplate;
use token::{Token, TokenType, SymbolicTokenType, NumberType};
use parser::tokenizer::{Tokenizer, TokenError};
use math_context::MathContext;
use tree::TreeNode;

/// Defines the errors that may occur when parsing the user input string.
#[derive(Clone, Debug)]
pub enum ParseError {
    /// Error if the current token does not match the expected token.
    /// Arguments: Expected symbol, marked in the input string
    ExpectedError(ExpectedErrorTemplate),
    /// General input error of the user input (syntax error).
    /// Arguments: TokenError that causes the InputError
    InputError(TokenError),
    /// Given expression is incomplete.
    IncompleteInputError
}

impl fmt::Display for ParseError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::ExpectedError(ref e) => write!(f, "{0}", e),
            ParseError::InputError(ref e) => write!(f, "{0}", e),
            ParseError::IncompleteInputError => write!(f, "{0}", self.description())
        }
    }
}

impl From<TokenError> for ParseError {

    /// Converts a TokenError into a ParseError.
    fn from(err: TokenError) -> ParseError {
        ParseError::InputError(err)
    }
}

impl From<ExpectedErrorTemplate> for ParseError {

    /// Converts a ExpectedErrorTemplate into a ParseError.
    fn from(tmpl: ExpectedErrorTemplate) -> ParseError {
        ParseError::ExpectedError(tmpl)
    }
}

impl Error for ParseError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            ParseError::ExpectedError(_) => "Expected a symbol.",
            ParseError::InputError(ref err) => err.description(),
            ParseError::IncompleteInputError => "Expression is incomplete."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            ParseError::ExpectedError(_) => None,
            ParseError::InputError(ref err) => Some(err),
            ParseError::IncompleteInputError => None
        }
    }
}

/// Defines the Parser.
pub struct Parser<'a> {
    /// The mathematical environment.
    context: &'a MathContext,
    /// The Tokenizer.
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {

    /// Creates a new Parser instance.
     pub fn new(context: &'a MathContext, s: &'a str) -> Parser<'a> {
         Parser { context: context, tokenizer: Tokenizer::new(context, s) }
     }

    /// Returns true if the current token is the specified punctuation character.
    /// Returns false otherwise.
    fn is_punc(& self, s: & str) -> bool {
        let token = match self.tokenizer.peek() {
            Some(Ok(t)) => t,
            _ => return false
        };

        if token.get_type() == TokenType::Punctuation && token.get_value() == s {
            true
        }
        else {
            false
        }
    }

    /// Checks whether the current token is the specified punctuation token and skips it if this
    /// is true. Returns an error otherwise.
    fn skip_punc(& mut self, s: & str) -> Result<(), ParseError> {
        if self.is_punc(s) {
            match self.tokenizer.next() {
                Some(res) => res?,
                None => return Err(ParseError::IncompleteInputError)
            };
            Ok(())
        }
        else {
            match self.tokenizer.peek() {
                Some(Ok(t)) => Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(),
                                                                         format!("symbol \"{0}\"", s), Some(format!("\"{}\"", t)), t.get_end_pos()))),

                Some(Err(e)) => Err(ParseError::from(e)),
                
                None => Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), format!("symbol \"{}\"", s),
                                                                        None, self.tokenizer.get_pos() + 1)))
            }
        }
    }

    /// Starts parsing the user input.
    pub fn parse_toplevel(& mut self) -> Result<TreeNode<Token>, ParseError> {

        let result = self.parse_expression();
        if result.is_ok() {
            if !self.tokenizer.eof() {
                return Err(ParseError::ExpectedError(ExpectedErrorTemplate::new(self.tokenizer.get_input(),
                            "end of input".to_string(), None, self.tokenizer.get_pos() + 1)));
            }
        }

        result
    }

    /// Parses an element of the expression. This can be one of the following:
    /// (1) a whole expression in parenthesis
    /// (2) an operand (number, constant or function call)
    /// (3) an unary operation (that is not further processed)
    fn parse_element(& mut self) -> Result<TreeNode<Token>, ParseError> {

        if self.is_punc("(") {
            // expression in parenthesis
            match self.tokenizer.next() {
                Some(res) => res?,
                None => return Err(ParseError::IncompleteInputError)
            };
            let exp = self.parse_expression()?;
            self.skip_punc(")")?;
            return Ok(exp);
        }
        else {
            let t = match self.tokenizer.next() {
                Some(res) => res?,
                None => return Err(ParseError::IncompleteInputError)
            };
            let token_type = t.get_type();

            match token_type {
                TokenType::Number(_) | TokenType::Constant | TokenType::UserConstant | TokenType::Symbol(SymbolicTokenType::UnknownConstant) => {
                    Ok(TreeNode::new(t))
                },
                TokenType::Function | TokenType::UserFunction | TokenType::Symbol(SymbolicTokenType::UnknownFunction) => {
                    // return the complete parsed function call subtree
                    self.parse_function(t)
                },
                TokenType::Operation => {
                    // Return the unprocessed unary operation symbol.
                    // This case is relevant as an unary operation can appear instead of an operand
                    // (e.g. "3+-2" (= 1), where after the "+" operation an operand is expected,
                    // but the unary "-" operation appears)
                    if self.context.is_unary_operation(t.get_value()) {
                        Ok(TreeNode::new(t))
                    }
                    else {
                        Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation",
                                                                        Some(format!("non-unary operation \"{0}\"", t)), t.get_end_pos())))
                    }
                },
                _ => {
                    Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "operand (number, constant, function call) or an unary operation",
                                                                    Some(format!("unexpected symbol \"{0}\"", t)), t.get_end_pos())))
                }
            }
        }
    }

    /// Parses an expression.
    fn parse_expression(& mut self) -> Result<TreeNode<Token>, ParseError> {

        self.parse_operation()
    }

    /// Parses a function call.
    fn parse_function(& mut self, t: Token) -> Result<TreeNode<Token>, ParseError> {

        self.skip_punc("(")?;
        let args = self.parse_function_arg_list()?;
        self.skip_punc(")")?;

        let mut ret = TreeNode::new(t);
        // add the function arguments as a subtrees to the function
        for arg in args.into_iter() {
            ret.successors.push(Box::new(arg));
        }
        Ok(ret)
    }

    /// Parses the argument list of a function call.
    fn parse_function_arg_list(& mut self) -> Result<Vec<TreeNode<Token>>, ParseError> {

        let mut args : Vec<TreeNode<Token>> = Vec::new();
        if self.tokenizer.eof() || self.is_punc(")") {
            // The function call has no arguments
            return Ok(args);
        }

        while !self.tokenizer.eof() {
            let arg = self.parse_expression()?;
            args.push(arg);

            if self.tokenizer.eof() {
                break;
            }

            if self.is_punc(",") {
                self.skip_punc(",")?;
                if self.is_punc(")") {
                    let pos = self.tokenizer.get_pos();
                    return Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(),
                                                                           "an argument", Some("symbol \")\"".to_string()), pos)));
                }
            }
            else if self.is_punc(")") {
                // All arguments have been parsed
                break;
            }
            else {
                // If in the argument list after an expression neither a "," symbol nor an ")" occurs,
                // return an error
                let peeked = match self.tokenizer.peek() { // this should be safe because it has been tested for eof
                    Some(Ok(t)) => t,
                    Some(Err(e)) => return Err(ParseError::from(e)),

                    None => return Err(ParseError::IncompleteInputError)
                };
                return Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "\",\" or \")\"",
                                                                       Some(format!("\"{0}\"", peeked)), peeked.get_end_pos())));
            }
        }

        Ok(args)
    }

    /// Parses an operation expression tree while making the distinction between unary and binary
    /// operations (e.g. "-3*-5" is an unary expression ("-3") followed by an binary operation
    /// ("<...>*<...>") followed by an unary expression ("-5")).
    fn parse_operation(& mut self) -> Result<TreeNode<Token>, ParseError> {

        let elem = self.parse_element()?;

        if !self.tokenizer.eof() {
            if elem.content.get_type() == TokenType::Operation && elem.successors.len() == 0 {

                // If elem is an operation, it must be a unary operation, otherwise it would be a syntax error.
                // Hence, if the unary operation has not yet been parsed (that means, the unary
                // expression has no operand yet), it does not have any successors.
                if self.context.is_unary_operation(elem.content.get_value()) {
                    let unary = self.recursive_parse_unary(elem)?;

                    // After the unary expression, the user input might go on with an binary expression
                    // (e.g. "-3+2"; after the unary expression "-3" the user input goes on with the
                    // binary expression "(-3)+2"), so we have to parse that, too!
                    if !self.tokenizer.eof() {
                        // Call recursive binary with precedence 0, because an unary expression is
                        // interpreted as a modification of an operand, and operands always have
                        // precedence 0.
                        self.recursive_parse_binary(unary, 0)
                    }
                    else {
                        // The user input consists only of one single unary expression, e.g. "-3".
                        Ok(unary)
                    }
                }
                else {
                    Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation",
                                                                    Some(format!("non-unary operation \"{}\"", elem.content)), elem.content.get_end_pos())))
                }
            }
            else {
                // If elem is an operand or an unary expression that has already been parsed, then
                // the next thing in the user input can only be a binary expression (because unary
                // expressions are always parsed completely).
                self.recursive_parse_binary(elem, 0)
            }
        }
        else {
            // There are no more tokens to parse...
            Ok(elem)
        }
    }

    /// Parses a binary expression.
    fn recursive_parse_binary(& mut self, left: TreeNode<Token>, my_prec: u32) -> Result<TreeNode<Token>, ParseError> {

        // The argument "left" must be an operand (number constant or function call) or an unary expression
        // (that is interpreted also as a modified operand).
        let t = match self.tokenizer.peek() {
            Some(Ok(t)) => t,
            Some(Err(t)) => return Err(ParseError::from(t)),
            None => return Err(ParseError::IncompleteInputError)
        };
        if t.get_type() == TokenType::Operation {
            let his_prec = self.context.get_operation_precedence(t.get_value()).unwrap();
            if his_prec > my_prec {
                let t = match self.tokenizer.next() {
                    Some(res) => res?,
                    None => return Err(ParseError::IncompleteInputError)
                };
                let mut wrap = TreeNode::new(t); 
                // "left" is the left operand of the binary operation "t", so add it as an successor
                wrap.successors.push(Box::new(left));
                let elem = self.parse_element()?;

                // Now, "elem" can either be an operand (number, constant or function call) or
                // an unary expression (meaning a modified operand)
                if elem.content.get_type() == TokenType::Operation && elem.successors.len() == 0 {
                    // If "elem" is an unparsed (=> no successors) unary operation, parse it
                    if self.context.is_unary_operation(elem.content.get_value()) {

                        // the unary expression is the right operand of the binary operation "t"
                        let unary = self.recursive_parse_unary(elem)?;
                        if !self.tokenizer.eof() {
                            let right = self.recursive_parse_binary(unary, his_prec)?;
                            wrap.successors.push(Box::new(right));
                        }
                        else {
                            wrap.successors.push(Box::new(unary));
                        }
                    }
                    else {
                        return Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation",
                                                                               Some(format!("non-unary operation \"{0}\"", elem.content)), elem.content.get_end_pos())));
                    }
                }
                else {
                    // "elem" must be an operand or an parsed unary expression.
                    // Check for further operations with higher precedence than "t".
                    if !self.tokenizer.eof() {
                        let right = self.recursive_parse_binary(elem, his_prec)?;
                        wrap.successors.push(Box::new(right));
                    }
                    else {
                        wrap.successors.push(Box::new(elem));
                    }
                }

                // Check for further binary operations.
                let ret: TreeNode<Token>;
                if !self.tokenizer.eof() {
                    ret = self.recursive_parse_binary(wrap, my_prec)?;
                }
                    else {
                    ret = wrap;
                }

                return Ok(ret);
            }
        }

        Ok(left)
    }

    /// Parses an unary expression.
    fn recursive_parse_unary(& mut self, left: TreeNode<Token>) -> Result<TreeNode<Token>, ParseError> {

        let t = self.parse_element()?;
        let mut m_left = left;
        let t_type = t.content.get_type();

        // Now, "t" can either be an operand (number, constant or function call) or
        // an unary expression (meaning a modified operand)
        if t_type == TokenType::Operation && t.successors.len() == 0 {
            if self.context.is_unary_operation(t.content.get_value()) {

                let unary = self.recursive_parse_unary(t)?;
                m_left.successors.push(Box::new(unary));
                Ok(m_left)
            }
            else {
                Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation",
                                                                Some(format!("non-unary operation \"{0}\"", t.content)), t.content.get_end_pos())))
            }
        }
        else if t_type == TokenType::Number(NumberType::Real) || t_type == TokenType::Number(NumberType::Complex) ||
            t_type == TokenType::Constant || t_type == TokenType::Function || t_type == TokenType::UserConstant || t.successors.len() > 0 {

            m_left.successors.push(Box::new(t));
            Ok(m_left)
        }
        else if t_type == TokenType::Symbol(SymbolicTokenType::UnknownConstant) {
            Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation or operand",
                                                            Some(format!("undefined constant \"{0}\"", t.content)), t.content.get_end_pos())))
        }
        else if t_type == TokenType::Symbol(SymbolicTokenType::UnknownFunction) {
            Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation or operand",
                                                            Some(format!("undefined function \"{0}\"", t.content)), t.content.get_end_pos())))
        }
        else {
            Err(ParseError::from(ExpectedErrorTemplate::new(self.tokenizer.get_input(), "unary operation or operand",
                                                            Some(format!("unexpected symbol \"{0}\"", t.content)), t.content.get_end_pos())))
        }
    }
}
