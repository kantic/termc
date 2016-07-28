use super::get_result;
use super::math_context::MathContext;

#[test]
fn tst_get_result() {
    let context = MathContext::new();

    let result = get_result("1+1", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 2.0 < 10e-10);

    let result = get_result("1+cos(pi)*8", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() + 7.0 < 10e-10);

    let result = get_result("exp(-1/8)+tan(1545.56464-pi*3)^sqrt(4)", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.892351383 < 10e-10);

    let result = get_result("(-1*-1+1*e^(5/7))/(cos(pi/7)*8+2)", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.3304527988 < 10e-10);

    let result = get_result("5^-2", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.04 < 10e-10);

    let result = get_result("6*--2", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 12.0 < 10e-10);

    let result = get_result("+15.7^+--+-0.5", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.2523772326 < 10e-10);

    let result = get_result("tanh(.2)", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.197375320 < 10e-10);

    let result = get_result("tan(pi/3)", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 1.732050807 < 10e-10);

    let result = get_result("sin(0.7)", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 0.644217687 < 10e-10);

    let result = get_result("exp(ln(3))", & context);
    assert!(result.is_ok());
    assert!(result.ok().unwrap() - 3.0 < 10e-10);
}