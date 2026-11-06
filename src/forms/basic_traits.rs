use std::fmt::Display;

use num::Rational64;

use crate::*;

/**
 * Common trait for symbolic values that can render a SymPy-compatible string.
 */
pub trait Form: Display + Clone + PartialEq {
    /// Returns the value's SymPy-compatible string representation.
    fn to_string_sympy(&self) -> String;
}

impl Form for Rational64 {
    fn to_string_sympy(&self) -> String {
        format!("({}/{})", self.numer(), self.denom())
    }
}

impl Form for Variable {
    fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

impl Form for Term {
    fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

impl Form for Expression {
    fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

impl Form for RationalExpression {
    fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

/**
 * Common trait for to check if form is a scalar.
 */
pub trait IsScalar {
    /// Returns true when the value is scalar.
    fn is_scalar(&self) -> bool;
}

impl IsScalar for Rational64 {
    fn is_scalar(&self) -> bool {
        true
    }
}

impl IsScalar for Term {
    fn is_scalar(&self) -> bool {
        self.variables.values().all(|p| *p == 0)
    }
}

impl IsScalar for Expression {
    fn is_scalar(&self) -> bool {
        self.terms().all(|t| t.is_scalar())
    }
}

impl IsScalar for RationalExpression {
    fn is_scalar(&self) -> bool {
        self.numerator.is_scalar() && self.denominator.is_scalar()
    }
}

/**
 * Common trait for scalar-like values that can be compared to zero or one.
 */
pub trait IsStandardScalar: IsScalar {
    /// Returns true when the value is the additive identity.
    fn is_zero(&self) -> bool;
    /// Returns true when the value is the multiplicative identity.
    fn is_unity(&self) -> bool;
}

impl<T: Eq + From<Rational64> + Form + IsScalar> IsStandardScalar for T {
    fn is_zero(&self) -> bool {
        *self == T::from(0.into())
    }

    fn is_unity(&self) -> bool {
        *self == T::from(1.into())
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.to_string_internal(false);
        write!(f, "{}", output)
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_internal(false))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.to_string_internal(false);
        write!(f, "{output}")
    }
}

impl Display for RationalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.to_string_internal(false);
        write!(f, "{output}")
    }
}

impl Default for Term {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            multiplier: TermMultiplierType::new(1, 1),
        }
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

impl PartialEq for RationalExpression {
    fn eq(&self, other: &Self) -> bool {
        let (s_num, s_den) = self.clone().dissolve_into_numerator_and_denominator();
        let (o_num, o_den) = other.clone().dissolve_into_numerator_and_denominator();
        s_num * o_den == s_den * o_num
    }
}

#[cfg(test)]
mod tests {
    use super::IsStandardScalar;
    use crate::{Expression, RationalExpression, Term, Variable};
    use num::Rational64;

    #[test]
    fn test_is_standard_scalar() {
        let verify = |value: &dyn IsStandardScalar, expected_zero: bool, expected_unity: bool| {
            assert_eq!(value.is_zero(), expected_zero);
            assert_eq!(value.is_unity(), expected_unity);
        };

        verify(&Rational64::from(0), true, false);
        verify(&Term::from(0), true, false);
        verify(&Expression::from(0), true, false);
        verify(&RationalExpression::from(0), true, false);

        verify(&Rational64::from(1), false, true);
        verify(&Term::from(1), false, true);
        verify(&Expression::from(1), false, true);
        verify(&RationalExpression::from(1), false, true);

        verify(&Rational64::new(2, 3), false, false);
        verify(&Term::from(Variable::new('x', None)), false, false);
        verify(
            &(Expression::from(Variable::new('x', None)) + 2_i64),
            false,
            false,
        );
        verify(
            &RationalExpression::new(
                1 + Variable::new('x', None),
                Variable::new('y', None) + Variable::new('y', None),
            ),
            false,
            false,
        );
    }

    #[test]
    fn test_default() {
        let default_term = Term::default();
        assert_eq!(default_term.multiplier(), 1.into());
        assert!(default_term.variables_map().is_empty());

        let default_expression = Expression::default();
        assert!(default_expression.terms.is_empty());

        let default_rational_expression = RationalExpression::default();
        assert_eq!(default_rational_expression.numerator, 0.into());
        assert_eq!(default_rational_expression.denominator, 1.into());
    }
}
