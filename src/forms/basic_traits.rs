use std::fmt::Display;

use num::Rational64;

use crate::*;

/**
 * Common trait for symbolic values that can render a SymPy-compatible string.
 */
pub trait Form: Display {
    /// Returns the value's SymPy-compatible string representation.
    fn to_string_sympy(&self) -> String;
}

impl Form for Variable {
    fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

impl Form for Term {
    fn to_string_sympy(&self) -> String {
        let variables = self.variables();
        let multiplier = self.multiplier();
        Term::to_string_internal(variables, multiplier, true)
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
 * Common trait for scalar-like values that can be compared to zero or one.
 */
pub trait IsStandardScalar {
    /// Returns true when the value is the additive identity.
    fn is_zero(&self) -> bool;
    /// Returns true when the value is the multiplicative identity.
    fn is_unity(&self) -> bool;
}

impl<T: Eq + From<Rational64> + Form> IsStandardScalar for T {
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
        let variables = self.variables();
        let multiplier = self.multiplier();

        write!(
            f,
            "{}",
            Term::to_string_internal(variables, multiplier, false)
        )
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
