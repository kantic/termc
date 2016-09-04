
use std::fmt;
use std::error::Error;
use parser::ParseError;
use evaluator::EvaluationError;

/// Represents the errors that may occur in the whole process of parsing and evaluating.
#[derive(Clone, Debug)]
pub enum ResultError {
    /// Represents the errors that may occur in the parsing process
    /// Arguments: ParseError
    ParseError(ParseError),
    /// Represents the errors that may occur in the evaluation process
    /// Arguments: EvaluationError
    EvaluationError(EvaluationError),
}

impl fmt::Display for ResultError {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResultError::ParseError(ref p) => write!(f, "{}", p),
            ResultError::EvaluationError(ref e) => write!(f, "{}", e)
        }
    }
}

impl From<ParseError> for ResultError {

    /// Converts a ParseError into a ResultError.
    fn from(p: ParseError) -> ResultError {
        ResultError::ParseError(p)
    }
}

impl From<EvaluationError> for ResultError {

    /// Converts a EvaluationError into a ResultError.
    fn from(e: EvaluationError) -> ResultError {
        ResultError::EvaluationError(e)
    }
}

impl Error for ResultError {

    /// Returns the description of the error.
    fn description(& self) -> & str {
        match *self {
            ResultError::ParseError(_) => "The user input could not be parsed.",
            ResultError::EvaluationError(_) => "The evaluation of the user input failed."
        }
    }

    /// Returns the preceding error.
    fn cause(& self) -> Option<& Error> {
        match *self {
            ResultError::ParseError(ref p) => Some(p),
            ResultError::EvaluationError(ref e) => Some(e)
        }
    }
}