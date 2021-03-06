use std::f64;
use serde_json;
use super::get_result;
use math_context::MathContext;
use token::{NumberType, TokenType, SymbolicTokenType, Token};
use tree::TreeNode;
use math_result::MathResult;

static TEST_BOUND : f64 = 10e-10;

#[test]
fn tst_get_result() {
    let mut context = MathContext::new();

    // Basic tests

    // test ordinary real numbers
    let result = get_result("55.78", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 55.78 < TEST_BOUND);

    // test number starting with decimal point
    let result = get_result(".09878", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.09878 < TEST_BOUND);

    // test number in scientific notation
    let result = get_result("-.48E7i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.0 < TEST_BOUND);
    assert!(result.value.im + 4800000.0 < TEST_BOUND);

    // test number in binary notation
    let result = get_result("0b101", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 5.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0b11i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.0 < TEST_BOUND);
    assert!(result.value.im - 3.0 < TEST_BOUND);

    let result = get_result("-0b1010.11-0b11.1i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re + 10.75 < TEST_BOUND);
    assert!(result.value.im + 3.5 < TEST_BOUND);

    let result = get_result("-0b.1+0b.i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 0.5 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0b111.", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 7.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    // test number in octal notation
    let result = get_result("0o775", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 509.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0o457i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.0 < TEST_BOUND);
    assert!(result.value.im - 303.0 < TEST_BOUND);

    let result = get_result("-0o124.73-0o1.1i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re + 83.921875 < TEST_BOUND);
    assert!(result.value.im + 1.125 < TEST_BOUND);

    let result = get_result("-0o.3+0o.i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 0.375 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0o111.", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 73.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    // test number in hexadecimal notation
    let result = get_result("0xa1b", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 2587.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0xabci", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.0 < TEST_BOUND);
    assert!(result.value.im - 2748.0 < TEST_BOUND);

    let result = get_result("-0x92.7c-0x3.00bi", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re + 146.484375 < TEST_BOUND);
    assert!(result.value.im + 3.002685546875 < TEST_BOUND);

    let result = get_result("-0x.e+0x.i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 0.875 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    let result = get_result("0xf.", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 15.0 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    // test constant pi
    let result = get_result("pi", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::PI < TEST_BOUND);

    // test constant e
    let result = get_result("e", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::E < TEST_BOUND);

    // (2) complex numbers

    // Unary expression tests

    // test unary operation "-"
    let result = get_result("-1", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 1.0 < TEST_BOUND);

    // test unary operation "+"
    let result = get_result("+11.1", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 11.1 < TEST_BOUND);

    // test unary expression with number starting with decimal point
    let result = get_result("+.111", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.111 < TEST_BOUND);

    // test unary expression with constant
    let result = get_result("--+-e", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + f64::consts::E < TEST_BOUND);

    // test unary expression with user constant
    let result = get_result("x = -15", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_none());

    let result = get_result("--+-x", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 15.0_f64 < TEST_BOUND);
     // reset context
    let mut context = MathContext::new();

    // test nested unary expressions
    let result = get_result("---+-2.44", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 2.44 < TEST_BOUND);

    // test nested unary expressions with parenthesis
    let result = get_result("(-(-(-(-999))))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 999.0 < TEST_BOUND);

    // Binary expression tests

    // test binary operation "+"
    let result = get_result("1+1.2", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 2.2 < TEST_BOUND);

    // test binary operation "-"
    let result = get_result("0-23.23", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 23.23 < TEST_BOUND);

    // test binary operation "*"
    let result = get_result("1.2*0.5", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.6 < TEST_BOUND);

    // test binary operation "/"
    let result = get_result("1.0/8.0", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.125 < TEST_BOUND);

    // test binary operation "%"
    let result = get_result("78%43.0", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 35.0 < TEST_BOUND);

    // test binary operation "^"
    let result = get_result("25^0.5", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 5.0 < TEST_BOUND);

    // test binary with higher precedence following a binary operation with lower precedence
    // with operands that have unary opeartions
    let result = get_result("1 + -2 * -1", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 3.0 < TEST_BOUND);

    // test assignment of constant
    let result = get_result("c = e + pi", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_none());
    let c = context.get_constant_value("c");
    assert!(c.is_some());
    let c = c.unwrap();
    let c = c.value.re;
    assert!(c - (f64::consts::PI + f64::consts::E) < TEST_BOUND);
    // reset context
    let mut context = MathContext::new();

    // test assignment of function
    let result = get_result("f(x, y) = x + y", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_none());
    let is_fun = context.is_user_function("f");
    assert!(is_fun == true);
    let result = get_result("f(3, 15.2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 18.2 < TEST_BOUND);
    let result = get_result("f(3+5, arccos(0.7))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 8.795398830 < TEST_BOUND);
    // reset context
    let mut context = MathContext::new();

    // test assignment of existing user function with less arguments
    let _ = get_result("f(x, y) = x + y", & mut context);
    let result = get_result("f(x) = x + 1", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_none());
    let is_fun = context.is_user_function("f");
    assert!(is_fun == true);
    let result = get_result("f(3)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 4.0 < TEST_BOUND);
    // reset context
    let mut context = MathContext::new();

    // test the definition of the ans constant
    // for this test, the context should be reset
    let result = get_result("15-8.78", & mut context);
    assert!(result.is_ok());
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 6.22 < TEST_BOUND);
    let ans = context.get_constant_value("ans");
    assert!(ans.is_some());
    let ans = ans.unwrap();
    assert!(ans.value.re - 6.22 < TEST_BOUND);


    // test chained binary operations
    let result = get_result("24*74+9^1.55-88/3", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1776.801992365 < TEST_BOUND);

    // Parenthesis tests

    // test priority gain with parenthesis
    let result = get_result("12*(1.0+2.7)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 44.4 < TEST_BOUND);

    // test parenthesis at start of expression
    let result = get_result("(25+3)/-7", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 4.0 < TEST_BOUND);

    // Function tests

    // test cos function
    let result = get_result("cos(0.4)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.921060994 < TEST_BOUND);

    // test sin function
    let result = get_result("sin(pi/2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.0 < TEST_BOUND);

    // test tan function
    let result = get_result("tan(0.45)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.483055065 < TEST_BOUND);

    // test cot function
    let result = get_result("cot(7)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.147515422 < TEST_BOUND);

    // test acos function (real)
    let result = get_result("acos(cos(pi))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::PI < TEST_BOUND);

    // test acos function (complex)
    let result = get_result("acos(45)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.0 < TEST_BOUND);
    assert!(result.value.im - 4.499686190 < TEST_BOUND);

    // test asin function (real)
    let result = get_result("asin(sin(pi/3))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::PI / 3.0 < TEST_BOUND);

    // test asin function (complex)
    let result = get_result("asin(45)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - f64::consts::PI / 2.0 < TEST_BOUND);
    assert!(result.value.im + 4.499686190 < TEST_BOUND);

    // test atan function
    let result = get_result("atan(tan(pi/7))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::PI / 7.0 < TEST_BOUND);

    // test acot function
    let result = get_result("acot(cot(pi/4))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::PI / 4.0 < TEST_BOUND);

    // test cosh function
    let result = get_result("cosh(0.7897)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.328358237 < TEST_BOUND);

    // test arccosh function
    let result = get_result("arccosh(1.7897)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.186000090 < TEST_BOUND);

    // test sinh function
    let result = get_result("sinh(pi+9/3)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 232.395542404 < TEST_BOUND);

    // test arcsinh function
    let result = get_result("arcsinh(0.5)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.481211825 < TEST_BOUND);

    // test tanh function
    let result = get_result("tanh(e)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.991328915 < TEST_BOUND);

    // test arctanh function
    let result = get_result("arctanh(-0.233)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 0.237359350 < TEST_BOUND);

    // test coth function
    let result = get_result("coth(0.887)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.408631623 < TEST_BOUND);

    // test arccoth function
    let result = get_result("arccoth(-1.7)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 0.674963358 < TEST_BOUND);

    // test exp function
    let result = get_result("exp(1)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - f64::consts::E < TEST_BOUND);

    // test ln function
    let result = get_result("ln(e^87)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 87.0 < TEST_BOUND);

    // test pow function
    let result = get_result("pow(5, 2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 25.0 < TEST_BOUND);

    // test root function
    let result = get_result("root(25, 2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 5.0 < TEST_BOUND);

    // test im function
    let result = get_result("im(57)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    // test re function
    let result = get_result("re(-57-9i)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 57.0 < TEST_BOUND);

    // test nested functions
    let result = get_result("cos(exp(0.5)+pi/2*ln(2))-root(1, 2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 1.919465158 < TEST_BOUND);

    // Convoluted expression tests

    let result = get_result("1+cos(pi)*8", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 7.0 < TEST_BOUND);

    let result = get_result("exp(-1/8)+tan(1545.56464-pi*3)^sqrt(4)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.892351383 < TEST_BOUND);

    let result = get_result("(-1*-1+1*e^(5/7))/(cos(pi/7)*8+2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.3304527988 < TEST_BOUND);

    let result = get_result("5^-2", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.04 < TEST_BOUND);

    let result = get_result("6*--2", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 12.0 < TEST_BOUND);

    let result = get_result("+15.7^+--+-0.5", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.2523772326 < TEST_BOUND);

    let result = get_result("tanh(.2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.197375320 < TEST_BOUND);

    let result = get_result("tan(pi/3)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 1.732050807 < TEST_BOUND);

    let result = get_result("sin(0.7)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.644217687 < TEST_BOUND);

    let result = get_result("exp(ln(3))", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 3.0 < TEST_BOUND);

    let result = get_result("pi - 9 / 2 ^- 0.7", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re + 11.478950480 < TEST_BOUND);

    // Complex number tests

    // test ordinary complex number
    let result = get_result("0.5 - 1.8i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.5 < TEST_BOUND);
    assert!(result.value.im + 1.8 < TEST_BOUND);

    // test complex number starting with "."
    let result = get_result(".458+.97i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.458 < TEST_BOUND);
    assert!(result.value.im - 0.97 < TEST_BOUND);

    // test result of a expression to be complex if any operand is complex
    let result = get_result("(-1*-1+1*e^(5/7))/(cos(pi/(7+2i))*8+2)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 0.324096360 < TEST_BOUND);
    assert!(result.value.im + 0.013251332 < TEST_BOUND);

    // test result of a expression to be complex if any operand is complex
    let result = get_result("sinh(3) - cos(pi/e) + .5i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Complex);
    assert!(result.value.re - 9.614621876 < TEST_BOUND);
    assert!(result.value.im - 0.5 < TEST_BOUND);

    // test result of an expression with complex numbers to be real if the result of the imaginary part is zero
    let result = get_result("5+3i -7i +78 +4i", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 83.0_f64 < TEST_BOUND);
    assert!(result.value.im - 0.0 < TEST_BOUND);

    // Error message tests

    // test missing ")"
    let result = get_result("2*(5-3", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected symbol \")\".\n2*(5-3\n      ^~~~");

    // test unknown function
    let result = get_result("3-cis(pi/2)+sin(0)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected built-in or user defined function.\n3-cis(pi/2)+sin(0)\n    ^~~~ Found: unknown function \"cis(...)\"");

    // test unknown constant
    let result = get_result("5*3+cos(py)-7^1", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected built-in or user defined constant.\n5*3+cos(py)-7^1\n         ^~~~ Found: unknown constant \"py\"");

    // test expectation of unary operation
    let result = get_result("5+--*2.7", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected unary operation.\n5+--*2.7\n    ^~~~ Found: non-unary operation \"*\"");


    // test expectation of unary operation or operand
    let result = get_result("3-)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected operand (number, constant, function call) or an unary operation.\n3-)\n  ^~~~ Found: unexpected symbol \")\"");


    // test unexpected token
    let result = get_result("5+|", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    println!("Error-msg: {}", msg);
    assert!(msg == "Error: Unknown token found: \"|\".\n5+|\n  ^~~~");


    // test expectation of ")" in argument list
    let result = get_result("pow(5,", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected symbol \")\".\npow(5,\n      ^~~~");


    // test argument number error for functions
    let result = get_result("pow(5)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected 2 argument(s).\npow(5)\n  ^~~~ Found: 1 argument(s)");


    // test expectation of argument in function argument list
    let result = get_result("pow(5,)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected an argument.\npow(5,)\n      ^~~~ Found: symbol \")\"");

    // test expectation of "," or ")" in a function argument list
    let result = get_result("sqrt(4, 3 % 5.000000000000 01)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected \",\" or \")\".\nsqrt(4, 3 % 5.000000000000 01)\n                            ^~~~ Found: \"01\"");

    // test expectation of non-built-in constant when a user constant is defined
    let result = get_result("pi = 5", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected new constant name or function name.\npi = 5\n ^~~~ Found: built-in expression \"pi\"");

    // test expectation error for recursive user function definition
    let result = get_result("z(x) = z(x) + 2", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected non-symbolic expression.\nz(x) = z(x) + 2\n       ^~~~ Found: symbolic expression \"z\"");
    // reset context
    let mut context = MathContext::new();

    // test definition and use of function with wrong (symbolical) content
    let result = get_result("y(x) = z", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected non-symbolic expression.\ny(x) = z\n       ^~~~ Found: symbolic expression \"z\"");
    // reset context
    let mut context = MathContext::new();

    // test definition of user function with equal arguments
    let result = get_result("h(x, y, x) = x^2+y", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected distinct arguments.\nh(x, y, x) = x^2+y\n^~~~ Found: function definition with partly equal arguments");
    let mut context = MathContext::new();

    // test wrong digit in binary number
    let result = get_result("0b10201", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected literal number.\n0b10201\n      ^~~~ Found: Invalid literal symbol(s)");

    // test wrong digit in binary number
    let result = get_result("0o43927", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected literal number.\n0o43927\n      ^~~~ Found: Invalid literal symbol(s)");

    // test wrong digit in hexadecimal number
    let result = get_result("0x25a3u", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected literal number.\n0x25a3u\n      ^~~~ Found: Invalid literal symbol(s)");
}

#[test]
fn tst_deserialization() {
    // test deserialization of NumberType
    let n_type : Result<NumberType, serde_json::Error> = serde_json::from_str("\"Real\"");
    assert!(n_type.is_ok());
    let n_type = n_type.ok().unwrap();
    assert!(n_type == NumberType::Real);

    let n_type : Result<NumberType, serde_json::Error> = serde_json::from_str("\"Complex\"");
    assert!(n_type.is_ok());
    let n_type = n_type.ok().unwrap();
    assert!(n_type == NumberType::Complex);

    // test deserialization of SymbolicTokenType
    let s_type : Result<SymbolicTokenType, serde_json::Error> = serde_json::from_str("\"UnknownConstant\"");
    assert!(s_type.is_ok());
    let s_type = s_type.ok().unwrap();
    assert!(s_type == SymbolicTokenType::UnknownConstant);

    let s_type : Result<SymbolicTokenType, serde_json::Error> = serde_json::from_str("\"UnknownFunction\"");
    assert!(s_type.is_ok());
    let s_type = s_type.ok().unwrap();
    assert!(s_type == SymbolicTokenType::UnknownFunction);

    // test deserialization of TokenType
    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("{ \"Number\": \"Real\" }");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Number(NumberType::Real));

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"Constant\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Constant);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"UserConstant\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::UserConstant);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"Function\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Function);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"UserFunction\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::UserFunction);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"Operation\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Operation);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"Punctuation\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Punctuation);

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("{ \"Symbol\": \"UnknownFunction\"}");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::Symbol(SymbolicTokenType::UnknownFunction));

    let t_type : Result<TokenType, serde_json::Error> = serde_json::from_str("\"FunctionArg\"");
    assert!(t_type.is_ok());
    let t_type = t_type.ok().unwrap();
    assert!(t_type == TokenType::FunctionArg);

    // test deserialization for Token
    let t : Result<Token, serde_json::Error> = serde_json::from_str("{ \"token_type\": \"Constant\", \"value\": \"pi\", \"end_pos\": 15 }");
    assert!(t.is_ok());
    let t = t.ok().unwrap();
    assert!(t.get_type() == TokenType::Constant);
    assert!(t.get_value() == "pi");
    assert!(t.get_end_pos() == 15);

    // test deserialization for TreeNode<Token>
    let t : Result<TreeNode<Token>, serde_json::Error> = serde_json::from_str("{ \"content\": { \"token_type\": \"Constant\", \"value\": \"e\", \"end_pos\": 38 }, \"successors\": [] }");
    assert!(t.is_ok());
    let t = t.ok().unwrap();
    assert!(t.content.get_type() == TokenType::Constant);
    assert!(t.content.get_value() == "e");
    assert!(t.content.get_end_pos() == 38);
    assert!(t.successors.len() == 0);

    let t : Result<TreeNode<Token>, serde_json::Error> = serde_json::from_str("{ \"content\": { \"token_type\": \"Constant\", \"value\": \"e\", \"end_pos\": 38 }, \"successors\": [{ \"content\": { \"token_type\": \"Function\", \"value\": \"sin\", \"end_pos\": 2556 }, \"successors\": [] }] }");
    assert!(t.is_ok());
    let t = t.ok().unwrap();
    assert!(t.content.get_type() == TokenType::Constant);
    assert!(t.content.get_value() == "e");
    assert!(t.content.get_end_pos() == 38);
    assert!(t.successors.len() == 1);
    let succ = t.successors[0].to_owned();
    assert!(succ.content.get_type() == TokenType::Function);
    assert!(succ.content.get_value() == "sin");
    assert!(succ.content.get_end_pos() == 2556);
    assert!(succ.successors.len() == 0);

    // test deserialization of MathResult
    let m : Result<MathResult, serde_json::Error> = serde_json::from_str("{ \"result_type\": \"Complex\", \"re\": 4.77, \"im\": 101.897553 }");
    assert!(m.is_ok());
    let m = m.ok().unwrap();
    assert!(m.result_type == NumberType::Complex);
    assert!(m.value.re - 4.77 < TEST_BOUND);
    assert!(m.value.re - 101.897553 < TEST_BOUND);

    // test deserialization of MathContext
    let m : Result<MathContext, serde_json::Error> = serde_json::from_str("{\"user_constants\":{\"c\": {\"result_type\":\"Real\",\"im\":0.0,\"re\":78.99}},\"user_function_inputs\":{\"f\":\"f(x) = x^2\"},\"user_functions\":{\"f\": [{\"content\":{\"end_pos\":8,\"token_type\":\"Operation\",\"value\":\"^\"},\"successors\":[{\"content\":{\"end_pos\":7,\"token_type\":{\"Symbol\":\"UnknownConstant\"},\"value\":\"x\"},\"successors\":[]},{\"content\":{\"end_pos\":9,\"token_type\":{\"Number\":\"Real\"},\"value\":\"2\"},\"successors\":[]}]},[\"x\"]]}}");
    assert!(m.is_ok());
    let m = m.ok().unwrap();
    assert!(m.is_user_constant("c"));
    let c = m.get_constant_value("c");
    assert!(c.is_some());
    let c = c.unwrap();
    assert!(c.result_type == NumberType::Real);
    assert!(c.value.re - 78.99 < TEST_BOUND);
    assert!(m.is_user_function("f"));
    let arg_num = m.get_function_arg_num("f");
    assert!(arg_num.is_some());
    let arg_num = arg_num.unwrap();
    assert!(arg_num == 1);
    let f_input = m.get_user_function_input("f");
    assert!(f_input.is_some());
    let f_input = f_input.unwrap();
    assert!(f_input == "f(x) = x^2");
}
