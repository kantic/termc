use std::f64;
use std::collections::{HashMap, HashSet};

/// Defines the types of supported operations.
#[derive(Clone, Copy, PartialEq)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Pow,
    Div
}

/// Defines the types of supported built-in functions.
#[derive(Clone, Copy, PartialEq)]
pub enum FunctionType {
    Cos,
    Sin,
    Tan,
    Exp,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Ln
}

/// Defines the mathematical context.
pub struct MathContext {
    /// Map of supported operations.
    operations: HashMap<String, (OperationType, u32)>,
    /// Set of symbols representing numbers.
    number_symbols: HashSet<char>,
    /// Set of symbols representing words.
    literals : HashSet<char>,
    /// Set of functions.
    functions: HashMap<String, FunctionType>,
    /// Map of supported constants.
    constants : HashMap<String, f64>,
    /// Set of punctuation symbols.
    punctuation : HashSet<char>
}

impl<'a> MathContext {

    /// Creates a new instance of type MathContext.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// ```
    pub fn new() -> MathContext {
        let number_symbols: HashSet<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
            .into_iter().collect();

        // all literal symbols with which function names or constant names can start with
        // e.g. "pi" or "c0", but now allowed is starting with a number like "0c"
        let literals: HashSet<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
        'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C',
        'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
        'V', 'W', 'X', 'Y', 'Z', '_'].into_iter().collect();

        // define the operation types associated with their string representation
        let mut operations: HashMap<String, (OperationType, u32)> = HashMap::new();
        operations.insert(String::from("+"), (OperationType::Add, 1));
        operations.insert(String::from("-"), (OperationType::Sub, 1));
        operations.insert(String::from("*"), (OperationType::Mul, 2));
        operations.insert(String::from("/"), (OperationType::Div, 2));
        operations.insert(String::from("^"), (OperationType::Pow, 3));

        // defines functions types with associated with their string representation
        let mut functions: HashMap<String, FunctionType> = HashMap::new();
        functions.insert(String::from("cos"), FunctionType::Cos);
        functions.insert(String::from("cosh"), FunctionType::Cosh);
        functions.insert(String::from("sin"), FunctionType::Sin);
        functions.insert(String::from("sinh"), FunctionType::Sinh);
        functions.insert(String::from("tan"), FunctionType::Tan);
        functions.insert(String::from("tanh"), FunctionType::Tanh);
        functions.insert(String::from("exp"), FunctionType::Exp);
        functions.insert(String::from("sqrt"), FunctionType::Sqrt);
        functions.insert(String::from("ln"), FunctionType::Ln);

        // defines constants
        let mut constants: HashMap<String, f64> = HashMap::new();
        constants.insert(String::from("pi"), f64::consts::PI);
        constants.insert(String::from("e"), f64::consts::E);

        let mut punctuation: HashSet<char> = HashSet::new();
        punctuation.insert('(');
        punctuation.insert(')');

        MathContext {
            operations: operations, number_symbols: number_symbols, literals: literals,
            functions: functions, constants: constants, punctuation: punctuation
        }
    }

    /// Checks whether the specified string is an operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_op = context.is_operation("+");
    /// assert!(is_op == true);
    /// ```
    pub fn is_operation(&self, s: &'a str) -> bool {
        self.operations.contains_key(s)
    }

    /// Checks whether the specified string is an unary operation.
    /// An unary operation is an operation that may take only one operand, e.g. "-3", where the
    /// "-" has only one operand "3".
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_op = context.is_unary_operation("-");
    /// assert!(is_op == true);
    /// let is_op = context.is_unary_operation("*");
    /// assert!(is_op == false);
    /// ```
    pub fn is_unary_operation(&self, s: &'a str) -> bool {
        match self.get_operation_type(s) {
            Some(x) => {
                if x == OperationType::Add || x == OperationType::Sub {
                    true
                }
                else {
                    false
                }
            },
            None => false
        }
    }

    /// Checks whether the specified string is a function.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_func = context.is_function("cos");
    /// assert!(is_func == true);
    /// ```
    pub fn is_function(&self, s: &'a str) -> bool {
        self.functions.contains_key(s)
    }

    /// Checks whether the specified character is a number symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_num = context.is_number_symbol(& '3');
    /// assert!(is_num == true);
    /// ```
    pub fn is_number_symbol(&self, c: & char) -> bool {
        self.number_symbols.contains(c)
    }

    /// Checks whether the specified character is a literal symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_literal = context.is_literal_symbol(& 'f');
    /// assert!(is_literal == true);
    /// ```
    pub fn is_literal_symbol(&self, c: & char) -> bool {
        self.literals.contains(c)
    }

    /// Check whether the specified string is a constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_constant = context.is_constant("pi");
    /// assert!(is_constant == true);
    /// ```
    pub fn is_constant(&self, s: &'a str) -> bool {
        self.constants.contains_key(s)
    }

    /// Checks whether the specified character is a punctuation symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let is_punc = context.is_punctuation_symbol(& '(');
    /// assert!(is_punc == true);
    /// ```
    pub fn is_punctuation_symbol(&self, c: & char) -> bool {
        self.punctuation.contains(c)
    }

    /// Returns the value of the specified constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    /// use std::f64;
    ///
    /// let context = MathContext::new();
    /// let const_val = context.get_constant_value("pi");
    /// assert!(const_val.is_some());
    /// let const_val = const_val.unwrap();
    /// assert!(const_val - f64::consts::PI < 10e-10);
    ///
    /// let const_val = context.get_constant_value("e");
    /// assert!(const_val.is_some());
    /// let const_val = const_val.unwrap();
    /// assert!(const_val - f64::consts::E < 10e-10);
    /// ```
    pub fn get_constant_value(&self, s: & str) -> Option<f64> {
        self.constants.get(s).cloned()
    }

    /// Gets the operation type of the specified operation string.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::{MathContext, OperationType};
    ///
    /// let context = MathContext::new();
    /// let op_type = context.get_operation_type("+");
    /// assert!(op_type == Some(OperationType::Add));
    /// ```
    pub fn get_operation_type(&self, s: &'a str) -> Option<OperationType> {
        match self.operations.get(s) {
            Some(x) => Some(x.0),
            None => None
        }
    }

    /// Returns the precedence of the specified operation string.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::MathContext;
    ///
    /// let context = MathContext::new();
    /// let op_prec = context.get_operation_precedence("+");
    /// assert!(op_prec == Some(1 as u32));
    /// ```
    pub fn get_operation_precedence(&self, s: &'a str) -> Option<u32> {
        match self.operations.get(s) {
            Some(x) => Some(x.1),
            None => None
        }
    }

    /// Gets the function type of the specified function string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use termc_model::math_context::{MathContext, FunctionType};
    ///
    /// let context = MathContext::new();
    /// let func_type = context.get_function_type("cosh");
    /// assert!(func_type == Some(FunctionType::Cosh));
    /// ```
    pub fn get_function_type(&self, s: &'a str) -> Option<FunctionType> {
        self.functions.get(s).cloned()
    }
}