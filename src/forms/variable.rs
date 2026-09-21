use std::fmt::Display;

use fmtastic::Subscript;

use crate::*;

/// A symbolic variable identified by a name and optional subscript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Variable {
    name: char,
    subscript: Option<i64>,
}

impl Variable {
    /// Creates a new symbolic variable.
    pub fn new(name: char, subscript: Option<i64>) -> Self {
        Self { name, subscript }
    }

    /// Raises the variable to the given integer power.
    pub fn pow(self, power: TermVariablePowerType) -> Term {
        Term::from(self).pow(power)
    }

    pub(super) fn to_string_internal(self, is_sympy: bool) -> String {
        if let Some(sub) = self.subscript {
            if is_sympy {
                format!("{}_{}", self.name, sub)
            } else {
                format!("{}{}", self.name, Subscript(sub))
            }
        } else {
            format!("{}", self.name)
        }
    }
    /// Returns the variable's SymPy-compatible string representation.
    pub fn to_string_sympy(&self) -> String {
        self.to_string_internal(true)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.to_string_internal(false);
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::TermMultiplierType;

    use super::*;

    #[test]
    fn test_variable_display_and_power() {
        let x = Variable::new('x', None);
        let xp2 = x.pow(2);

        assert_eq!(x.to_string(), "x");
        assert_eq!(xp2.to_string(), "x²");
        assert!(xp2.contains_variable(&x));
    }

    #[test]
    fn test_variable_operator_coverage() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);

        let negated = -x;
        let sum = x + y;
        let difference = x - y;
        let product = x * y;
        let quotient = x / y;
        let quotient_by_term = x / Term::from(y);

        let scalar_sum = x + TermMultiplierType::new(3, 1);
        let scalar_difference = x - TermMultiplierType::new(3, 1);
        let scalar_product = x * TermMultiplierType::new(2, 1);
        let scalar_quotient = x / TermMultiplierType::new(2, 1);
        let scalar_subtracted_by_variable = TermMultiplierType::new(3, 1) - x;
        let scalar_divided_by_variable = TermMultiplierType::new(3, 1) / x;

        assert_eq!(negated.to_string(), "-1x");
        assert_eq!(sum.to_string(), "x + y");
        assert_eq!(difference.to_string(), "x - y");
        assert_eq!(product.to_string(), "xy");
        assert_eq!(quotient.to_string(), "xy⁻¹");
        assert_eq!(quotient_by_term.to_string(), "xy⁻¹");

        assert_eq!(scalar_sum.to_string(), "3 + x");
        assert_eq!(scalar_difference.to_string(), "-3 + x");
        assert_eq!(scalar_product.multiplier(), TermMultiplierType::new(2, 1));
        assert_eq!(scalar_product.to_string(), "2x");
        assert_eq!(scalar_quotient.multiplier(), TermMultiplierType::new(1, 2));
        assert_eq!(scalar_quotient.to_string(), "(1/2)x");
        assert_eq!(scalar_subtracted_by_variable.to_string(), "3 - x");
        assert_eq!(scalar_divided_by_variable.to_string(), "3x⁻¹");
    }

    #[test]
    fn test_variable_arithmetic_conversions() {
        let x = Variable::new('x', None);
        let expr = x + 3;
        let term = x * 2;

        assert!(expr.contains_variable(&x));
        assert!(term.contains_variable(&x));
        assert_eq!(term.multiplier(), TermMultiplierType::new(2, 1));
    }

    #[test]
    fn test_to_print_sympy() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', Some(3));

        assert!(x.to_string_sympy() == "x");
        assert!(y.to_string_sympy() == "y_3");
    }
}
