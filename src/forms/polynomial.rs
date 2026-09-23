use std::fmt::Display;

use crate::{algos::ElementarySymmetricPolynomials, operations::derivative::PartialDerivative, *};

/// A polynomial consisting of expression coefficients and a variable raised to a successive power.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Polynomial {
    pub(super) variable: Variable,
    pub(super) coefficients: Vec<Expression>,
    pub(super) order: usize,
}

impl Polynomial {
    /// Create a Polynomial from a variable and a vector of coefficients.
    /// The coefficients are in order of increasing power of the variable.
    /// Leading coefficient cannot be zero, and if the coefficients vector is empty,
    /// the polynomial is considered to be the zero polynomial.
    pub fn new(variable: Variable, coefficients: Vec<impl Into<Expression>>) -> Self {
        let coefficients: Vec<Expression> = coefficients.into_iter().map(|ex| ex.into()).collect();
        if coefficients.len() > 1 && coefficients.last().is_some_and(|ex| ex.is_zero()) {
            panic!("Leading coefficient cannot be zero!");
        }
        if coefficients.is_empty() {
            Self {
                variable,
                order: 0,
                coefficients: vec![0.into()],
            }
        } else {
            Self {
                variable,
                order: coefficients.len() - 1,
                coefficients,
            }
        }
    }

    /// Create a Polynomial from an expression and a variable. The expression is decomposed into terms to extract the coefficients for each power of the variable.
    pub fn from_expression(expression: Expression, variable: Variable) -> Self {
        if expression.is_zero() {
            return Self::new(variable, vec![0]);
        }

        let terms = expression.dissolve_into_terms();
        let highest_power = terms
            .iter()
            .map(|term| term.get_power_of_variable(&variable))
            .max()
            .unwrap();
        let mut coefficients: Vec<Expression> =
            vec![Expression::default(); highest_power as usize + 1];
        for mut term in terms {
            let variable_power = term.remove_variables(std::iter::once(&variable));
            let power = variable_power.get_power_of_variable(&variable);

            if let Some(handle) = coefficients.get_mut(power as usize) {
                *handle += term;
            }
        }
        Self::new(variable, coefficients)
    }

    /// Returns the order (degree) of the polynomial.
    pub fn order(&self) -> usize {
        self.order
    }

    /// Computes the derivative wrt to the variable of the polynomial.
    pub fn compute_nth_derivative(&self, n: usize) -> Self {
        let output = Expression::from(self);
        let output = output.calculate_nth_derivate_wrt_variable(n, &self.variable);

        Self::from_expression(output, self.variable)
    }

    /// Constructs an ElementarySymmetricPolynomials object for the roots of the polynomial, using a specified variable to represent the roots.
    /// For Example, if 'root_variable is 'x', then roots will be represented as x₁, x₂, ... xₙ when n is the order of the polynomial.
    pub fn construct_esc_for_roots(&self, root_variable: char) -> ElementarySymmetricPolynomials {
        let leading_coeff: Term = match self.coefficients.last().unwrap().clone().try_into() {
            Ok(term) => term,
            Err(_) => panic!("Leading coefficient is a expression!"),
        };
        let esc_variables = (1..=self.order())
            .map(|k| Variable::new(root_variable, Some(k as i64)))
            .collect::<Vec<Variable>>();
        let mut esc_calculator = ElementarySymmetricPolynomials::from_variables(&esc_variables);
        for combination_length in 1..=self.order() {
            let minus_one_to_power = if combination_length % 2 == 0 { 1 } else { -1 };
            let numerator = self.coefficients[self.order - combination_length].clone();
            let denominator = leading_coeff.clone();
            esc_calculator.set_ek_substitution_as(
                combination_length,
                minus_one_to_power * numerator / denominator,
            );
        }
        esc_calculator
    }

    fn to_string_internal(&self, is_sympy: bool) -> String {
        let mut data = String::new();
        let poly_variable = self.variable;
        let mut k = 0_usize;

        for coeff in &self.coefficients {
            if coeff.is_zero() && self.coefficients.len() == 1 {
                return "0".into();
            }
            if coeff.is_zero() && self.coefficients.len() > 1 {
                k += 1;
                continue;
            }

            if is_sympy {
                data.push_str(&format!("({})", coeff.to_string_sympy()));
            } else {
                data.push_str(&format!("({})", coeff));
            }

            if k > 0 {
                if is_sympy {
                    data.push('*');
                    data.push_str(
                        &poly_variable
                            .pow(k as TermVariablePowerType)
                            .to_string_sympy(),
                    );
                } else {
                    data.push_str(&format!(
                        "{}",
                        poly_variable.pow(k as TermVariablePowerType)
                    ));
                }
            }

            if k < self.coefficients.len() - 1 {
                data.push_str(" + ");
            }
            k += 1;
        }

        data
    }

    /// Returns the polynomial's SymPy-compatible string representation.
    pub fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }

    /// Evaluates polynomial for the input expression
    pub fn eval_at_expression<T: Into<Expression>>(&self, expression: T) -> Expression {
        let poly_expression = Expression::from(self);
        poly_expression.substitute_multiple_variables_with_expressions(std::iter::once((
            self.variable,
            expression,
        )))
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_internal(false))
    }
}

impl From<&Polynomial> for Expression {
    fn from(polynomial: &Polynomial) -> Self {
        polynomial
            .coefficients
            .iter()
            .enumerate()
            .map(|(power, coefficient)| {
                coefficient.clone() * polynomial.variable.pow(power as TermVariablePowerType)
            })
            .fold(Expression::default(), std::ops::Add::add)
    }
}

impl From<Polynomial> for Expression {
    fn from(polynomial: Polynomial) -> Self {
        Expression::from(&polynomial)
    }
}

impl PartialDerivative for Polynomial {
    fn calculate_derivate_wrt_variable(&self, variable: &Variable) -> Self {
        let expression = Expression::from(self);
        let derivative_expression = expression.calculate_derivate_wrt_variable(variable);

        Polynomial::from_expression(derivative_expression, self.variable)
    }

    fn calculate_nth_derivate_wrt_variable(&self, n: usize, variable: &Variable) -> Self
    where
        Self: Sized,
    {
        let expression = Expression::from(self);
        let derivative_expression = expression.calculate_nth_derivate_wrt_variable(n, variable);

        Polynomial::from_expression(derivative_expression, self.variable)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use crate::{operations::derivative::PartialDerivative, *};

    #[test]
    fn basic_test() {
        let variable = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let quadratic = Polynomial::new(variable, vec![c, b, a]);

        assert_eq!(quadratic.to_string(), "(c) + (b)x + (a)x²");

        let variable = Variable::new('x', None);
        let quadratic = Polynomial::new(variable, Vec::<crate::Expression>::new());

        assert_eq!(quadratic.to_string(), "0");

        let quadratic = Polynomial::new(
            variable,
            vec![
                Expression::default() + c,
                Expression::default(),
                Expression::default() + a,
            ],
        );

        assert_eq!(quadratic.to_string(), "(c) + (a)x²");

        let quadratic = a * (variable - b).pow(2);
        let quadratic = Polynomial::from_expression(quadratic, variable);

        assert_eq!(quadratic.to_string(), "(ab²) + (-2ab)x + (a)x²");

        let constant = a;
        let constant = Polynomial::from_expression(constant.into(), variable);

        assert_eq!(constant.to_string(), "(a)");
    }

    #[test]
    fn esc_creator_test() {
        let variable = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let quadratic = Polynomial::new(variable, vec![c, b, a]);

        let root = 'r';
        let r1 = Variable::new(root, Some(1));
        let r2 = Variable::new(root, Some(2));
        let mut esc = quadratic.construct_esc_for_roots(root);

        let simplified = esc.simplify_symmetric_expression(r1 + r2);
        assert_eq!(simplified.to_string(), "-a⁻¹b");

        let simplified = esc.simplify_symmetric_expression(r1 * r2 + 0);
        assert_eq!(simplified.to_string(), "a⁻¹c");

        let simplified = esc.simplify_symmetric_expression((r1 + r2).pow(3) + r1 * r2);
        assert_eq!(simplified.to_string(), "-a⁻³b³ + a⁻¹c");
    }

    #[test]
    fn derivative_test() {
        let variable = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let quadratic = Polynomial::new(variable, vec![c, b, a]);
        let linear = quadratic.compute_nth_derivative(1);
        assert_eq!(linear.to_string(), "(b) + (2a)x");

        let quadratic = a * (variable - b).pow(2);
        let quadratic = Polynomial::from_expression(quadratic, variable);
        let constant = quadratic.compute_nth_derivative(2);
        assert_eq!(constant.to_string(), "(2a)");
    }

    #[test]
    fn partial_derivative_wrt_variable_test() {
        let x = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let polynomial = Polynomial::new(x, vec![c, b, a]);

        assert_eq!(
            polynomial.calculate_derivate_wrt_variable(&x),
            Polynomial::new(x, vec![Expression::from(b), (2 * a).into()])
        );
        assert_eq!(
            polynomial.calculate_derivate_wrt_variable(&a),
            Polynomial::new(x, vec![0, 0, 1])
        );
        assert_eq!(
            polynomial.calculate_derivate_wrt_variable(&Variable::new('y', None)),
            Polynomial::new(x, vec![0])
        );
    }

    #[test]
    fn nth_partial_derivative_wrt_variable_test() {
        let x = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let polynomial = Polynomial::new(
            x,
            vec![
                Expression::from(1),
                Expression::from(b),
                Expression::from(a),
            ],
        );

        assert_eq!(
            polynomial.calculate_nth_derivate_wrt_variable(2, &x),
            Polynomial::new(x, vec![2 * a])
        );
        assert_eq!(
            polynomial.calculate_nth_derivate_wrt_variable(2, &a),
            Polynomial::new(x, vec![0])
        );
        assert_eq!(
            polynomial.calculate_nth_derivate_wrt_variable(0, &x),
            polynomial
        );
    }

    #[test]
    fn to_string_test() {
        let variable = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let quadratic = Polynomial::new(variable, vec![c + b, b + a, a + c]);

        assert_eq!(quadratic.to_string(), "(b + c) + (a + b)x + (a + c)x²");
        assert_eq!(
            quadratic.to_string_sympy(),
            "(b + c) + (a + b)*x + (a + c)*x**2"
        );
    }

    #[test]
    fn to_eval_at_expression() {
        let variable = Variable::new('x', None);
        let a = Variable::new('a', None);
        let b = Variable::new('b', None);
        let c = Variable::new('c', None);
        let quadratic = Polynomial::new(variable, vec![c + b, b + a, a + c]);

        assert_eq!(
            quadratic.eval_at_expression(a).to_string(),
            "ab + a² + a²c + a³ + b + c"
        );

        assert_eq!(
            quadratic.eval_at_expression(a + b + c).to_string(),
            "2ab + 4abc + ab² + ac + 3ac² + a² + 2a²b + 3a²c + a³ + b + bc + 2bc² + b² + b²c + c + c³"
        );

        assert_eq!(
            quadratic.eval_at_expression(a * b * c).to_string(),
            "ab²c + a²bc + a²b²c³ + a³b²c² + b + c"
        );
    }
}
