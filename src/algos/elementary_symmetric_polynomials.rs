use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul},
};

use itertools::Itertools;

use crate::{
    TermVariablePowerType,
    forms::{expression::Expression, term::Term, variable::Variable},
};

/// A small symbolic engine for rewriting symmetric expressions using elementary symmetric
/// polynomials.
///
/// The implementation follows a simple pipeline:
/// 1. Group terms by their exponent pattern.
/// 2. Check that the grouped structure is compatible with symmetry.
/// 3. Rewrite each pattern recursively using Newton-Girard identities.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ElementarySymmetricPolynomials {
    elementary_polynomial_representations: HashMap<usize, Expression>,
    elementary_polynomial_substitutions: HashMap<usize, Expression>,
    variables: HashSet<Variable>,
    cache_power_sums: HashMap<Vec<TermVariablePowerType>, Expression>,
}

impl ElementarySymmetricPolynomials {
    /// Build an elementary symmetric polynomial tracker from a collection of variables.
    pub fn from_variables<T>(variables: &T) -> Self
    where
        for<'a> &'a T: IntoIterator<Item = &'a Variable>,
    {
        let combinations = Self::generate_all_combinations(variables);

        let mut obj = Self {
            elementary_polynomial_representations: combinations.into_iter().collect(),
            elementary_polynomial_substitutions: Default::default(),
            variables: variables.into_iter().copied().collect(),
            cache_power_sums: Default::default(),
        };

        // Seed the recursive cache with the simplest base cases.
        obj.cache_power_sums.insert(
            vec![0],
            (obj.variables.len() as TermVariablePowerType).into(),
        );
        if obj.variable_count() >= 1 {
            obj.cache_power_sums
                .insert(vec![1], Variable::new('e', Some(1)).into());
        }
        if obj.variable_count() >= 2 {
            obj.cache_power_sums.insert(
                vec![2],
                Variable::new('e', Some(1)).pow(2) - 2 * Variable::new('e', Some(2)),
            );
        }

        obj
    }

    /// Build the original elementary symmetric polynomial representations for each degree.
    fn generate_all_combinations<T>(variables: &T) -> impl IntoIterator<Item = (usize, Expression)>
    where
        for<'a> &'a T: IntoIterator<Item = &'a Variable>,
    {
        let variables_vec = variables.into_iter().copied().collect::<Vec<_>>();
        (0..variables_vec.len()).map(move |k| {
            (
                k + 1,
                variables_vec
                    .iter()
                    .combinations(k + 1)
                    .map(|chosen_variables| {
                        chosen_variables
                            .into_iter()
                            .copied()
                            .fold(Term::default(), Mul::mul)
                    })
                    .fold(Expression::default(), Add::add),
            )
        })
    }

    /// Get an iterator for the variables of the ESP
    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables.iter()
    }

    /// Set a substitution for the sum of all distinct products of k distinct variables
    /// This will be used while simplifying a ESP using simplify() method
    pub fn set_ek_substitution_as(&mut self, k: usize, new_expression: Expression) {
        if k < 1 || k > self.variables.len() {
            panic!("k is outside range for ESP!");
        }
        self.elementary_polynomial_substitutions
            .insert(k, new_expression);
    }

    #[allow(dead_code)]
    fn get_ek_original_representation(&self, k: usize) -> &Expression {
        self.elementary_polynomial_representations
            .get(&k)
            .expect("k is outside range for this ESP!")
    }

    /// Get the substitution for the sum of all distinct products of k distinct variables
    /// Set using set_ek_substitution_as() method
    pub fn get_ek_substitution(&self, k: usize) -> &Expression {
        self.elementary_polynomial_substitutions
            .get(&k)
            .expect("k is outside range for this ESP or substitution not set!")
    }

    /// Number of variables which make up the ESP
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Whether this variable is part of the ESP variables
    pub fn is_variable_present(&self, variable: &Variable) -> bool {
        self.variables.contains(variable)
    }

    fn count_occurrences(
        values: &[TermVariablePowerType],
    ) -> HashMap<TermVariablePowerType, usize> {
        let mut counts = HashMap::new();
        for value in values {
            *counts.entry(*value).or_insert(0) += 1;
        }
        counts
    }

    fn factorial(num: usize) -> usize {
        (1..=num).product()
    }

    /// Compute the symmetry factor for an exponent pattern.
    ///
    /// If the pattern contains repeated exponents, the sum must be divided by the product of
    /// the factorials of those repetitions to avoid overcounting.
    fn symmetry_factor_for_pattern(&self, pattern: &[TermVariablePowerType]) -> usize {
        Self::count_occurrences(pattern)
            .values()
            .map(|count| Self::factorial(*count))
            .product()
    }

    /// Build a normalized symmetric sum for a given exponent pattern.
    #[allow(dead_code)]
    fn generate_symmetric_expression(&self, powers: &[TermVariablePowerType]) -> Expression {
        let common_factor = self.symmetry_factor_for_pattern(powers) as TermVariablePowerType;
        let number_of_slots = powers.len();
        self.variables
            .iter()
            .permutations(number_of_slots)
            .map(|chosen_variables| {
                chosen_variables
                    .into_iter()
                    .copied()
                    .zip(powers.iter())
                    .map(|(variable, power)| variable.pow(*power))
                    .fold(Term::default(), Mul::mul)
            })
            .fold(Expression::default(), Add::add)
            / common_factor
    }

    /// Extract the exponent pattern of a term, keeping only the variables in this system.
    fn power_pattern_for_term(&self, term: &Term) -> Vec<TermVariablePowerType> {
        term.variables()
            .iter()
            .filter(|(variable, _)| self.variables.contains(*variable))
            .map(|(variable, _)| term.get_power_of_variable(variable))
            .sorted()
            .rev()
            .collect()
    }

    /// Partition an expression into groups of terms sharing the same exponent pattern.
    fn partition_into_symmetric_expression(
        &self,
        input: &Expression,
    ) -> HashMap<Vec<TermVariablePowerType>, Expression> {
        let coefficient_groups = self.group_terms_by_power_pattern(input);
        if !self.is_valid_symmetric_pattern_map(&coefficient_groups) {
            panic!("Not a symmetric expression!");
        }

        let mut result = HashMap::new();
        for (pattern, terms) in coefficient_groups {
            let coefficient_expression = terms.into_keys().fold(Expression::default(), Add::add);
            result.insert(pattern, coefficient_expression);
        }

        result
    }

    /// Group terms by exponent pattern and count how many times each coefficient appears.
    fn group_terms_by_power_pattern(
        &self,
        input: &Expression,
    ) -> HashMap<Vec<TermVariablePowerType>, HashMap<Term, usize>> {
        let mut groups: HashMap<Vec<TermVariablePowerType>, HashMap<Term, usize>> =
            HashMap::default();

        for mut term in input.terms() {
            let pattern = self.power_pattern_for_term(&term);
            term.remove_variables(self.variables());
            let multiplier = term;

            if let Some(group) = groups.get_mut(&pattern) {
                *group.entry(multiplier).or_insert(0) += 1;
            } else {
                let mut group = HashMap::new();
                group.insert(multiplier, 1);
                groups.insert(pattern, group);
            }
        }

        groups
    }

    /// Check whether the grouping respects the expected multiplicities of a symmetric expression.
    fn is_valid_symmetric_pattern_map(
        &self,
        groups: &HashMap<Vec<TermVariablePowerType>, HashMap<Term, usize>>,
    ) -> bool {
        for (pattern, terms) in groups.iter() {
            let total_permutations = Self::factorial(self.variable_count())
                / Self::factorial(self.variable_count() - pattern.len());
            let symmetry_factor = self.symmetry_factor_for_pattern(pattern);
            let expected_multiplicity = total_permutations / symmetry_factor;

            for multiplicity in terms.values() {
                if *multiplicity != expected_multiplicity {
                    return false;
                }
            }
        }

        true
    }

    /// Check if input expression is symmetric with respect to the variables in this.
    pub fn is_symmetric(&self, input: &Expression) -> bool {
        let groups = self.group_terms_by_power_pattern(input);
        self.is_valid_symmetric_pattern_map(&groups)
    }

    fn minus_1_pow(k: TermVariablePowerType) -> TermVariablePowerType {
        if k % 2 == 0 { 1 } else { -1 }
    }

    fn ek_as_variable_expression(&self, k: TermVariablePowerType) -> Expression {
        if k == 0 {
            1.into()
        } else if k as usize <= self.variable_count() {
            Variable::new('e', Some(k)).into()
        } else {
            0.into()
        }
    }

    /// Recursively compute the power sums using the Newton-Girard recurrence.
    fn get_sk_using_newton_girard_formula(&mut self, k: TermVariablePowerType) -> Expression {
        let cache_key = vec![k];
        if let Some(cached_value) = self.cache_power_sums.get(&cache_key) {
            return cached_value.clone();
        }

        let sign_for_k = Self::minus_1_pow(k - 1);
        let mut output = sign_for_k * k * self.ek_as_variable_expression(k);

        for i in 1..k {
            let sign_for_i = Self::minus_1_pow(i - 1);
            let elementary_i = self.ek_as_variable_expression(i);
            output += sign_for_i * elementary_i * self.get_sk_using_newton_girard_formula(k - i);
        }

        self.cache_power_sums.insert(cache_key, output.clone());
        output
    }

    /// Rewrite a power pattern into an expression built from elementary symmetric polynomials.
    fn expression_for_power_pattern(&mut self, pattern: &[TermVariablePowerType]) -> Expression {
        let normalized_pattern = pattern.iter().copied().sorted().rev().collect::<Vec<_>>();

        if normalized_pattern.is_empty() {
            Term::default().into()
        } else if normalized_pattern.len() == 1 {
            self.get_sk_using_newton_girard_formula(normalized_pattern[0])
        } else if normalized_pattern.first() == normalized_pattern.last()
            && *normalized_pattern.first().unwrap() == 1
        {
            (Self::factorial(normalized_pattern.len()) as TermVariablePowerType)
                * self.ek_as_variable_expression(normalized_pattern.len() as TermVariablePowerType)
        } else {
            let split_index = normalized_pattern.len() - 1;
            let left_pattern = &normalized_pattern[..split_index];
            let right_pattern = &normalized_pattern[split_index..];

            let mut output = self.expression_for_power_pattern(left_pattern)
                * self.expression_for_power_pattern(right_pattern);

            for index in 0..left_pattern.len() {
                let mut extended_left = left_pattern.to_vec();
                extended_left[index] += right_pattern[0];
                output -= self.expression_for_power_pattern(&extended_left);
            }

            output
        }
    }

    fn get_min_power_in_expression(&self, input: &Expression) -> TermVariablePowerType {
        input
            .terms()
            .into_iter()
            .map(|term| {
                self.variables
                    .iter()
                    .map(|variable| term.get_power_of_variable(variable))
                    .min()
                    .unwrap_or_default()
            })
            .min()
            .unwrap_or_default()
    }

    /// Simplify a symmetric expression by rewriting it in terms of elementary symmetric polynomials.
    pub fn simplify_symmetric_expression(&mut self, mut input: Expression) -> Expression {
        let min_power = self.get_min_power_in_expression(&input);
        if min_power < 0 {
            let denominator =
                self.ek_as_variable_expression(self.variable_count() as TermVariablePowerType);
            let numerator = self
                .variables()
                .map(|variable| variable.pow(-min_power))
                .fold(Term::default(), Mul::mul);
            input *= numerator * denominator.pow(min_power);
        }
        let partitions = self.partition_into_symmetric_expression(&input);
        let mut output = Expression::default();

        for (pattern, multiplier) in partitions {
            let symmetry_factor =
                self.symmetry_factor_for_pattern(&pattern) as TermVariablePowerType;
            let pattern_expression = self.expression_for_power_pattern(&pattern);
            output += multiplier * pattern_expression / symmetry_factor;
        }

        let mut substitutions = HashSet::new();
        for k in 1..=self.variables.len() {
            let variable = Variable::new('e', Some(k as i64));
            if let Some(substitution) = self.elementary_polynomial_substitutions.get(&(k)) {
                substitutions.insert((variable, substitution.clone()));
            }
        }

        output.substitute_multiple_variables_with_expressions(substitutions)
    }
}

#[cfg(test)]
mod tests {
    use crate::TermMultiplierType;

    use super::*;
    use crate::forms::variable::Variable;

    #[test]
    fn test_is_symmetric() {
        let mvar = Variable::new('m', None);
        let (alpha, beta, gamma) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let f_z = 4 * (alpha + beta - gamma).pow(2) + 8 * (alpha + beta + gamma).pow(3) + alpha
            - mvar * beta
            + mvar.pow(2) * gamma;

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![alpha, beta, gamma]);
        esp.set_ek_substitution_as(1, Variable::new('p', Some(1)).into());
        esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
        esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());

        let output = esp.is_symmetric(&f_z);
        assert!(!output);

        let (alpha, beta, gamma) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let f_z = 4 * (alpha + beta + gamma).pow(2) + 8 * (alpha + beta + gamma).pow(12);

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![alpha, beta, gamma]);
        esp.set_ek_substitution_as(1, Variable::new('p', Some(1)).into());
        esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
        esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());

        let output = esp.is_symmetric(&f_z);
        assert!(output);

        let mvar = Variable::new('m', None);
        let (alpha, beta, gamma) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let f_z = 4 * mvar * (alpha + beta + gamma).pow(2);

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![alpha, beta, gamma]);
        esp.set_ek_substitution_as(1, Variable::new('p', Some(1)).into());
        esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
        esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());

        let output = esp.is_symmetric(&f_z);
        assert!(output);
    }

    #[test]
    fn test_simplify_symmetric() {
        let mvar = Variable::new('m', None);
        let (alpha, beta, gamma) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let f_z = 4 * mvar * (alpha + beta + gamma).pow(2) * mvar * (alpha * beta * gamma).pow(2)
            + (alpha * beta + beta * gamma + gamma * alpha).pow(4);

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![alpha, beta, gamma]);

        let p1 = Variable::new('p', Some(1));
        let p2 = Variable::new('p', Some(2));
        let p3 = Variable::new('p', Some(3));

        esp.set_ek_substitution_as(1, p1.into());
        esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
        esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());

        let actual = esp.simplify_symmetric_expression(f_z);
        let expected = 4 * mvar * p1.pow(2) * mvar * p3.pow(2) + p2.pow(4);

        assert!(expected == actual);

        let f_z = 1 / alpha + 1 / beta + 1 / gamma;

        let actual = esp.simplify_symmetric_expression(f_z);
        let expected = p2 / p3;

        assert_eq!(expected.to_string(), actual.to_string());

        let f_z = Term::from(TermMultiplierType::new(2, 3));

        let actual = esp.simplify_symmetric_expression(f_z.clone().into());

        assert_eq!(actual.to_string(), f_z.to_string());
    }

    #[test]
    fn test_simplify_symmetric_large_1() {
        let (a, b, c, d) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
            Variable::new('D', None),
        );
        let f_z = 4 * (a + b + c + d).pow(20)
            + TermMultiplierType::new(7, 8)
                * (a + b + c + d).pow(5)
                * (a * b + a * c + a * d + b * c + b * d + c * d).pow(3);

        let p1 = Variable::new('p', Some(1));
        let p2 = Variable::new('p', Some(2));
        let p3 = Variable::new('p', Some(3));
        let p4 = Variable::new('p', Some(4));

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c, d]);
        esp.set_ek_substitution_as(1, p1.into());
        esp.set_ek_substitution_as(2, p2.into());
        esp.set_ek_substitution_as(3, p3.into());
        esp.set_ek_substitution_as(4, p4.into());

        let actual = esp.simplify_symmetric_expression(f_z);
        let expected = 4 * p1.pow(20) + TermMultiplierType::new(7, 8) * p1.pow(5) * p2.pow(3);

        assert_eq!(actual, expected);

        let f_z = 1 / a.pow(2) + 1 / b.pow(2) + 1 / c.pow(2) + 1 / d.pow(2);

        let actual = esp.simplify_symmetric_expression(f_z);
        let expected = esp.simplify_symmetric_expression(
            (a * b * c).pow(2) + (a * b * d).pow(2) + (a * c * d).pow(2) + (b * c * d).pow(2),
        ) / p4.pow(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simplify_symmetric_large_2() {
        let (a, b, c, d) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
            Variable::new('D', None),
        );

        let e1 = a + b + c + d;
        let e2 = a * b + a * c + a * d + b * c + b * d + c * d;
        let e3 = a * b * c + a * b * d + a * c * d + b * c * d;
        let e4 = a * b * c * d;

        let m = Variable::new('m', None);

        let f_z = TermMultiplierType::new(4, 3) * e1.pow(3) * e2.pow(2) * e3.pow(3) * e4.pow(4) + m;

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c, d]);
        esp.set_ek_substitution_as(1, Variable::new('p', Some(1)).into());
        esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
        esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());
        esp.set_ek_substitution_as(4, Variable::new('p', Some(4)).into());

        let actual = esp.simplify_symmetric_expression(f_z);
        let expected = TermMultiplierType::new(4, 3)
            * Variable::new('p', Some(1)).pow(3)
            * Variable::new('p', Some(2)).pow(2)
            * Variable::new('p', Some(3)).pow(3)
            * Variable::new('p', Some(4)).pow(4)
            + m;

        assert!(expected == actual);
    }

    #[test]
    fn test_power_pattern_and_symmetric_helpers() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);

        assert_eq!(esp.variable_count(), 3);
        assert!(esp.is_variable_present(&a));
        assert!(!esp.is_variable_present(&Variable::new('Z', None)));
        assert_eq!(
            esp.get_ek_original_representation(1).to_string(),
            "A + B + C"
        );
        assert!(esp.is_symmetric(&(a * b + b * c + c * a)));
    }

    #[test]
    fn test_newton_girard_formula() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);

        let sk_0 = esp.get_sk_using_newton_girard_formula(0);
        let sk_1 = esp.get_sk_using_newton_girard_formula(1);
        let sk_2 = esp.get_sk_using_newton_girard_formula(2);
        let sk_3 = esp.get_sk_using_newton_girard_formula(3);

        let expected_sk_2 = Variable::new('e', Some(1)).pow(2) - 2 * Variable::new('e', Some(2));
        let expected_sk_3 = Variable::new('e', Some(1)).pow(3)
            - 3 * Variable::new('e', Some(1)) * Variable::new('e', Some(2))
            + 3 * Variable::new('e', Some(3));

        assert_eq!(sk_0, 3.into());
        assert_eq!(sk_1, Variable::new('e', Some(1)).into());
        assert_eq!(sk_2, expected_sk_2);
        assert_eq!(sk_3, expected_sk_3);
    }

    #[test]
    fn test_expression_for_power_pattern_all_ones_branch() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);

        let actual = esp.expression_for_power_pattern(&[1, 1, 1]);
        let expected = Expression::from(6) * Variable::new('e', Some(3));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ek_as_variable_expression_zero_degree_is_one() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);

        let actual = esp.ek_as_variable_expression(0);

        assert_eq!(actual.to_string(), "1");
    }

    #[test]
    fn test_get_ek_substitution_and_generate_symmetric_expression() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);
        let substitution: Expression = Variable::new('p', Some(2)).into();

        esp.set_ek_substitution_as(2, substitution.clone());

        assert_eq!(esp.get_ek_substitution(2), &substitution);
        assert_eq!(
            esp.generate_symmetric_expression(&[1, 1]).to_string(),
            "AB + AC + BC"
        );
    }

    #[test]
    #[should_panic(expected = "k is outside range for ESP!")]
    fn test_set_ek_substitution_panic_message() {
        let (a, b, c) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);
        let substitution: Expression = Variable::new('p', Some(2)).into();

        esp.set_ek_substitution_as(4, substitution.clone());
    }

    #[test]
    #[should_panic(expected = "Not a symmetric expression!")]
    fn test_simplify_substitution_panic_message() {
        let (alpha, beta, gamma) = (
            Variable::new('A', None),
            Variable::new('B', None),
            Variable::new('C', None),
        );
        let f_z = 4 * (alpha - beta + gamma);

        let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![alpha, beta, gamma]);

        let _ = esp.simplify_symmetric_expression(f_z);
    }
}
