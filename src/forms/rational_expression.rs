use crate::*;
use num::Rational64;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Rational expression consisting of a numerator and a denominator, both of which are expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RationalExpression {
    numerator: Expression,
    denominator: Expression,
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
    pub fn pow(self, power: i32) -> Self {
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
        known_power: i32,
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

impl Neg for RationalExpression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.numerator, self.denominator)
    }
}

impl Add for RationalExpression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator.clone() + rhs.numerator * self.denominator.clone(),
            self.denominator * rhs.denominator,
        )
    }
}

impl<T> Add<T> for RationalExpression
where
    T: Into<Expression>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = RationalExpression::from(rhs);
        self + rhs
    }
}

impl_commutative_op!(Add::add, +, Expression, RationalExpression, RationalExpression);
impl_commutative_op!(Add::add, +, Term, RationalExpression, RationalExpression);
impl_commutative_op!(Add::add, +, Variable, RationalExpression, RationalExpression);
impl_commutative_op!(Add::add, +, Rational64, RationalExpression, RationalExpression);

impl AddAssign for RationalExpression {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for RationalExpression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator.clone() - rhs.numerator * self.denominator.clone(),
            self.denominator * rhs.denominator,
        )
    }
}

impl<T> Sub<T> for RationalExpression
where
    T: Into<Expression>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = RationalExpression::from(rhs);
        self - rhs
    }
}

impl SubAssign for RationalExpression {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl Mul for RationalExpression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl<T> Mul<T> for RationalExpression
where
    T: Into<Expression>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = RationalExpression::from(rhs);
        self * rhs
    }
}

impl_commutative_op!(Mul::mul, *, Expression, RationalExpression, RationalExpression);
impl_commutative_op!(Mul::mul, *, Term, RationalExpression, RationalExpression);
impl_commutative_op!(Mul::mul, *, Variable, RationalExpression, RationalExpression);
impl_commutative_op!(Mul::mul, *, Rational64, RationalExpression, RationalExpression);

impl MulAssign for RationalExpression {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div for RationalExpression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl<T> Div<T> for RationalExpression
where
    T: Into<Expression>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = RationalExpression::from(rhs);
        self / rhs
    }
}

impl DivAssign for RationalExpression {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rational(numerator: i64, denominator: i64) -> RationalExpression {
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
}
