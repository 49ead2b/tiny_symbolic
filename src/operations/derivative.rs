use crate::*;

/// Trait for calculating the partial derivative of an expression with respect to a variable.
pub trait PartialDerivative {
    /// Calculates the partial derivative of the implementing type with respect to the specified variable.
    fn calculate_derivate_wrt_variable(&self, variable: &Variable) -> Self;

    /// Calculates the nth partial derivative of the implementing type with respect to the specified variable.
    fn calculate_nth_derivate_wrt_variable(&self, mut n: usize, variable: &Variable) -> Self
    where
        Self: Sized,
    {
        let mut result = self.calculate_derivate_wrt_variable(variable);
        n -= 1;

        while n > 0 {
            result = result.calculate_derivate_wrt_variable(variable);
            n -= 1;
        }

        result
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

impl PartialDerivative for Expression {
    fn calculate_derivate_wrt_variable(&self, variable: &Variable) -> Self {
        let mut output = Expression::default();
        let terms = self.clone().dissolve_into_terms();
        for term in terms {
            output += term.calculate_derivate_wrt_variable(variable);
        }
        output
    }
    fn calculate_nth_derivate_wrt_variable(&self, n: usize, variable: &Variable) -> Self {
        let mut output = Expression::default();
        let terms = self.clone().dissolve_into_terms();
        for term in terms {
            output += term.calculate_nth_derivate_wrt_variable(n, variable);
        }
        output
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
