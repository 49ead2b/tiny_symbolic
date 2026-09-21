use crate::Variable;

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
