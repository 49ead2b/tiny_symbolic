use std::collections::BTreeMap;

use crate::*;

/// A symbolic expression consisting of a sum of terms.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Expression {
    pub(super) terms: BTreeMap<BTreeMap<Variable, TermVariablePowerType>, TermMultiplierType>,
}

impl Expression {
    pub(super) fn to_string_internal(&self, is_sympy: bool) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut terms_iter = self.terms();
        let first_term = terms_iter.next().unwrap();
        let mut output = if first_term.multiplier() < 0.into() {
            format!("-{}", (-first_term).to_string_internal(is_sympy))
        } else {
            first_term.to_string_internal(is_sympy)
        };

        for term in terms_iter {
            if term.multiplier() < 0.into() {
                output = format!("{} - {}", output, (-term).to_string_internal(is_sympy));
            } else {
                output = format!("{} + {}", output, term.to_string_internal(is_sympy));
            }
        }

        output
    }

    /// Extracts the terms of the expression into a vector of `Term` objects.
    pub fn dissolve_into_terms(self) -> Vec<Term> {
        self.terms
            .into_iter()
            .map(|(variables, multiplier)| Term::from_btreemap(variables, multiplier))
            .collect()
    }

    /// Raises this expression to a power
    /// Raising to a negative power is not supported in this version unless self is just a Term
    pub fn pow(self, power: TermVariablePowerType) -> Self {
        if power == 0 {
            if self.is_zero() {
                panic!("0 to the power of 0 is undefined!");
            }
            Term::default().into()
        } else if power < 0 {
            match TryInto::<Term>::try_into(self) {
                Ok(term) => term.pow(power).into(),
                Err(_) => {
                    panic!(
                        "Raising a expression to a negative power is not supported in this version!"
                    );
                }
            }
        } else {
            let mut base = self;
            let mut exponent = power;
            let mut result = Term::default().into();

            while exponent > 0 {
                if exponent % 2 == 1 {
                    result *= base.clone();
                }
                exponent /= 2;
                if exponent > 0 {
                    base = base.clone() * base;
                }
            }

            result
        }
    }

    /// Get iterator of the terms composing the expression
    pub fn terms(&self) -> impl Iterator<Item = Term> {
        self.terms
            .iter()
            .map(|(variables, multiplier)| Term::from_btreemap(variables.clone(), *multiplier))
    }

    /// Returns true if the expression contains the specified term.
    /// The term should identically match
    pub fn contains_term(&self, term: &Term) -> bool {
        self.terms
            .get(term.variables_map())
            .is_some_and(|multiplier| *multiplier == term.multiplier())
    }

    /// Returns true if the expression is divisible by the specified divisor term.
    /// Every term in the expression must be divible by the input term
    pub fn is_divisible_by(&self, divisor: impl Into<Term> + Clone) -> bool {
        for term in self.terms() {
            if !term.is_divisible_by(divisor.clone()) {
                return false;
            }
        }

        true
    }

    /// Returns true if the expression contains the specified variable.
    pub fn contains_variable(&self, variable: &Variable) -> bool {
        self.terms.keys().any(|term| term.contains_key(variable))
    }

    /// Returns true if the expression is a term (i.e., it has at most one term).
    pub fn is_term(&self) -> bool {
        self.terms.len() <= 1
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        forms::basic_traits::{Form, IsScalar},
        operations::derivative::PartialDerivative,
    };

    use super::*;
    use fmtastic::Superscript;

    #[test]
    fn test_basic() {
        let vp1 = Variable::new('x', Some(2)).pow(6);
        let vp2 = Variable::new('y', None).pow(4);

        let expr = vp1 + vp2;

        assert_eq!(expr.to_string(), "x₂⁶ + y⁴");
    }

    #[test]
    fn test_expression_operator_coverage() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let x_term = x.pow(2);

        let sum = (x + y) + x_term.clone();
        let difference = (x + y) - x;
        let product = (x + y) * x;
        let quotient = (x + y) / x;

        let scalar_sum = (x + y) + TermMultiplierType::new(3, 1);
        let scalar_difference = (x + y) - TermMultiplierType::new(3, 1);
        let scalar_product = (x + y) * TermMultiplierType::new(2, 1);
        let scalar_quotient = (x + y) / TermMultiplierType::new(2, 1);

        assert_eq!(sum.to_string(), "x + x² + y");
        assert_eq!(difference.to_string(), "y");
        assert_eq!(product.to_string(), "xy + x²");
        assert_eq!(quotient.to_string(), "1 + x⁻¹y");

        assert_eq!(scalar_sum.to_string(), "3 + x + y");
        assert_eq!(scalar_difference.to_string(), "-3 + x + y");
        assert_eq!(scalar_product.to_string(), "2x + 2y");
        assert_eq!(scalar_quotient.to_string(), "(1/2)x + (1/2)y");
    }

    #[test]
    fn test_expression_assignment_and_helper_traits() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let x_term = x.pow(2);

        let summed = [x + y, x_term.clone().into()]
            .into_iter()
            .sum::<Expression>();
        let negated = -(x + y);

        let mut add_assign_term = x + y;
        add_assign_term += x_term.clone();

        let mut add_assign_expression = x + y;
        add_assign_expression += x + y;

        let mut add_assign_scalar = x + y;
        add_assign_scalar += TermMultiplierType::new(3, 1);

        let mut mul_assign_variable = x + y;
        mul_assign_variable *= x;

        let mut mul_assign_scalar = x + y;
        mul_assign_scalar *= TermMultiplierType::new(2, 1);

        let mut mul_assign_expression = x + y;
        mul_assign_expression *= x + y;

        let mut sub_assign_variable = x + y;
        sub_assign_variable -= x;

        let mut sub_assign_scalar = x + y;
        sub_assign_scalar -= TermMultiplierType::new(3, 1);

        let mut sub_assign_expression = x + y;
        sub_assign_expression -= x + y;

        let mut div_assign_variable = x + y;
        div_assign_variable /= x;

        let mut div_assign_term = x + y;
        div_assign_term /= x_term.clone();

        let mut div_assign_scalar = x + y;
        div_assign_scalar /= TermMultiplierType::new(2, 1);

        let scalar_sub_expression = TermMultiplierType::new(3, 1) - (x + y);

        assert_eq!(summed.to_string(), "x + x² + y");
        assert_eq!(negated.to_string(), "-x - y");
        assert_eq!(add_assign_term.to_string(), "x + x² + y");
        assert_eq!(add_assign_expression.to_string(), "2x + 2y");
        assert_eq!(add_assign_scalar.to_string(), "3 + x + y");
        assert_eq!(mul_assign_variable.to_string(), "xy + x²");
        assert_eq!(mul_assign_scalar.to_string(), "2x + 2y");
        assert_eq!(mul_assign_expression.to_string(), "2xy + x² + y²");
        assert_eq!(sub_assign_variable.to_string(), "y");
        assert_eq!(sub_assign_scalar.to_string(), "-3 + x + y");
        assert_eq!(sub_assign_expression.to_string(), "0");
        assert_eq!(div_assign_variable.to_string(), "1 + x⁻¹y");
        assert_eq!(div_assign_term.to_string(), "x⁻²y + x⁻¹");
        assert_eq!(div_assign_scalar.to_string(), "(1/2)x + (1/2)y");
        assert_eq!(scalar_sub_expression.to_string(), "3 - x - y");
    }

    #[test]
    fn test_moderate() {
        let vp1 = Variable::new('y', Some(2)).pow(3);
        let vp2 = Variable::new('x', Some(1)).pow(4);

        let term = (vp1 * vp2.clone()) * TermMultiplierType::new(3, 2);
        let expr = term + vp2.clone();

        assert!(expr.contains_term(&vp2));
        assert!(expr.contains_variable(&Variable::new('x', Some(1))));
        assert!(expr.contains_variable(&Variable::new('y', Some(2))));
    }

    #[test]
    #[should_panic(expected = "0 to the power of 0 is undefined!")]
    fn test_expression_pow_zero_edge_case_panics_for_empty_expression() {
        let expression = Expression::from(2).pow(0);

        assert_eq!(expression.to_string(), "1");

        let _ = Expression::default().pow(0);
    }

    #[test]
    #[should_panic(
        expected = "Raising a expression to a negative power is not supported in this version!"
    )]
    fn test_expression_pow_negative() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let expression = x.pow(2) + 0;

        let result = expression.pow(-3);

        assert_eq!(result.to_string(), "x⁻⁶");

        let _ = (x.pow(2) + y).pow(-3);
    }

    #[test]
    fn test_expression_try_into_term_empty_expression_returns_zero() {
        let expression = Expression::default();

        let term: Term = TryInto::<Term>::try_into(expression).unwrap();

        assert_eq!(term.to_string(), "0");
    }

    #[test]
    fn test_expression_mul_assign_expression_branch() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let mut expression = x + y;

        expression *= x + y;

        assert_eq!(expression.to_string(), "2xy + x² + y²");

        let z = 2 * Variable::new('z', None);
        expression *= z;

        assert_eq!(expression.to_string(), "4xyz + 2x²z + 2y²z");

        expression += Variable::new('z', None);

        assert_eq!(expression.to_string(), "4xyz + 2x²z + 2y²z + z");
    }

    #[test]
    fn test_expression_substitute_variable_power_with_expression() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let expression = x.pow(4) + y;

        let substituted = substitution_operations::VariablePowerSubstitutions::<
            Expression,
        >::substitute_variable_power(expression, x, 2, y.pow(2).into());

        assert_eq!(substituted.to_string(), "y + y⁴");
    }

    #[test]
    #[should_panic(expected = "Duplicate substitutions found for x!")]
    fn test_expression_substitute_multiple_variables() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);

        let expression = x.pow(4) + y;

        let substituted = <Expression as substitution_operations::VariableSubstitutions<
            Expression,
        >>::substitute_variables(
            expression.clone(), [(x, 2 + y.pow(2)), (y, 2 + z.pow(2))]
        );

        assert_eq!(substituted.to_string(), "18 + 32y² + 24y⁴ + 8y⁶ + y⁸ + z²");

        let _ = <Expression as substitution_operations::VariableSubstitutions<
            Expression,
        >>::substitute_variables(
            expression.clone(), [(x, 2 + y.pow(2)), (x, z.into())]
        );
    }

    #[test]
    fn test_expression_helper_behaviors() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let scalar_expression = Expression::from(TermMultiplierType::new(3, 1));
        let divisible_expression = x.pow(2) * y + x;
        let multi_term_expression = x + y;

        assert!(Expression::default().is_scalar());
        assert!(scalar_expression.is_scalar());
        assert!(scalar_expression.is_term());
        assert_eq!(
            TryInto::<Term>::try_into(scalar_expression)
                .unwrap()
                .to_string(),
            "3"
        );
        assert!(divisible_expression.is_divisible_by(x));
        assert!(!divisible_expression.is_divisible_by(x.pow(3)));
        assert!(!multi_term_expression.is_term());
        assert!(TryInto::<Term>::try_into(multi_term_expression).is_err());
    }

    #[test]
    fn test_complex() {
        let vp1 = Variable::new('x', None);
        let vp2 = Variable::new('y', None);

        let expr = vp1 - vp1 / vp2;

        assert!(expr.contains_variable(&vp1));
        assert!(expr.contains_variable(&vp2));
        assert!(!expr.is_zero());
    }

    #[test]
    fn test_to_print_sympy() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let expression = x.pow(2) * y.pow(3) * 4 + z;

        assert!(expression.to_string_sympy() == "4*(x**2)*(y**3) + z");
    }

    #[test]
    fn test_partial_derivative() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let expression = x.pow(2) * y.pow(3) * 4 + z;

        let expected = format!("8xy{}", Superscript(3));
        assert_eq!(
            expression.calculate_derivate_wrt_variable(&x).to_string(),
            expected
        );

        let expected = format!("24x{}", Superscript(2));
        assert_eq!(
            expression
                .calculate_nth_derivate_wrt_variable(3, &y)
                .to_string(),
            expected
        );

        let expected = format!("4x{}y{} + z", Superscript(2), Superscript(3));
        assert_eq!(
            expression
                .calculate_nth_derivate_wrt_variable(0, &z)
                .to_string(),
            expected
        );
    }
}
