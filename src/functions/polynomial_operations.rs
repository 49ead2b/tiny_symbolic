/// Assignment operations for symbolic terms and expressions.
pub mod assignment_operations {
    use std::ops::{AddAssign, MulAssign, SubAssign};

    use crate::functions::polynomial::{Polynomial, PolynomialCoefficient};

    impl<T> AddAssign for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + std::ops::Add<Output = T>,
    {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs;
        }
    }

    impl<T> SubAssign for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
    {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs;
        }
    }

    impl<T> MulAssign for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
    {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs;
        }
    }

    #[cfg(test)]
    mod assignment_operations_tests {
        use super::*;
        use crate::*;

        #[test]
        fn test_all_arithmetic_combinations() {
            let x = Variable::new('x', None);
            let polynomial = Polynomial::<Expression>::new(x, vec![1, 1]);

            let mut polynomial_assign = polynomial.clone();
            polynomial_assign += polynomial.clone();
            polynomial_assign -= polynomial.clone();
            polynomial_assign *= polynomial;
        }
    }
}

/// Binary operations for symbolic terms and expressions.
pub mod binary_operations {
    use crate::{functions::polynomial::PolynomialCoefficient, *};
    use std::ops::{Add, Div, Mul, Neg, Sub};

    use crate::functions::polynomial::Polynomial;

    impl<T> Neg for Polynomial<T>
    where
        T: PolynomialCoefficient + Neg<Output = T>,
    {
        type Output = Polynomial<T>;

        fn neg(mut self) -> Self::Output {
            self.coefficients = self.coefficients.into_iter().map(|c| -c).collect();
            self
        }
    }

    impl<T> Add<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + Add<Output = T>,
    {
        type Output = Polynomial<T>;

        fn add(self, rhs: Polynomial<T>) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot add polynomials with different variables!");
            }

            let max_order = usize::max(self.order, rhs.order);
            let mut new_coefficients = vec![T::default(); max_order + 1];

            for (i, coeff) in self.coefficients.into_iter().enumerate() {
                new_coefficients[i] = new_coefficients[i].clone() + coeff;
            }

            for (i, coeff) in rhs.coefficients.into_iter().enumerate() {
                new_coefficients[i] = new_coefficients[i].clone() + coeff;
            }

            while new_coefficients.last().is_some_and(|c| c.is_zero()) {
                new_coefficients.pop();
            }
            Polynomial::new(self.variable, new_coefficients)
        }
    }

    impl<T> Sub<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + Add<Output = T> + Sub<Output = T>,
    {
        type Output = Polynomial<T>;

        fn sub(self, rhs: Polynomial<T>) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot subtract polynomials with different variables!");
            }

            let max_order = usize::max(self.order, rhs.order);
            let mut new_coefficients = vec![T::default(); max_order + 1];

            for (i, coeff) in self.coefficients.into_iter().enumerate() {
                new_coefficients[i] = new_coefficients[i].clone() + coeff;
            }

            for (i, coeff) in rhs.coefficients.into_iter().enumerate() {
                new_coefficients[i] = new_coefficients[i].clone() - coeff;
            }

            while new_coefficients.last().is_some_and(|c| c.is_zero()) {
                new_coefficients.pop();
            }
            Polynomial::new(self.variable, new_coefficients)
        }
    }

    impl<T> Mul<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Polynomial<T>;

        fn mul(self, rhs: Polynomial<T>) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot multiply polynomials with different variables!");
            }

            let mut coefficients = vec![T::default(); self.order + rhs.order + 1];
            for (left_power, left_coefficient) in self.coefficients.into_iter().enumerate() {
                for (right_power, right_coefficient) in rhs.coefficients.iter().enumerate() {
                    let product = left_coefficient.clone() * right_coefficient.clone();
                    coefficients[left_power + right_power] =
                        coefficients[left_power + right_power].clone() + product;
                }
            }

            while coefficients.len() > 1 && coefficients.last().is_some_and(|c| c.is_zero()) {
                coefficients.pop();
            }

            Polynomial::new(self.variable, coefficients)
        }
    }

    impl<T> Div<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Into<RationalExpression>,
    {
        type Output = RationalExpression;

        fn div(self, rhs: Polynomial<T>) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot divide polynomials with different variables!");
            }

            let variable = self.variable;
            let expression1 = self
                .coefficients
                .into_iter()
                .enumerate()
                .map(|(power, coefficient)| {
                    let coefficient: RationalExpression = coefficient.into();
                    coefficient
                        * RationalExpression::from(variable.pow(power as TermVariablePowerType))
                })
                .fold(RationalExpression::default(), Add::add);
            let expression2 = rhs
                .coefficients
                .into_iter()
                .enumerate()
                .map(|(power, coefficient)| {
                    let coefficient: RationalExpression = coefficient.into();
                    coefficient
                        * RationalExpression::from(variable.pow(power as TermVariablePowerType))
                })
                .fold(RationalExpression::default(), Add::add);

            expression1 / expression2
        }
    }

    macro_rules! impl_polynomial_expression_operation_for_multiplier {
        ($trait:ident, $method:ident, $operator:tt) => {
            impl<T> $trait<T> for Polynomial<Expression>
            where T: Into<TermMultiplierType> {
                type Output = Polynomial<Expression>;

                fn $method(self, rhs: T) -> Self::Output {
                    let variable = self.variable;
                    let expression = Expression::from(self) $operator rhs.into();
                    Polynomial::<Expression>::from_expression(expression, variable)
                }
            }
        };
    }

    macro_rules! impl_polynomial_expression_operation {
        ($trait:ident, $method:ident, $operator:tt, $rhs:ty) => {
            impl $trait<$rhs> for Polynomial<Expression> {
                type Output = Polynomial<Expression>;

                fn $method(self, rhs: $rhs) -> Self::Output {
                    let variable = self.variable;
                    let expression = Expression::from(self) $operator rhs;
                    Polynomial::<Expression>::from_expression(expression, variable)
                }
            }
        };
    }

    macro_rules! impl_polynomial_division {
        ($rhs:ty) => {
            impl Div<$rhs> for Polynomial<Expression> {
                type Output = Polynomial<Expression>;

                fn div(self, rhs: $rhs) -> Self::Output {
                    let variable = self.variable;
                    let divisor = Term::from(rhs);
                    if divisor.contains_variable(&variable) {
                        panic!("Divisor cannot contain the polynomial variable. Convert the polynomial to a Expression first!");
                    }
                    let expression = Expression::from(self) / divisor;
                    Polynomial::<Expression>::from_expression(expression, variable)
                }
            }
        };
    }

    impl_polynomial_expression_operation_for_multiplier!(Add, add, +);
    impl_polynomial_expression_operation!(Add, add, +, Variable);
    impl_polynomial_expression_operation!(Add, add, +, Term);
    impl_polynomial_expression_operation!(Add, add, +, Expression);

    impl_polynomial_expression_operation_for_multiplier!(Sub, sub, -);
    impl_polynomial_expression_operation!(Sub, sub, -, Variable);
    impl_polynomial_expression_operation!(Sub, sub, -, Term);
    impl_polynomial_expression_operation!(Sub, sub, -, Expression);

    impl_polynomial_expression_operation_for_multiplier!(Mul, mul, *);
    impl_polynomial_expression_operation!(Mul, mul, *, Variable);
    impl_polynomial_expression_operation!(Mul, mul, *, Term);
    impl_polynomial_expression_operation!(Mul, mul, *, Expression);

    impl_polynomial_expression_operation_for_multiplier!(Div, div, /);
    impl_polynomial_division!(Variable);
    impl_polynomial_division!(Term);
    // impl_polynomial_division!(Expression);

    #[cfg(test)]
    mod binary_operations_tests {
        use super::*;

        #[test]
        fn generic_polynomial_coefficient_operations() {
            let x = Variable::new('x', None);
            let left = Polynomial::<RationalExpression>::new(x, vec![1, 1]);
            let right = Polynomial::<RationalExpression>::new(x, vec![2, 1]);

            assert_eq!(
                (left.clone() + right.clone()).coefficients,
                vec![3.into(), 2.into()]
            );
            assert_eq!((right.clone() - left.clone()).coefficients, vec![1.into()]);
            assert_eq!(
                (left.clone() * right.clone()).coefficients,
                vec![2.into(), 3.into(), 1.into()]
            );
            assert_eq!((-left.clone()).coefficients, vec![(-1).into(), (-1).into()]);
            assert_eq!(
                vec![left.clone(), right.clone()]
                    .into_iter()
                    .sum::<Polynomial<RationalExpression>>()
                    .coefficients,
                vec![3.into(), 2.into()]
            );
            assert_eq!(
                vec![left, right]
                    .into_iter()
                    .product::<Polynomial<RationalExpression>>()
                    .coefficients,
                vec![2.into(), 3.into(), 1.into()]
            );

            let mut assigned = Polynomial::<RationalExpression>::new(x, vec![1, 1]);
            assigned += Polynomial::<RationalExpression>::new(x, vec![2, 1]);
            assigned -= Polynomial::<RationalExpression>::new(x, vec![1, 1]);
            assigned *= Polynomial::<RationalExpression>::new(x, vec![1, 1]);
            assert_eq!(assigned.coefficients, vec![2.into(), 3.into(), 1.into()]);

            let _quotient = Polynomial::<RationalExpression>::new(x, vec![1, 1])
                / Polynomial::<RationalExpression>::new(x, vec![2, 1]);
        }

        #[test]
        fn test_all_arithmetic_combinations() {
            let x = Variable::new('x', None);
            let y = Variable::new('y', None);
            let scalar = TermMultiplierType::new(2, 1);
            let term = Term::from(y).pow(2) * Term::from(x) * scalar;
            let expression = Expression::from(term.clone()) + y;

            let polynomial = Polynomial::new(x, vec![1, 1]);

            let _ = -polynomial.clone();
            let _ = polynomial.clone() + scalar;
            let _ = polynomial.clone() + y;
            let _ = polynomial.clone() + term.clone();
            let _ = polynomial.clone() + expression.clone();
            let _ = polynomial.clone() - scalar;
            let _ = polynomial.clone() - y;
            let _ = polynomial.clone() - term.clone();
            let _ = polynomial.clone() - expression.clone();
            let _ = polynomial.clone() * scalar;
            let _ = polynomial.clone() * y;
            let _ = polynomial.clone() * term.clone();
            let _ = polynomial.clone() * expression.clone();
            let _ = polynomial.clone() / scalar;
            let _ = polynomial.clone() / y;
            let _ = polynomial.clone() / Term::from(y);
            // let _ = polynomial.clone() / (y + 1);
            let _ = polynomial.clone() + polynomial.clone();
            let _ = polynomial.clone() - polynomial.clone();
            let _ = polynomial.clone() * polynomial.clone();
            let _ = polynomial.clone() / polynomial.clone();
        }

        #[test]
        #[should_panic(
            expected = "Divisor cannot contain the polynomial variable. Convert the polynomial to a Expression first!"
        )]
        fn test_polynomial_division_by_own_variable_panics() {
            let x = Variable::new('x', None);
            let polynomial = Polynomial::new(x, vec![1, 1]);

            let _ = polynomial / x;
        }
    }
}

/// Cumulative operations for symbolic terms and expressions.
pub mod cumulative_operations {
    use std::{
        iter::{Product, Sum},
        ops::{Add, Mul},
    };

    use crate::functions::polynomial::{Polynomial, PolynomialCoefficient};

    impl<T> Sum<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + Add<Output = T>,
    {
        /// Adds polynomials defined over the same variable.
        ///
        /// # Panics
        ///
        /// Panics if the iterator is empty or contains polynomials defined over
        /// different variables.
        ///
        /// ```
        /// use tiny_symbolic::*;
        /// use tiny_symbolic::functions::polynomial::Polynomial;
        /// use std::iter::Sum;
        ///
        /// let x = Variable::new('x', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(x, vec![2, 0, 1]),
        /// ];
        ///
        /// assert_eq!(
        ///     polynomials.into_iter().sum::<Polynomial<Expression>>(),
        ///     Polynomial::<Expression>::new(x, vec![3, 1, 1]),
        /// );
        /// ```
        ///
        /// ```should_panic
        /// use tiny_symbolic::*;
        /// use tiny_symbolic::functions::polynomial::Polynomial;
        /// use std::iter::Sum;
        ///
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(y, vec![2, 1]),
        /// ];
        ///
        /// let _: Polynomial<Expression> = polynomials.into_iter().sum();
        /// ```
        fn sum<I: Iterator<Item = Polynomial<T>>>(mut iter: I) -> Self {
            let first = iter
                .next()
                .expect("Cannot sum an empty iterator of polynomials!");
            iter.fold(first, |acc, polynomial| {
                if acc.variable != polynomial.variable {
                    panic!("Cannot sum polynomials with different variables!");
                }
                acc + polynomial
            })
        }
    }

    impl<T> Product<Polynomial<T>> for Polynomial<T>
    where
        T: PolynomialCoefficient + Clone + Add<Output = T> + Mul<Output = T>,
    {
        /// Multiplies polynomials defined over the same variable.
        ///
        /// # Panics
        ///
        /// Panics if the iterator is empty or contains polynomials defined over
        /// different variables.
        ///
        /// ```
        /// use tiny_symbolic::*;
        /// use tiny_symbolic::functions::polynomial::Polynomial;
        /// use std::iter::Product;
        ///
        /// let x = Variable::new('x', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(x, vec![2, 0, 1]),
        /// ];
        ///
        /// assert_eq!(
        ///     polynomials.into_iter().product::<Polynomial<Expression>>(),
        ///     Polynomial::<Expression>::new(x, vec![2, 2, 1, 1]),
        /// );
        /// ```
        ///
        /// ```should_panic
        /// use tiny_symbolic::*;
        /// use tiny_symbolic::functions::polynomial::Polynomial;
        /// use std::iter::Product;
        ///
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(y, vec![2, 1]),
        /// ];
        ///
        /// let _: Polynomial<Expression> = polynomials.into_iter().product();
        /// ```
        fn product<I: Iterator<Item = Polynomial<T>>>(mut iter: I) -> Self {
            let first = iter
                .next()
                .expect("Cannot multiply an empty iterator of polynomials!");
            iter.fold(first, |acc, polynomial| {
                if acc.variable != polynomial.variable {
                    panic!("Cannot multiply polynomials with different variables!");
                }
                acc * polynomial
            })
        }
    }
}
