pub mod tokenizer;

use std::fmt;
use std::error::Error;
use parser::tokenizer::{Tokenizer, TokenType, TokenError, Token};
use parser::tokenizer::input_stream::StreamEndError;
use math_context::MathContext;
use tree::TreeNode;

/// Defines the errors that may occur when parsing the user input string.
#[derive(Debug)]
pub enum ParseError {
    /// Error if the current token does not match the expected token.
    ExpectedError(String, String),
    /// General input error of the user input (syntax error).
    InputError(TokenError)
}

impl fmt::Display for ParseError {

    /// Returns the formatted error message.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::ExpectedError(ref symbol, ref location) => write!(f, "Error: Expected {}.\n{}",
                                                                  & symbol, & location),
            ParseError::InputError(ref e) => write!(f, "{}", e)
        }
    }
}

impl From<TokenError> for ParseError {

    /// Converts a TokenError into a ParseError.
    fn from(err: TokenError) -> ParseError {
        match err {
            TokenError::StreamEndError(ref e) => ParseError::InputError(TokenError::StreamEndError(e.clone())),
            TokenError::UnknownTokenError(ref t, ref l) => ParseError::InputError(TokenError::UnknownTokenError(t.clone(), l.clone()))
        }
    }
}

impl Error for ParseError {

    /// Returns the description of the error.
    fn description(&self) -> &str {
        match *self {
            ParseError::ExpectedError(_,_) => "Expected a symbol.",
            ParseError::InputError(ref err) => err.description()
        }
    }

    /// Returns the preceding error.
    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::ExpectedError(_,_) => None,
            ParseError::InputError(ref err) => Some(err)
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
    fn is_punc(&self, s: &str) -> bool {
        let token = match self.tokenizer.peek() {
            Ok(t) => t,
            Err(_) => return false
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
    fn skip_punc(&mut self, s: &str) -> Result<(), ParseError> {
        if self.is_punc(s) {
            try!(self.tokenizer.next());
            Ok(())
        }
        else {
            match self.tokenizer.peek() {
                Ok(t) => Err(ParseError::ExpectedError(format!("symbol \"{}\"", s),
                                          self.tokenizer.get_err_string(self.tokenizer.get_pos() + 1,
                                                                        & format!("Found: symbol {}", t.get_value())))),
                Err(e) => {
                    match e {
                        TokenError::StreamEndError(_) => Err(ParseError::ExpectedError(format!("symbol \"{}\"", s),
                                                                                       self.tokenizer.get_err_string(self.tokenizer.get_pos() + 1,
                                                                                                                     ""))),
                        TokenError::UnknownTokenError(t, _) => Err(ParseError::ExpectedError(format!("symbol \"{}\"", s),
                                                                                             self.tokenizer.get_err_string(self.tokenizer.get_pos() + 1,
                                                                                                                           & format!("Found: {}.", t))))
                    }
                }
            }
        }
    }

    /// Starts parsing the user input.
    pub fn parse_toplevel(&mut self) -> Result<TreeNode<Token>, ParseError> {

        let result = self.parse_expression();
        if result.is_err() {
            result
        }
        else {
            // we should have read the whole expression
            if !self.tokenizer.eof() {
                Err(ParseError::InputError(TokenError::StreamEndError(StreamEndError {})))
            }
                else {
                result
            }
        }
    }

    /// Parses an element of the expression. This can be one of the following:
    /// (1) a whole expression in parenthesis
    /// (2) an operand (number, constant or function call)
    /// (2) an unary operation (that is not further processed)
    fn parse_element(&mut self) -> Result<TreeNode<Token>, ParseError> {

        if self.is_punc("(") {
            // expression in parenthesis
            try!(self.tokenizer.next());
            let exp = try!(self.parse_expression());
            try!(self.skip_punc(")"));
            return Ok(exp);
        }
        else {
            let pos = self.tokenizer.get_pos();
            let t = try!(self.tokenizer.next());
            let token_type = t.get_type();

            match token_type {
                TokenType::Number => {
                    Ok(TreeNode::new(Token::new(TokenType::Number, t.get_value())))
                },
                TokenType::Constant => {
                    Ok(TreeNode::new(Token::new(TokenType::Constant, t.get_value())))
                },
                TokenType::Function => {
                    // return the complete parsed function call subtree
                    self.parse_function(t)
                },
                TokenType::Operation => {
                    // Return the unprocessed unary operation symbol.
                    // This case is relevant as an unary operation can appear instead of an operand
                    // (e.g. "3+-2" (= 1), where after the "+" operation an operand is expected,
                    // but the unary "-" operation appears)
                    if self.context.is_unary_operation(t.get_value()) {
                        Ok(TreeNode::new(Token::new(TokenType::Operation, t.get_value())))
                    }
                    else {
                        Err(ParseError::ExpectedError(String::from("unary operation"),
                                                      self.tokenizer.get_err_string(pos, & format!(
                                                          "Found: non-unary operation \"{}\".", t.get_value()))))
                    }
                }
                _ => {
                    Err(ParseError::ExpectedError(String::from("operand (number, constant, function call) or an unary operation"),
                                                  self.tokenizer.get_err_string(pos, & format!("Found: unexpected symbol \"{}\".", t.get_value()))))
                }
            }
        }
    }

    /// Parses an expression.
    fn parse_expression(&mut self) -> Result<TreeNode<Token>, ParseError> {

        self.parse_operation()
    }

    /// Parses a function call.
    fn parse_function(&mut self, t: Token) -> Result<TreeNode<Token>, ParseError> {

        try!(self.skip_punc("("));
        let expr = try!(self.parse_expression());
        try!(self.skip_punc(")"));

        let mut ret = TreeNode::new(t);
        ret.successors.push(Box::new(expr)); // add the function argument as a subtree to the function
        Ok(ret)
    }

    /// Parses an operation expression tree while making the distinction between unary and binary
    /// operations (e.g. "-3*-5" is an unary expression ("-3") followed by an binary operation
    /// ("<...>+<...>") followed by an unary expression ("-5").
    fn parse_operation(&mut self) -> Result<TreeNode<Token>, ParseError> {

        let pos = self.tokenizer.get_pos();
        let elem = try!(self.parse_element());

        if !self.tokenizer.eof() {
            if elem.content.get_type() == TokenType::Operation && elem.successors.len() == 0 {

                // If elem is an operation, it must be a unary operation, otherwise it would be a syntax error.
                // Hence, if the unary operation has not yet been parsed (that means, the unary
                // expression has no operand yet), it does not have any successors.
                if self.context.is_unary_operation(elem.content.get_value()) {
                    let unary = try!(self.recursive_parse_unary(elem));

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
                    Err(ParseError::ExpectedError(String::from("unary operation"),
                                                  self.tokenizer.get_err_string(pos, & format!("Found: non-unary operation \"{}\".",
                                                                                               elem.content.get_value()))))
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
    fn recursive_parse_binary(&mut self, left: TreeNode<Token>, my_prec: u32) -> Result<TreeNode<Token>, ParseError> {

        // The argument "left" must be an operand (number constant or function call) or an unary expression
        // (that is interpreted also as a modified operand).
        let t = try!(self.tokenizer.peek());
        if t.get_type() == TokenType::Operation {
            let his_prec = self.context.get_operation_precedence(t.get_value()).unwrap();
            if his_prec > my_prec {
                let t = try!(self.tokenizer.next());
                let mut wrap = TreeNode::new(t);
                // "left" is the left operand of the binary operation "t", so add id
                wrap.successors.push(Box::new(left));
                let pos = self.tokenizer.get_pos();
                let elem = try!(self.parse_element());

                // Now, "elem" can either be an operand (number, constant or function call) or
                // an unary expression (meaning a modified operand)
                if elem.content.get_type() == TokenType::Operation && elem.successors.len() == 0 {
                    // If "elem" is an unparsed (=> no successors) unary operation, parse it
                    if self.context.is_unary_operation(elem.content.get_value()) {

                        // the unary expression is the right operand of the binary operation "t"
                        let unary = try!(self.recursive_parse_unary(elem));
                        wrap.successors.push(Box::new(unary));
                    }
                    else {
                        return Err(ParseError::ExpectedError(String::from("unary operation"),
                                                             self.tokenizer.get_err_string(pos, & format!("Found: non-unary operation \"{}\".",
                                                                                                          elem.content.get_value()))));
                    }
                }
                else {
                    // "elem" must be an operand or an parsed unary expression.
                    // Check for further operations with higher precedence than "t".
                    if !self.tokenizer.eof() {
                        let right = try!(self.recursive_parse_binary(elem, his_prec));
                        wrap.successors.push(Box::new(right));
                    }
                    else {
                        wrap.successors.push(Box::new(elem));
                    }
                }

                // Check for further binary operations.
                let ret: TreeNode<Token>;
                if !self.tokenizer.eof() {
                    ret = try!(self.recursive_parse_binary(wrap, my_prec));
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
    fn recursive_parse_unary(&mut self, left: TreeNode<Token>) -> Result<TreeNode<Token>, ParseError> {

        let pos = self.tokenizer.get_pos();
        let t = try!(self.parse_element());
        let mut m_left = left;
        let t_type = t.content.get_type();

        // Now, "t" can either be an operand (number, constant or function call) or
        // an unary expression (meaning a modified operand)
        if t_type == TokenType::Operation && t.successors.len() == 0 {
            if self.context.is_unary_operation(t.content.get_value()) {

                let unary = try!(self.recursive_parse_unary(t));
                m_left.successors.push(Box::new(unary));
                Ok(m_left)
            }
            else {
                Err(ParseError::ExpectedError(String::from("unary operation"),
                                              self.tokenizer.get_err_string(pos, & format!("Found: non-unary operation \"{}\".",
                                                                                           t.content.get_value()))))
            }
        }
        else if t_type == TokenType::Number || t_type == TokenType::Constant || t_type == TokenType::Function || t.successors.len() > 0 {

            m_left.successors.push(Box::new(t));
            Ok(m_left)
        }
        else {
            Err(ParseError::ExpectedError(String::from("unary operation or operand"),
                                          self.tokenizer.get_err_string(pos, & format!("Found: unexpected symbol \"{}\".",
                                                                                       t.content.get_value()))))
        }
    }
}