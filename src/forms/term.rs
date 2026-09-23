use fmtastic::Superscript;
use num::Zero;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::iter::Product;
use std::{fmt::Display, ops::Mul};

use crate::operations::derivative::PartialDerivative;
use crate::*;

/// A symbolic term consisting of a rational multiplier and a product of variables raised to integer exponents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Term {
    pub(super) variables: BTreeMap<Variable, TermVariablePowerType>,
    pub(super) multiplier: TermMultiplierType,
}

impl Term {
    pub(crate) fn variables(&self) -> &BTreeMap<Variable, TermVariablePowerType> {
        &self.variables
    }

    /// Returns the rational multiplier of the term.
    pub fn multiplier(&self) -> TermMultiplierType {
        self.multiplier
    }

    pub(super) fn to_string_internal(
        variables: &BTreeMap<Variable, TermVariablePowerType>,
        multiplier: TermMultiplierType,
        is_sympy: bool,
    ) -> String {
        if *multiplier.numer() == 0 {
            return "0".into();
        }
        let mut data = if *multiplier.denom() == 1 && *multiplier.numer() == 1 {
            if variables.is_empty() {
                "1".to_string()
            } else {
                String::new()
            }
        } else if *multiplier.denom() == 1 {
            format!("{}", multiplier)
        } else {
            format!("({})", multiplier)
        };
        for (variable, power) in variables {
            data = match *power {
                1 => {
                    let variable = variable.to_string_internal(is_sympy);
                    if is_sympy {
                        if data.is_empty() {
                            variable
                        } else {
                            format!("{}*{}", data, variable)
                        }
                    } else {
                        format!("{}{}", data, variable)
                    }
                }
                other => {
                    let variable = variable.to_string_internal(is_sympy);
                    if is_sympy {
                        if data.is_empty() {
                            if variables.len() == 1 {
                                format!("{}**{}", variable, other)
                            } else {
                                format!("({}**{})", variable, other)
                            }
                        } else {
                            format!("{}*({}**{})", data, variable, other)
                        }
                    } else {
                        format!("{}{}{}", data, variable, Superscript(other))
                    }
                }
            };
        }

        data
    }

    /// Returns the term's SymPy-compatible string representation.
    pub fn to_string_sympy(&self) -> String {
        let variables = self.variables();
        let multiplier = self.multiplier();
        Term::to_string_internal(variables, multiplier, true)
    }

    pub(super) fn dissolve(
        self,
    ) -> (
        BTreeMap<Variable, TermVariablePowerType>,
        TermMultiplierType,
    ) {
        (self.variables, self.multiplier)
    }

    pub(super) fn from_btreemap(
        variables: BTreeMap<Variable, TermVariablePowerType>,
        multiplier: TermMultiplierType,
    ) -> Self {
        let variables = variables
            .into_iter()
            .filter(|(_, power)| *power != 0)
            .collect();
        Self {
            variables,
            multiplier,
        }
    }

    /// Raises the term to the specified integer power.
    pub fn pow(mut self, power: TermVariablePowerType) -> Self {
        if power == 0 {
            if self.multiplier().is_zero() {
                panic!("0 to the power of 0 is undefined!");
            }
            return Term::default();
        }

        if power < 0 && self.multiplier().is_zero() {
            panic!("Division by 0!");
        }

        self.multiplier = self.multiplier.pow(
            i32::try_from(power).expect("Term power exceeds TermMultiplierType exponent range"),
        );

        for current_power in &mut self.variables.values_mut() {
            *current_power *= power;
        }

        self
    }

    /// Substitutes a variable raised to `power` with another term.
    pub fn substitute_variable_power_with_term(
        mut self,
        variable: Variable,
        power: TermVariablePowerType,
        term: Term,
    ) -> Self {
        if let Some(current_power) = self.variables.get_mut(&variable) {
            let (q, r) = num_integer::div_rem(*current_power, power);
            self.variables.remove(&variable);
            let multiplier = term.pow(q) * variable.pow(r);
            self *= multiplier;
        }

        self
    }

    /// Substitutes a variable raised to a `power` with an expression.
    pub fn substitute_variable_power_with_expression(
        mut self,
        variable: Variable,
        power: TermVariablePowerType,
        subst: Expression,
    ) -> Expression {
        if let Some(current_power) = self.variables.get_mut(&variable) {
            let (q, r) = num_integer::div_rem(*current_power, power);
            self.variables.remove(&variable);

            subst.pow(q) * variable.pow(r) * self
        } else {
            self.into()
        }
    }

    /// Substitutes multiple variables with corresponding term.
    pub fn substitute_multiple_variables_with_terms<T, U>(mut self, substitutions: T) -> Self
    where
        T: IntoIterator<Item = (Variable, U)>,
        U: Into<Term>,
    {
        let mut substituted_terms = Term::default();
        let mut seen = HashSet::new();
        for (variable, value) in substitutions.into_iter() {
            if !seen.insert(variable) {
                panic!("Duplicate substitutions found for {variable}!");
            }
            if let Some(power) = self.variables.remove(&variable) {
                substituted_terms *= Into::<Term>::into(value).pow(power);
            }
        }

        self * substituted_terms
    }

    /// Substitutes multiple variables with corresponding expressions.
    pub fn substitute_multiple_variables_with_expressions<T>(
        mut self,
        substitutions: T,
    ) -> Expression
    where
        T: IntoIterator<Item = (Variable, Expression)>,
    {
        let mut substituted_expression = Into::<Expression>::into(Term::default());

        let mut seen = HashSet::new();
        for (variable, value) in substitutions.into_iter() {
            if !seen.insert(variable) {
                panic!("Duplicate substitutions found for {variable}!");
            }
            if let Some(power) = self.variables.remove(&variable) {
                substituted_expression *= value.pow(power);
            }
        }

        substituted_expression * self
    }

    /// Checks if the term is divisible by a variable raised to a specific power.
    pub fn is_divisible_by_variable_power(
        &self,
        divisor: Variable,
        power: TermVariablePowerType,
    ) -> bool {
        self.variables
            .get(&divisor)
            .is_some_and(|p| *p != 0 && *p >= power)
    }

    /// Checks if the term is divisible by input term. Only the input term's variables are considered.
    /// Returns true if all divisor variables have lower power than this term's variables.
    pub fn is_divisible_by(&self, divisor: impl Into<Term>) -> bool {
        for (variable, power) in Into::<Term>::into(divisor).variables {
            if !self.is_divisible_by_variable_power(variable, power) {
                return false;
            }
        }

        true
    }

    /// Returns true if the multiplier is non zero and variable has non zero power in term.
    pub fn contains_variable(&self, variable: &Variable) -> bool {
        if let Some(power) = self.variables.get(variable) {
            *power != 0 && self.multiplier != 0.into()
        } else {
            false
        }
    }

    /// Returns the power of the specified variable in the term, or 0 if the variable is not present.
    pub fn get_power_of_variable(&self, variable: &Variable) -> TermVariablePowerType {
        self.variables.get(variable).copied().unwrap_or(0)
    }

    /// Set the power of the specified variable in the term.
    /// Implicitly a term contains all variables raised to power of zero
    /// So this in essence will create the variable even if term previously did not contain it
    /// as long as power is non zero
    pub fn set_power_of_variable(&mut self, variable: &Variable, power: TermVariablePowerType) {
        if power == 0 {
            self.variables.remove_entry(variable);
        } else {
            self.variables
                .entry(*variable)
                .and_modify(|p| *p = power)
                .or_insert(power);
        }
    }

    /// Return true if the term is a scalar (i.e., it has no variables).
    pub fn is_scalar(&self) -> bool {
        self.variables.values().all(|p| *p == 0)
    }

    /// Removes the specified variables from the term and returns a new term containing only the removed variables and their powers.
    pub fn remove_variables<'a>(&mut self, variables: impl Iterator<Item = &'a Variable>) -> Self {
        let mut residue = Term::default();

        for variable in variables {
            if let Some(power) = self.variables.remove(variable)
                && power != 0
            {
                residue.variables.insert(*variable, power);
            }
        }

        residue
    }

    /// Compute LCM of terms. Returns a term that is the least common multiple of the input terms.
    /// The LCM is computed by taking the maximum power of each variable across all terms.
    /// LCM of multipliers is not considered
    pub fn lcm(terms: impl IntoIterator<Item = Term>) -> Term {
        let mut lcm_multiplier = TermMultiplierType::new(1, 1);

        let mut variables_powers: HashMap<Variable, TermVariablePowerType> = HashMap::new();

        for term in terms {
            lcm_multiplier *= term.multiplier;
            for (variable, power) in term.variables {
                variables_powers
                    .entry(variable)
                    .and_modify(|max_power| *max_power = (*max_power).max(power))
                    .or_insert(power);
            }
        }

        let lcm_variables = variables_powers
            .into_iter()
            .map(|(v, p)| v.pow(p))
            .fold(Term::default(), Mul::mul);

        lcm_multiplier * lcm_variables
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

impl From<Variable> for Term {
    fn from(value: Variable) -> Self {
        let mut term = Self::default();
        term.variables.insert(value, 1);
        term
    }
}

impl TryInto<Variable> for Term {
    type Error = String;

    fn try_into(self) -> Result<Variable, Self::Error> {
        let mut variable_powers = self
            .variables
            .into_iter()
            .filter(|(_, power)| *power != 0)
            .collect::<Vec<_>>();
        if self.multiplier != 1.into()
            || variable_powers.len() != 1
            || variable_powers.first().expect("Check Length above").1 != 0
        {
            Err("Cannot be reduced to a variable!".to_string())
        } else {
            Ok(variable_powers.pop().expect("Check Length above").0)
        }
    }
}

impl<T> From<T> for Term
where
    T: Into<TermMultiplierType>,
{
    fn from(value: T) -> Self {
        Self {
            variables: Default::default(),
            multiplier: Into::<TermMultiplierType>::into(value),
        }
    }
}

impl Product for Term {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Mul::mul)
    }
}

impl PartialDerivative for Term {
    fn calculate_derivate_wrt_variable(&self, variable: &Variable) -> Self {
        let mut output = self.clone();
        let mut residue = output.remove_variables(std::iter::once(variable));
        let power = residue.get_power_of_variable(variable);
        residue.set_power_of_variable(variable, power - 1);
        residue *= TermMultiplierType::from(power as TermVariablePowerType);
        residue * output
    }
    fn calculate_nth_derivate_wrt_variable(&self, n: usize, variable: &Variable) -> Self {
        let mut output = self.clone();
        let mut residue = output.remove_variables(std::iter::once(variable));
        let power = residue.get_power_of_variable(variable);
        let smallest = power - n as TermVariablePowerType;
        let mut multiplier = 1;
        for m in (smallest + 1)..=power {
            multiplier *= m;
        }
        residue.set_power_of_variable(variable, smallest);
        residue *= TermMultiplierType::from(multiplier);
        residue * output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "0 to the power of 0 is undefined!")]
    fn test_term_creation_and_pow() {
        let x = Variable::new('x', None);
        let term = x.pow(3) * 2;

        assert_eq!(term.multiplier(), TermMultiplierType::new(2, 1));
        assert!(term.contains_variable(&x));
        assert_eq!(term.get_power_of_variable(&x), 3);
        assert!(!term.is_scalar());

        let _ = Term::from(0).pow(0);
    }

    #[test]
    #[should_panic(expected = "Division by 0!")]
    fn test_term_neg_pow() {
        let x = Variable::new('x', None);
        let term = x.pow(3) * 2;

        assert_eq!(term.multiplier(), TermMultiplierType::new(2, 1));
        assert!(term.contains_variable(&x));
        assert_eq!(term.get_power_of_variable(&x), 3);
        assert!(!term.is_scalar());

        let _ = Term::from(0).pow(-1);
    }

    #[test]
    #[should_panic(expected = "Division by 0!")]
    fn test_term_operator_coverage() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);

        let xy = x * y;
        let x_squared = x.pow(2);
        let sum = xy.clone() + x_squared.clone();
        let difference = x_squared.clone() - y;
        let product = xy.clone() * x;
        let quotient = x_squared.clone() / y;
        let quotient_by_term = x_squared.clone() / x_squared.clone();

        let expression = Expression::from(y);
        let term_plus_expression = x_squared.clone() + expression.clone();
        let term_minus_expression = x_squared.clone() - expression.clone();
        let term_mul_expression = x_squared.clone() * expression.clone();

        let scalar_sum = xy.clone() + TermMultiplierType::new(1, 1);
        let scalar_difference = xy.clone() - TermMultiplierType::new(1, 1);
        let scalar_product = xy.clone() * TermMultiplierType::new(2, 1);
        let scalar0_product = xy.clone() * TermMultiplierType::new(0, 1);
        let scalar_quotient = xy.clone() / TermMultiplierType::new(2, 1);
        let scalar_minus_term = TermMultiplierType::new(3, 1) - xy.clone();
        let scalar_divide_term = TermMultiplierType::new(3, 1) / xy.clone();

        assert_eq!(sum.to_string(), "xy + x²");
        assert_eq!(difference.to_string(), "x² - y");
        assert_eq!(product.to_string(), "x²y");
        assert_eq!(quotient.to_string(), "x²y⁻¹");
        assert_eq!(quotient_by_term.to_string(), "1");

        assert_eq!(term_plus_expression.to_string(), "x² + y");
        assert_eq!(term_minus_expression.to_string(), "x² - y");
        assert_eq!(term_mul_expression.to_string(), "x²y");

        assert_eq!(scalar_sum.to_string(), "1 + xy");
        assert_eq!(scalar_difference.to_string(), "-1 + xy");
        assert_eq!(scalar_product.multiplier(), TermMultiplierType::new(2, 1));
        assert_eq!(scalar_product.to_string(), "2xy");
        assert_eq!(scalar0_product.multiplier(), TermMultiplierType::new(0, 1));
        assert_eq!(scalar0_product.to_string(), "0");
        assert_eq!(scalar_quotient.multiplier(), TermMultiplierType::new(1, 2));
        assert_eq!(scalar_quotient.to_string(), "(1/2)xy");
        assert_eq!(scalar_minus_term.to_string(), "3 - xy");
        assert_eq!(scalar_divide_term.to_string(), "3x⁻¹y⁻¹");

        let mut temp = Term::default();
        let zero: Term = 0.into();
        temp /= zero;
    }

    #[test]
    fn test_term_divisibility() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let term = 4 * x.pow(4) * y * z.pow(5);

        let divisible = term.is_divisible_by(x.pow(2));
        let not_divisible = term.is_divisible_by(x.pow(5));

        assert!(divisible);
        assert!(!not_divisible);
    }

    #[test]
    #[should_panic(expected = "Duplicate substitutions found for y!")]
    fn test_term_substitution() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let term = 4 * x.pow(4) * y;

        let substituted = term
            .clone()
            .substitute_variable_power_with_term(x, 3, 5 * z.pow(2));

        assert_eq!(substituted.to_string(), "20xyz²");

        let term = 3 * x.pow(4) * y * z.pow(3);
        let substituted = term.clone().substitute_multiple_variables_with_terms([
            (y, x.pow(5)),
            (z, TermMultiplierType::new(7, 8) * x.pow(4)),
        ]);

        assert_eq!(substituted.to_string(), "(1029/512)x²¹");

        let _ = term
            .clone()
            .substitute_multiple_variables_with_terms([(y, x), (y, z)]);
    }

    #[test]
    #[should_panic(expected = "Duplicate substitutions found for y!")]
    fn test_expression_substitution() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let term = 4 * x.pow(4) * y;

        let substituted =
            term.clone()
                .substitute_variable_power_with_expression(x, 3, 5 * z.pow(2) + z);
        assert_eq!(substituted.to_string(), "4xyz + 20xyz²");

        let term = 3 * x.pow(4) * y * z.pow(3);
        let substituted = term
            .clone()
            .substitute_multiple_variables_with_expressions([
                (y, x.pow(5) + x),
                (z, TermMultiplierType::new(7, 8) * x.pow(4) + x),
            ]);

        assert_eq!(
            substituted.to_string(),
            "3x⁸ + (63/8)x¹¹ + 3x¹² + (441/64)x¹⁴ + (63/8)x¹⁵ + (1029/512)x¹⁷ + (441/64)x¹⁸ + (1029/512)x²¹"
        );

        let _ = term
            .clone()
            .substitute_multiple_variables_with_expressions([(y, x + z), (y, z + x)]);
    }

    #[test]
    #[should_panic(
        expected = "Raising a expression to a negative power is not supported in this version!"
    )]
    fn test_expression_substitution_unsupported_division() {
        let x = Variable::new('x', None);
        let z = Variable::new('z', None);
        let term = 4 * x.pow(8);

        let _ = term
            .clone()
            .substitute_variable_power_with_expression(x, -2, 5 * z.pow(2) + z);
    }

    #[test]
    fn test_term_sympy_formatting() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let term = Term {
            variables: BTreeMap::from([(x, 1), (y, 2)]),
            multiplier: TermMultiplierType::new(1, 1),
        };

        assert_eq!(term.to_string_sympy(), "x*(y**2)");
        assert_eq!(
            (TermMultiplierType::new(2, 1) * term.clone()).to_string_sympy(),
            "2*x*(y**2)"
        );
        assert_eq!(
            (TermMultiplierType::new(4, 3) * term.clone()).to_string_sympy(),
            "(4/3)*x*(y**2)"
        );

        let term = Term {
            variables: BTreeMap::from([(x, 2), (y, 3)]),
            multiplier: TermMultiplierType::new(1, 1),
        };

        assert_eq!(term.to_string_sympy(), "(x**2)*(y**3)");

        let term = Term {
            variables: BTreeMap::from([(x, 2)]),
            multiplier: TermMultiplierType::new(1, 1),
        };

        assert_eq!(term.to_string_sympy(), "x**2");

        let termsub = Variable::new('z', Some(1)) * term;

        assert_eq!(termsub.to_string_sympy(), "(x**2)*z_1");
    }

    #[test]
    fn test_term_mul_assign_zero_cancellation() {
        let x = Variable::new('x', None);
        let mut term = x.pow(2);

        term *= x.pow(-2);

        assert_eq!(term.to_string(), "1");
    }

    #[test]
    fn test_term_remove_variables_and_scalar_behavior() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let mut term = x.pow(2) * y.pow(3) * 4;

        let residue = term.remove_variables([&x].into_iter());
        let scalar = Term::from(7);

        assert_eq!(residue, x.pow(2));
        assert!(scalar.is_scalar());
        assert!(term.contains_variable(&y));
        assert!(!term.contains_variable(&x));
    }

    #[test]
    fn test_term_lcm() {
        let w = Variable::new('w', None);
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let z = Variable::new('z', None);
        let terms = vec![
            w.pow(-2) * z.pow(-7) * TermMultiplierType::new(2, 7),
            y.pow(3) * x.pow(-7) * TermMultiplierType::new(3, 14),
            x.pow(5) * z.pow(-4) * TermMultiplierType::new(5, 3),
        ];

        let lcm = Term::lcm(terms);
        let expected = TermMultiplierType::new(2, 7)
            * TermMultiplierType::new(3, 14)
            * TermMultiplierType::new(5, 3)
            * w.pow(-2)
            * y.pow(3)
            * x.pow(5)
            * z.pow(-4);
        assert_eq!(lcm.to_string(), expected.to_string());
    }
}
