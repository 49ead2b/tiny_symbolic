use tiny_symbolic::Variable;
use tiny_symbolic::algos::ElementarySymmetricPolynomials;

fn main() {
    let x = Variable::new('x', None);
    let y = Variable::new('y', None);
    let expr = x.pow(3) + 2 * x.pow(2) * y.pow(-3) - x + 7;

    let substituted = expr
        .clone()
        .substitute_multiple_variables_with_expressions([(x, 3)]);
    println!("{}", substituted); // Will output 31 + 18y⁻³

    let substituted = expr
        .clone()
        .substitute_multiple_variables_with_expressions([(x, 3), (y, -7)]);
    println!("{}", substituted); // Will output (10615/343)

    let a = Variable::new('A', None);
    let b = Variable::new('B', None);
    let c = Variable::new('C', None);
    let w = Variable::new('W', None); // Suppose W is the cube root of unity

    let expr = (a + b + c) * (a + w * b + w.pow(2) * c) * (a + w.pow(2) * b + w * c);
    let substituted = expr
        .substitute_variable_power_with_expression(w, 3, 1_i64.into())
        .substitute_variable_power_with_expression(w, 2, -1 - w);

    println!("{substituted}"); //Will output -3ABC + A³ + B³ + C³

    let mut esp_engine = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);
    let expr_as_esp = esp_engine.simplify_symmetric_expression(substituted);

    println!("{expr_as_esp}"); //Will output -3e₁e₂ + e₁³
}
