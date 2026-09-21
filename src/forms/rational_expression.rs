use crate::{operations::derivative::PartialDerivative, *};
use std::{collections::HashMap, fmt::Display, ops::Mul};

/// Rational expression consisting of a numerator and a denominator, both of which are expression
/// Since multiplying numerator and denominator with the same thing does not change the value
/// So values of numerator and denominator maybe individually different based on the construction
/// Even though taken together, they are mathematically identical
#[derive(Debug, Clone, Eq)]
pub struct RationalExpression {
    pub(super) numerator: Expression,
    pub(super) denominator: Expression,
}

impl PartialEq for RationalExpression {
    fn eq(&self, other: &Self) -> bool {
        let (s_num, s_den) = self.clone().dissolve_into_numerator_and_denominator();
        let (o_num, o_den) = other.clone().dissolve_into_numerator_and_denominator();
        s_num * o_den == s_den * o_num
    }
}

impl PartialDerivative for RationalExpression {
    fn calculate_derivate_wrt_variable(&self, variable: &Variable) -> Self {
        let (numerator, denominator) = self.clone().dissolve_into_numerator_and_denominator();
        let numerator_derivative = numerator.calculate_derivate_wrt_variable(variable);
        let denominator_derivative = denominator.calculate_derivate_wrt_variable(variable);

        let new_numerator =
            numerator_derivative * denominator.clone() - numerator.clone() * denominator_derivative;
        let new_denominator = denominator.pow(2);

        Self::new(new_numerator, new_denominator)
    }
}

impl RationalExpression {
    /// Creates a new rational expression with the given numerator and denominator.
    pub fn new(numerator: Expression, denominator: Expression) -> Self {
        if denominator.is_zero() {
            panic!("Division by 0!");
        }
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns the inverse of the rational expression (i.e., swaps the numerator and denominator).
    pub fn inverse(self) -> Self {
        Self::new(self.denominator, self.numerator)
    }

    /// Extracts the numerator and denominator of the rational expression.
    pub fn dissolve_into_numerator_and_denominator(self) -> (Expression, Expression) {
        (self.numerator, self.denominator)
    }

    /// Is rational expression just 0
    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    /// Raises this rational expression to a power
    pub fn pow(self, power: TermVariablePowerType) -> Self {
        if power == 0 {
            if self.is_zero() {
                panic!("0 to the power of 0 is undefined!");
            }
            Self::default()
        } else if power < 0 {
            Self::new(self.denominator.pow(-power), self.numerator.pow(-power))
        } else {
            Self::new(self.numerator.pow(power), self.denominator.pow(power))
        }
    }

    /// Substitutes multiple variables with provided corresponding expressions.
    pub fn substitute_multiple_variables_with_expressions<T, U>(self, substitutions: T) -> Self
    where
        T: IntoIterator<Item = (Variable, U)>,
        U: Into<Expression>,
    {
        let substitutions: Vec<(Variable, Expression)> = substitutions
            .into_iter()
            .map(|(var, expr)| (var, expr.into()))
            .collect();

        Self::new(
            self.numerator
                .substitute_multiple_variables_with_expressions(substitutions.clone()),
            self.denominator
                .substitute_multiple_variables_with_expressions(substitutions.clone()),
        )
    }

    /// Substitutes a variable raised to a power with a provided expression.
    pub fn substitute_variable_power_with_expression(
        self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: Expression,
    ) -> RationalExpression {
        Self::new(
            self.numerator.substitute_variable_power_with_expression(
                variable,
                known_power,
                subst.clone(),
            ),
            self.denominator.substitute_variable_power_with_expression(
                variable,
                known_power,
                subst.clone(),
            ),
        )
    }

    /// Returns true if the rational expression contains the specified variable.
    pub fn contains_variable(&self, variable: &Variable) -> bool {
        self.numerator.contains_variable(variable) || self.denominator.contains_variable(variable)
    }

    /// Returns true if the rational expression is a scalar (i.e., it has no variables).
    pub fn is_scalar(&self) -> bool {
        self.numerator.is_scalar() && self.denominator.is_scalar()
    }

    fn get_term_to_set_power_sign(expression: &Expression, is_positive: bool) -> Term {
        let mut variables_powers = HashMap::new();
        for term in expression.terms() {
            for (variable, power) in term.variables() {
                if is_positive && *power < 0 {
                    variables_powers
                        .entry(*variable)
                        .and_modify(|min_power: &mut TermVariablePowerType| {
                            *min_power = (*min_power).min(*power)
                        })
                        .or_insert(*power);
                } else if !is_positive && *power > 0 {
                    variables_powers
                        .entry(*variable)
                        .and_modify(|max_power: &mut TermVariablePowerType| {
                            *max_power = (*max_power).min(*power)
                        })
                        .or_insert(*power);
                }
            }
        }

        variables_powers
            .into_iter()
            .map(|(v, p)| v.pow(p))
            .fold(Term::default(), Mul::mul)
            .pow(-1)
    }

    /// Get Rational Expression with all variables having positive number
    /// Done my multiplying numerator and denominator with same term
    /// Usually used before to_string() to display in a more readable format
    pub fn change_to_positive_power(self) -> Self {
        let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
        let numerator_norm = Self::get_term_to_set_power_sign(&numerator, true);
        let denominator_norm = Self::get_term_to_set_power_sign(&denominator, true);

        let terms_lcm = Term::lcm(vec![numerator_norm, denominator_norm]);
        Self {
            numerator: terms_lcm.clone() * numerator,
            denominator: terms_lcm * denominator,
        }
    }

    /// Get Rational Expression with all variables having negative number
    /// Done my multiplying numerator and denominator with same term
    /// Usually used before to_string() to display in a more readable format
    pub fn change_to_negative_power(self) -> Self {
        let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
        let numerator_norm = Self::get_term_to_set_power_sign(&numerator, false);
        let denominator_norm = Self::get_term_to_set_power_sign(&denominator, false);

        let terms_lcm = Term::lcm(vec![numerator_norm, denominator_norm]);
        Self {
            numerator: terms_lcm.clone() * numerator,
            denominator: terms_lcm * denominator,
        }
    }

    /// Returns the rational expression's SymPy-compatible string representation.
    pub fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }

    pub(super) fn to_string_internal(&self, is_sympy: bool) -> String {
        format!(
            "({})/({})",
            self.numerator.to_string_internal(is_sympy),
            self.denominator.to_string_internal(is_sympy)
        )
    }

    /// Multiplies both the numerator and denominator of the rational expression by the given value.
    pub fn with_numerator_and_denominator_multiplied_by_expression<T: Into<Expression>>(
        mut self,
        value: T,
    ) -> Self {
        let value = value.into();
        self.numerator *= value.clone();
        self.denominator *= value;
        self
    }
}

impl Default for RationalExpression {
    fn default() -> Self {
        Self {
            numerator: Expression::default(),
            denominator: 1.into(),
        }
    }
}

impl TryInto<Expression> for RationalExpression {
    type Error = String;

    fn try_into(self) -> Result<Expression, Self::Error> {
        let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
        if denominator.is_term() && denominator != 0.into() {
            let denominator: Term = denominator.try_into().unwrap();
            Ok(numerator / denominator)
        } else {
            Err("Cannot be reduced to a expression!".to_string())
        }
    }
}

impl<T> From<T> for RationalExpression
where
    T: Into<Expression>,
{
    fn from(value: T) -> Self {
        Self {
            numerator: value.into(),
            denominator: 1.into(),
        }
    }
}

impl Display for RationalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.to_string_internal(false);
        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use num::traits::Inv;

    use super::*;

    fn rational(
        numerator: TermVariablePowerType,
        denominator: TermVariablePowerType,
    ) -> RationalExpression {
        RationalExpression::new(numerator.into(), denominator.into())
    }

    #[test]
    fn test_rational_expression_operators() {
        let first = rational(1, 2);
        let second = rational(1, 3);

        assert_eq!(first.clone() + second.clone(), rational(5, 6));
        assert_eq!(first.clone() - second.clone(), rational(1, 6));
        assert_eq!(first.clone() * second.clone(), rational(1, 6));
        assert_eq!(first / second, rational(3, 2));
    }

    #[test]
    fn test_rational_expression_operators_with_expression_operands() {
        let x = Variable::new('x', None);
        let expression: Expression = x + 1;
        let rational_expression = rational(2, 3);
        let two: Expression = 2.into();
        let three: Expression = 3.into();

        assert_eq!(
            rational_expression.clone() - expression.clone(),
            RationalExpression::new(
                two.clone() - expression.clone() * three.clone(),
                three.clone()
            )
        );
        assert_eq!(
            rational_expression.clone() * expression.clone(),
            RationalExpression::new(two.clone() * expression.clone(), three.clone())
        );
        assert_eq!(
            rational_expression / expression.clone(),
            RationalExpression::new(two, three * expression)
        );
    }

    #[test]
    fn test_rational_expression_operators_with_symbolic_expressions() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let x_expression: Expression = x.into();
        let y_expression: Expression = y.into();
        let sum = x + y;
        let x_squared: Expression = x.pow(2).into();

        let first = RationalExpression::new(sum.clone(), x_expression.clone());
        let second = RationalExpression::new(x_squared.clone(), y_expression.clone());

        assert_eq!(
            first.clone() + second.clone(),
            RationalExpression::new(
                sum.clone() * y_expression.clone() + x_squared.clone() * x_expression.clone(),
                x_expression.clone() * y_expression.clone(),
            )
        );
        assert_eq!(
            first.clone() - second.clone(),
            RationalExpression::new(
                sum.clone() * y_expression.clone() - x_squared.clone() * x_expression.clone(),
                x_expression.clone() * y_expression.clone(),
            )
        );
        assert_eq!(
            first.clone() * second.clone(),
            RationalExpression::new(
                sum.clone() * x_squared.clone(),
                x_expression.clone() * y_expression.clone(),
            )
        );

        assert_eq!(
            first / second,
            RationalExpression::new(sum * y_expression, x_expression * x_squared,)
        );
    }

    #[test]
    fn test_rational_expression_operators_with_full_expression_denominators() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let x_expression: Expression = x.into();
        let y_expression: Expression = y.into();
        let one: Expression = 1.into();
        let two: Expression = 2.into();
        let first_numerator = x_expression.clone() + one.clone();
        let first_denominator = y_expression.clone() + two.clone();
        let second_numerator = x_expression.clone() * y_expression.clone() + one.clone();
        let second_denominator = x_expression.clone() + y_expression.clone();
        let first = RationalExpression::new(first_numerator.clone(), first_denominator.clone());
        let second = RationalExpression::new(second_numerator.clone(), second_denominator.clone());

        assert_eq!(
            first.clone() + second.clone(),
            RationalExpression::new(
                first_numerator.clone() * second_denominator.clone()
                    + second_numerator.clone() * first_denominator.clone(),
                first_denominator.clone() * second_denominator.clone(),
            )
        );
        assert_eq!(
            first.clone() - second.clone(),
            RationalExpression::new(
                first_numerator.clone() * second_denominator.clone()
                    - second_numerator.clone() * first_denominator.clone(),
                first_denominator.clone() * second_denominator.clone(),
            )
        );
        assert_eq!(
            first.clone() * second.clone(),
            RationalExpression::new(
                first_numerator.clone() * second_numerator.clone(),
                first_denominator.clone() * second_denominator.clone(),
            )
        );
        assert_eq!(
            first / second,
            RationalExpression::new(
                first_numerator * second_denominator,
                first_denominator * second_numerator,
            )
        );
    }

    #[test]
    fn test_inverse_and_dissolve_with_full_expressions() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let numerator: Expression = x + y;
        let denominator: Expression = x.pow(2) + 1;
        let expression = RationalExpression::new(numerator.clone(), denominator.clone());

        assert_eq!(
            expression.clone().inverse(),
            RationalExpression::new(denominator.clone(), numerator.clone())
        );
        assert_eq!(
            expression.dissolve_into_numerator_and_denominator(),
            (numerator, denominator)
        );
    }

    #[test]
    fn test_is_zero() {
        assert!(RationalExpression::new(0.into(), Variable::new('x', None) + 1).is_zero());
        assert!(!RationalExpression::new(Variable::new('x', None) + 1, 2.into(),).is_zero());
    }

    #[test]
    fn test_contains_variable_and_is_scalar() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', Some(3));
        let z = Variable::new('z', None);
        let expression = RationalExpression::new(x + y, x.pow(2) + 1);

        assert!(expression.contains_variable(&x));
        assert!(expression.contains_variable(&y));
        assert!(!expression.contains_variable(&z));
        assert!(!expression.is_scalar());
        assert!(rational(2, 3).is_scalar());
    }

    #[test]
    fn test_rational_expression_string_representations() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', Some(3));
        let expression = RationalExpression::new(x + y, x.pow(2) + 1);

        assert_eq!(expression.to_string_internal(false), "(x + y₃)/(1 + x²)");
        assert_eq!(expression.to_string_sympy(), "(x + y_3)/(1 + x**2)");
        assert_eq!(
            expression.to_string_internal(true),
            expression.to_string_sympy()
        );
        assert_eq!(
            expression.clone().pow(2).to_string(),
            "(2xy₃ + x² + y₃²)/(1 + 2x² + x⁴)"
        );
        assert_eq!(
            expression.clone().pow(-2).to_string_sympy(),
            "(1 + 2*(x**2) + x**4)/(2*x*y_3 + x**2 + y_3**2)"
        );
    }

    #[test]
    fn test_rational_expression_pow() {
        let x = Variable::new('x', None);
        let numerator: Expression = x + 1;
        let denominator: Expression = x.pow(2) + 2;
        let expression = RationalExpression::new(numerator.clone(), denominator.clone());

        assert_eq!(
            expression.clone().pow(2),
            RationalExpression::new(numerator.clone().pow(2), denominator.clone().pow(2))
        );
        assert_eq!(expression.clone().pow(0), RationalExpression::default());
        assert_eq!(
            expression.pow(-2),
            RationalExpression::new(denominator.pow(2), numerator.pow(2))
        );
    }

    #[test]
    #[should_panic(expected = "0 to the power of 0 is undefined!")]
    fn test_zero_rational_expression_pow_zero() {
        let _ = RationalExpression::default().pow(0);
    }

    #[test]
    #[should_panic(expected = "Division by 0!")]
    fn test_division_by_zero_rational_expression() {
        let _ = rational(1, 2) / rational(0, 1);
    }

    #[test]
    fn test_rational_expression_from_term() {
        let w = Variable::new('w', None);
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let term = w.pow(-2) * y.pow(3) * x.pow(-7) * z.pow(4) * TermMultiplierType::new(7, 5);

        let rational_expression = RationalExpression::from(term);

        assert_eq!(
            rational_expression.change_to_positive_power().to_string(),
            "((7/5)y³z⁴)/(w²x⁷)"
        );
    }

    #[test]
    fn test_rational_expression_from_expression() {
        let w = Variable::new('w', None);
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let expression = w.pow(-2) * z.pow(-7) * TermMultiplierType::new(2, 7)
            + y.pow(3) * x.pow(-7) * TermMultiplierType::new(3, 14)
            + x.pow(5) * z.pow(-4) * TermMultiplierType::new(5, 3);

        let rational_expression = RationalExpression::new(expression.clone(), expression.pow(2));

        assert_eq!(
            rational_expression
                .clone()
                .change_to_positive_power()
                .to_string(),
            "((2/7)w²x¹⁴z⁷ + (3/14)w⁴x⁷y³z¹⁴ + (5/3)w⁴x¹⁹z¹⁰)/((6/49)w²x⁷y³z⁷ + (20/21)w²x¹⁹z³ + (5/7)w⁴x¹²y³z¹⁰ + (25/9)w⁴x²⁴z⁶ + (9/196)w⁴y⁶z¹⁴ + (4/49)x¹⁴)"
        );

        assert_eq!(
            rational_expression
                .clone()
                .change_to_negative_power()
                .to_string(),
            "((2/7)w⁻²x⁻⁵y⁻³z⁻⁷ + (3/14)x⁻¹² + (5/3)y⁻³z⁻⁴)/((4/49)w⁻⁴x⁻⁵y⁻³z⁻¹⁴ + (6/49)w⁻²x⁻¹²z⁻⁷ + (20/21)w⁻²y⁻³z⁻¹¹ + (9/196)x⁻¹⁹y³ + (5/7)x⁻⁷z⁻⁴ + (25/9)x⁵y⁻³z⁻⁸)"
        );
    }

    #[test]
    fn test_rational_expression_partial_derivative() {
        let x = Variable::new('x', None);
        let numerator: Expression = x.pow(2) + 1;
        let denominator: Expression = x + 1;
        let expression = RationalExpression::new(numerator.clone(), denominator.clone());

        let derivative = expression.calculate_derivate_wrt_variable(&x);
        let expected = RationalExpression::new(
            numerator.clone().calculate_derivate_wrt_variable(&x) * denominator.clone()
                - numerator.clone() * denominator.calculate_derivate_wrt_variable(&x),
            denominator.clone().pow(2),
        );

        assert_eq!(derivative, expected);
        assert_eq!(derivative.to_string(), "(-1 + 2x + x²)/(1 + 2x + x²)");
    }

    #[test]
    fn test_rational_expression_equality() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);

        let rational_expression_v1 =
            RationalExpression::new((x + y) * TermMultiplierType::new(4, 3), (x + y).pow(2));

        let rational_expression_v2 =
            RationalExpression::new(1.into(), TermMultiplierType::new(4, 3).inv() * (x + y));

        assert_eq!(rational_expression_v1, rational_expression_v2);
    }
}
