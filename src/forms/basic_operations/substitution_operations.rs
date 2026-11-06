use crate::{
    Expression, RationalExpression, Term, TermVariablePowerType, Variable,
    forms::basic_traits::Form,
};

/**
 * Replaces variables in a symbolic expression with their corresponding values.
 */
pub trait VariableSubstitutions<T: Form> {
    /// The symbolic form produced by the substitution.
    type Output;

    /// Substitutes multiple variables with provided corresponding values.
    fn substitute_variables(
        self,
        substitutions: impl IntoIterator<Item = (Variable, impl Into<T>)>,
    ) -> Self::Output;
}

/**
 * Replaces a variable raised to a known power with a supplied value.
 */
pub trait VariablePowerSubstitutions<T: Form> {
    /// The symbolic form produced by the substitution.
    type Output;

    /// Substitutes a variable power with the provided replacement value.
    fn substitute_variable_power(
        self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: T,
    ) -> Self::Output;
}

impl<T: Form + Into<Term>> VariablePowerSubstitutions<T> for Term {
    type Output = Term;

    fn substitute_variable_power(
        mut self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: T,
    ) -> Term {
        if let Some(current_power) = self.variables.get_mut(&variable) {
            let (quotient, remainder) = num_integer::div_rem(*current_power, known_power);
            self.variables.remove(&variable);
            subst.into().pow(quotient) * variable.pow(remainder) * self
        } else {
            self
        }
    }
}

impl VariablePowerSubstitutions<Expression> for Term {
    type Output = Expression;

    fn substitute_variable_power(
        mut self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: Expression,
    ) -> Expression {
        if let Some(current_power) = self.variables.get_mut(&variable) {
            let (quotient, remainder) = num_integer::div_rem(*current_power, known_power);
            self.variables.remove(&variable);
            Into::<Expression>::into(subst).pow(quotient) * variable.pow(remainder) * self
        } else {
            self.into()
        }
    }
}

impl<T: Form + Into<Expression>> VariablePowerSubstitutions<T> for Expression {
    type Output = Expression;

    fn substitute_variable_power(
        self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: T,
    ) -> Expression {
        let mut result = Expression::default();
        for term in self.dissolve_into_terms() {
            result += VariablePowerSubstitutions::<Expression>::substitute_variable_power(
                term,
                variable,
                known_power,
                subst.clone().into(),
            );
        }

        result
    }
}

impl VariablePowerSubstitutions<Expression> for RationalExpression {
    type Output = RationalExpression;

    fn substitute_variable_power(
        self,
        variable: Variable,
        known_power: TermVariablePowerType,
        subst: Expression,
    ) -> RationalExpression {
        let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
        let numerator = VariablePowerSubstitutions::<Expression>::substitute_variable_power(
            numerator,
            variable,
            known_power,
            subst.clone(),
        );
        let denominator = VariablePowerSubstitutions::<Expression>::substitute_variable_power(
            denominator,
            variable,
            known_power,
            subst,
        );

        RationalExpression::new(numerator, denominator)
    }
}

macro_rules! impl_term_substitutions {
    ($substitution:ty) => {
        impl VariableSubstitutions<$substitution> for Term {
            type Output = $substitution;

            fn substitute_variables(
                mut self,
                substitutions: impl IntoIterator<Item = (Variable, impl Into<$substitution>)>,
            ) -> $substitution {
                let mut substituted_terms: $substitution = 1_i64.into();
                let mut seen = std::collections::HashSet::new();
                for (variable, value) in substitutions {
                    if !seen.insert(variable) {
                        panic!("Duplicate substitutions found for {variable}!");
                    }
                    if let Some(power) = self.variables.remove(&variable) {
                        substituted_terms *= Into::<$substitution>::into(value).pow(power);
                    }
                }

                self * substituted_terms
            }
        }
    };
}

impl_term_substitutions!(Term);
impl_term_substitutions!(Expression);
impl_term_substitutions!(RationalExpression);

macro_rules! impl_expression_substitutions {
    ($input:ty, $normalized:ty, $output:ty) => {
        impl VariableSubstitutions<$input> for Expression {
            type Output = $output;

            fn substitute_variables(
                self,
                substitutions: impl IntoIterator<Item = (Variable, impl Into<$input>)>,
            ) -> $output {
                let substitutions: Vec<(Variable, $normalized)> = substitutions
                    .into_iter()
                    .map(|(variable, value)| {
                        let value: $input = value.into();
                        (variable, value.into())
                    })
                    .collect();
                let mut result = <$output>::default();
                for term in self.dissolve_into_terms() {
                    result += VariableSubstitutions::<$normalized>::substitute_variables(
                        term,
                        substitutions.clone(),
                    );
                }

                result
            }
        }
    };
}

impl_expression_substitutions!(Term, Expression, Expression);
impl_expression_substitutions!(Expression, Expression, Expression);
impl_expression_substitutions!(RationalExpression, RationalExpression, RationalExpression);

macro_rules! impl_rational_expression_substitutions {
    ($input:ty) => {
        impl VariableSubstitutions<$input> for RationalExpression {
            type Output = RationalExpression;

            fn substitute_variables(
                self,
                substitutions: impl IntoIterator<Item = (Variable, impl Into<$input>)>,
            ) -> RationalExpression {
                let substitutions: Vec<(Variable, RationalExpression)> = substitutions
                    .into_iter()
                    .map(|(variable, value)| {
                        let value: $input = value.into();
                        (variable, value.into())
                    })
                    .collect();
                let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
                let substituted_numerator =
                    VariableSubstitutions::<RationalExpression>::substitute_variables(
                        numerator,
                        substitutions.clone(),
                    );
                let substituted_denominator =
                    VariableSubstitutions::<RationalExpression>::substitute_variables(
                        denominator,
                        substitutions,
                    );

                substituted_numerator / substituted_denominator
            }
        }
    };
}

impl_rational_expression_substitutions!(Term);
impl_rational_expression_substitutions!(Expression);
impl_rational_expression_substitutions!(RationalExpression);
