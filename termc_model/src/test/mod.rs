
use std::f64;
use super::get_result;
use math_context::MathContext;
use token::NumberType;

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

    // test binary operation "^"
    let result = get_result("25^0.5", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 5.0 < TEST_BOUND);

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

    // test sinh function
    let result = get_result("sinh(pi+9/3)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 232.395542404 < TEST_BOUND);

    // test tanh function
    let result = get_result("tanh(e)", & mut context);
    assert!(result.is_ok());
    let result = result.ok().unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert!(result.result_type == NumberType::Real);
    assert!(result.value.re - 0.991328915 < TEST_BOUND);

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

    // Error message tests

    // test missing ")"
    /*
    let result = get_result("2*(5-3", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected symbol \")\".\n2*(5-3\n      ^~~~ ");



    // test unknown function
    let result = get_result("3-cis(pi/2)+sin(0)", & context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Unknown token found: \"cis(...)\".\n3-cis(pi/2)+sin(0)\n    ^~~~ ");

    // test unknown constant
    let result = get_result("5*3+cos(py)-7^1", & context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Unknown token found: \"py\".\n5*3+cos(py)-7^1\n         ^~~~ ");


    // test expectation of unary operation
    let result = get_result("5+--*2.7", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected unary operation.\n5+--*2.7\n    ^~~~ Found: non-unary operation \"*\".");

    // test expectation of unary operation or operand
    let result = get_result("3-)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected operand (number, constant, function call) or an unary operation.\n3-)\n  ^~~~ Found: unexpected symbol \")\".");

    // test unexpected token
    let result = get_result("5+|", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    println!("Error-msg: {}", msg);
    assert!(msg == "Error: Unknown token found: \"|\".\n5+|\n  ^~~~ ");

    // test expectation of ")" in argument list
    let result = get_result("pow(5,", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected symbol \")\".\npow(5,\n      ^~~~ ");

    // test argument number for functions
    let result = get_result("pow(5)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected 2 argument(s).\npow(5)\n     ^~~~ ");

    // test expectation of argument in function argument list
    let result = get_result("pow(5,)", & mut context);
    assert!(result.is_err());
    let msg = format!("{}", result.err().unwrap());
    assert!(msg == "Error: Expected an argument.\npow(5,)\n      ^~~~ Found: symbol \")\".");
    */
}