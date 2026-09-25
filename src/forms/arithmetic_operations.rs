mod assignment_operations {
    use crate::*;
    use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

    impl<T> MulAssign<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        fn mul_assign(&mut self, rhs: T) {
            let rhs_value = rhs.into();
            if rhs_value == 0.into() {
                self.variables.clear();
                self.multiplier = 0.into();
                return;
            }
            self.multiplier *= rhs_value;
            if self.multiplier() == 0.into() {
                self.variables.clear();
            }
        }
    }

    impl MulAssign<Variable> for Term {
        fn mul_assign(&mut self, rhs: Variable) {
            *self *= Term::from(rhs);
        }
    }

    impl MulAssign for Term {
        fn mul_assign(&mut self, rhs: Self) {
            self.multiplier *= rhs.multiplier();
            if self.multiplier() == 0.into() {
                self.variables.clear();
                return;
            }
            for (variable, power) in rhs.variables {
                if let Some(current_power) = self.variables.get_mut(&variable) {
                    *current_power += power;
                    if *current_power == 0 {
                        self.variables.remove(&variable);
                    }
                } else {
                    self.variables.insert(variable, power);
                }
            }
        }
    }

    impl<T> DivAssign<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        fn div_assign(&mut self, rhs: T) {
            let rhs_value = rhs.into();
            if rhs_value == 0.into() {
                panic!("Division by 0!");
            }
            self.multiplier /= rhs_value;
        }
    }

    impl DivAssign<Variable> for Term {
        fn div_assign(&mut self, rhs: Variable) {
            *self /= Term::from(rhs);
        }
    }

    impl DivAssign for Term {
        fn div_assign(&mut self, rhs: Self) {
            if rhs.multiplier() == 0.into() {
                panic!("Division by 0!");
            }
            self.multiplier /= rhs.multiplier();
            for (variable, power) in rhs.variables {
                if let Some(current_power) = self.variables.get_mut(&variable) {
                    *current_power -= power;
                    if *current_power == 0 {
                        self.variables.remove(&variable);
                    }
                } else {
                    self.variables.insert(variable, -power);
                }
            }
        }
    }

    impl<T> AddAssign<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        fn add_assign(&mut self, rhs: T) {
            *self = self.clone() + rhs
        }
    }

    impl AddAssign<Variable> for Expression {
        fn add_assign(&mut self, rhs: Variable) {
            *self = self.clone() + rhs
        }
    }

    impl AddAssign<Term> for Expression {
        fn add_assign(&mut self, rhs: Term) {
            if let Some(multiplier) = self.terms.get_mut(rhs.variables()) {
                *multiplier += rhs.multiplier();
                if *multiplier == 0.into() {
                    self.terms.remove(rhs.variables());
                }
            } else {
                let (variables, multiplier) = rhs.dissolve();
                if multiplier != 0.into() {
                    self.terms.insert(variables, multiplier);
                }
            }
        }
    }

    impl AddAssign for Expression {
        fn add_assign(&mut self, rhs: Self) {
            for (variables, multiplier) in rhs.terms {
                *self += Term::from_btreemap(variables, multiplier);
            }
        }
    }

    impl<T> SubAssign<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self = self.clone() - rhs
        }
    }

    impl SubAssign<Variable> for Expression {
        fn sub_assign(&mut self, rhs: Variable) {
            *self = self.clone() - rhs
        }
    }

    impl SubAssign<Term> for Expression {
        fn sub_assign(&mut self, rhs: Term) {
            if let Some(multiplier) = self.terms.get_mut(rhs.variables()) {
                *multiplier -= rhs.multiplier();
                if *multiplier == 0.into() {
                    self.terms.remove(rhs.variables());
                }
            } else {
                let (variables, multiplier) = rhs.dissolve();
                if multiplier != 0.into() {
                    self.terms.insert(variables, -multiplier);
                }
            }
        }
    }

    impl SubAssign for Expression {
        fn sub_assign(&mut self, rhs: Self) {
            for (variables, multiplier) in rhs.terms {
                *self += -Term::from_btreemap(variables, multiplier);
            }
        }
    }

    impl<T> MulAssign<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = self.clone() * rhs
        }
    }

    impl MulAssign<Variable> for Expression {
        fn mul_assign(&mut self, rhs: Variable) {
            *self = self.clone() * rhs
        }
    }

    impl MulAssign<Term> for Expression {
        fn mul_assign(&mut self, rhs: Term) {
            *self = self.clone() * rhs
        }
    }

    impl MulAssign for Expression {
        fn mul_assign(&mut self, rhs: Expression) {
            *self = self.clone() * rhs;
        }
    }

    impl<T> DivAssign<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = self.clone() / rhs
        }
    }

    impl DivAssign<Variable> for Expression {
        fn div_assign(&mut self, rhs: Variable) {
            *self = self.clone() / rhs
        }
    }

    impl DivAssign<Term> for Expression {
        fn div_assign(&mut self, rhs: Term) {
            *self = self.clone() / rhs
        }
    }

    impl AddAssign for RationalExpression {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs;
        }
    }

    impl<T> AddAssign<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        fn add_assign(&mut self, rhs: T) {
            *self = self.clone() + rhs;
        }
    }

    impl SubAssign for RationalExpression {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs;
        }
    }

    impl<T> SubAssign<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self = self.clone() - rhs;
        }
    }

    impl MulAssign for RationalExpression {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs;
        }
    }

    impl<T> MulAssign<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = self.clone() * rhs;
        }
    }

    impl DivAssign for RationalExpression {
        fn div_assign(&mut self, rhs: Self) {
            *self = self.clone() / rhs;
        }
    }

    impl<T> DivAssign<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        fn div_assign(&mut self, rhs: T) {
            *self = self.clone() / rhs;
        }
    }

    impl AddAssign for Polynomial {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs;
        }
    }

    impl SubAssign for Polynomial {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs;
        }
    }

    impl MulAssign for Polynomial {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs;
        }
    }

    #[cfg(test)]
    mod assignment_operations_tests {
        use super::*;

        #[test]
        fn test_all_arithmetic_combinations() {
            let x = Variable::new('x', None);
            let y = Variable::new('y', None);
            let scalar = TermMultiplierType::new(2, 1);
            let term = Term::from(x) * scalar;
            let expression = Expression::from(term.clone()) + y;
            let rational = RationalExpression::from(expression.clone());
            let polynomial = Polynomial::new(x, vec![1, 1]);

            let mut term_assign = term.clone();
            term_assign *= scalar;
            term_assign *= x;
            term_assign *= term.clone();
            term_assign /= scalar;
            term_assign /= x;
            term_assign /= term.clone();

            let mut expression_assign = expression.clone();
            expression_assign += scalar;
            expression_assign += x;
            expression_assign += term.clone();
            expression_assign += expression.clone();
            expression_assign *= scalar;
            expression_assign *= x;
            expression_assign *= term.clone();
            expression_assign *= expression.clone();
            expression_assign -= scalar;
            expression_assign -= x;
            expression_assign -= term.clone();
            expression_assign -= expression.clone();
            expression_assign /= scalar;
            expression_assign /= x;
            expression_assign /= term.clone();

            let mut rational_assign = rational.clone();
            rational_assign += rational.clone();
            rational_assign -= rational.clone();
            rational_assign *= rational.clone();
            rational_assign /= rational;

            rational_assign += scalar;
            rational_assign += x;
            rational_assign += term.clone();
            rational_assign += expression.clone();
            rational_assign -= scalar;
            rational_assign -= x;
            rational_assign -= term.clone();
            rational_assign -= expression.clone();
            rational_assign *= scalar;
            rational_assign *= x;
            rational_assign *= term.clone();
            rational_assign *= expression.clone();
            rational_assign /= scalar;
            rational_assign /= x;
            rational_assign /= term;
            rational_assign /= expression;

            let mut polynomial_assign = polynomial.clone();
            polynomial_assign += polynomial.clone();
            polynomial_assign -= polynomial.clone();
            polynomial_assign *= polynomial;
        }
    }
}

mod binary_operations {
    use crate::{impl_commutative_op, *};
    use std::ops::{Add, Div, Mul, Neg, Sub};

    impl Neg for Variable {
        type Output = Term;

        fn neg(self) -> Self::Output {
            -Term::from(self)
        }
    }

    impl<T> Add<T> for Variable
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn add(self, rhs: T) -> Self::Output {
            Term::from(self) + Term::from(rhs)
        }
    }

    impl Add for Variable {
        type Output = Expression;

        fn add(self, rhs: Self) -> Self::Output {
            Term::from(self) + Term::from(rhs)
        }
    }

    impl_commutative_op!(Add::add, +, Variable, Term, Expression);

    impl Add<Expression> for Variable {
        type Output = Expression;

        fn add(self, rhs: Expression) -> Self::Output {
            Term::from(self) + rhs
        }
    }

    impl_commutative_op!(Add::add, +, Variable, RationalExpression, RationalExpression);

    impl<T> Sub<T> for Variable
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn sub(self, rhs: T) -> Self::Output {
            Term::from(self) - Term::from(rhs)
        }
    }

    impl Sub for Variable {
        type Output = Expression;

        fn sub(self, rhs: Self) -> Self::Output {
            Term::from(self) - Term::from(rhs)
        }
    }

    impl Sub<Term> for Variable {
        type Output = Expression;

        fn sub(self, rhs: Term) -> Self::Output {
            Term::from(self) - rhs
        }
    }

    impl Sub<Expression> for Variable {
        type Output = Expression;

        fn sub(self, rhs: Expression) -> Self::Output {
            Term::from(self) - rhs
        }
    }

    impl Sub<RationalExpression> for Variable {
        type Output = RationalExpression;

        fn sub(self, rhs: RationalExpression) -> Self::Output {
            -(rhs - self)
        }
    }

    impl<T> Mul<T> for Variable
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Term;

        fn mul(self, rhs: T) -> Self::Output {
            Term::from(self) * Term::from(rhs)
        }
    }

    impl Mul for Variable {
        type Output = Term;

        fn mul(self, rhs: Self) -> Self::Output {
            Term::from(self) * Term::from(rhs)
        }
    }

    impl_commutative_op!(Mul::mul, *, Variable, Term, Term);

    impl_commutative_op!(Mul::mul, *, Variable, Expression, Expression);

    impl_commutative_op!(Mul::mul, *, Variable, RationalExpression, RationalExpression);

    impl<T> Div<T> for Variable
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Term;

        fn div(self, rhs: T) -> Self::Output {
            Term::from(self) / Term::from(rhs)
        }
    }

    impl Div for Variable {
        type Output = Term;

        fn div(self, rhs: Self) -> Self::Output {
            Term::from(self) / Term::from(rhs)
        }
    }

    impl Div<Term> for Variable {
        type Output = Term;

        fn div(self, rhs: Term) -> Self::Output {
            Term::from(self) / rhs
        }
    }

    impl Div<Expression> for Variable {
        type Output = RationalExpression;

        fn div(self, rhs: Expression) -> Self::Output {
            RationalExpression::from(self) / RationalExpression::from(rhs)
        }
    }

    impl Div<RationalExpression> for Variable {
        type Output = RationalExpression;

        fn div(self, rhs: RationalExpression) -> Self::Output {
            (rhs / self).inverse()
        }
    }

    impl Neg for Term {
        type Output = Term;

        fn neg(mut self) -> Self::Output {
            self.multiplier *= -1;
            self
        }
    }

    impl<T> Add<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn add(self, rhs: T) -> Self::Output {
            Expression::from(self) + Expression::from(rhs)
        }
    }

    impl Add<Variable> for Term {
        type Output = Expression;

        fn add(self, rhs: Variable) -> Self::Output {
            Expression::from(self) + Expression::from(rhs)
        }
    }

    impl Add for Term {
        type Output = Expression;

        fn add(self, rhs: Self) -> Self::Output {
            Expression::from(self) + Expression::from(rhs)
        }
    }

    impl_commutative_op!(Add::add, +, Term, Expression, Expression);

    impl_commutative_op!(Add::add, +, Term, RationalExpression, RationalExpression);

    impl<T> Sub<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn sub(self, rhs: T) -> Self::Output {
            Expression::from(self) - Expression::from(rhs)
        }
    }

    impl Sub<Variable> for Term {
        type Output = Expression;

        fn sub(self, rhs: Variable) -> Self::Output {
            Expression::from(self) - Expression::from(rhs)
        }
    }

    impl Sub for Term {
        type Output = Expression;

        fn sub(self, rhs: Self) -> Self::Output {
            Expression::from(self) - Expression::from(rhs)
        }
    }

    impl Sub<Expression> for Term {
        type Output = Expression;

        fn sub(self, rhs: Expression) -> Self::Output {
            -(rhs - self)
        }
    }

    impl Sub<RationalExpression> for Term {
        type Output = RationalExpression;

        fn sub(self, rhs: RationalExpression) -> Self::Output {
            -(rhs - self)
        }
    }

    impl<T> Mul<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Term;

        fn mul(mut self, rhs: T) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl Mul<Variable> for Term {
        type Output = Term;

        fn mul(mut self, rhs: Variable) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl Mul for Term {
        type Output = Term;

        fn mul(mut self, rhs: Self) -> Self::Output {
            self *= rhs;
            self
        }
    }

    impl_commutative_op!(Mul::mul, *, Term, Expression, Expression);

    impl_commutative_op!(Mul::mul, *, Term, RationalExpression, RationalExpression);

    impl<T> Div<T> for Term
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Term;

        fn div(mut self, rhs: T) -> Self::Output {
            self /= rhs;
            self
        }
    }

    impl Div<Variable> for Term {
        type Output = Term;

        fn div(mut self, rhs: Variable) -> Self::Output {
            self /= rhs;
            self
        }
    }

    impl Div for Term {
        type Output = Term;

        fn div(mut self, rhs: Self) -> Self::Output {
            self /= rhs;
            self
        }
    }

    impl Div<Expression> for Term {
        type Output = RationalExpression;

        fn div(self, rhs: Expression) -> Self::Output {
            RationalExpression::new(self.into(), rhs)
        }
    }

    impl Div<RationalExpression> for Term {
        type Output = RationalExpression;

        fn div(self, rhs: RationalExpression) -> Self::Output {
            (rhs / self).inverse()
        }
    }

    impl Neg for Expression {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            for multiplier in self.terms.values_mut() {
                *multiplier *= -1;
            }
            self
        }
    }

    impl<T> Add<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn add(self, rhs: T) -> Self::Output {
            self + Term::from(rhs)
        }
    }

    impl Add<Variable> for Expression {
        type Output = Expression;

        fn add(self, rhs: Variable) -> Self::Output {
            self + Term::from(rhs)
        }
    }

    impl Add<Term> for Expression {
        type Output = Expression;

        fn add(mut self, rhs: Term) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl Add for Expression {
        type Output = Expression;

        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl_commutative_op!(Add::add, +, Expression, RationalExpression, RationalExpression);

    impl<T> Sub<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn sub(self, rhs: T) -> Self::Output {
            self - Term::from(rhs)
        }
    }

    impl Sub<Variable> for Expression {
        type Output = Expression;

        fn sub(self, rhs: Variable) -> Self::Output {
            self - Term::from(rhs)
        }
    }

    impl Sub<Term> for Expression {
        type Output = Expression;

        fn sub(mut self, rhs: Term) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl Sub for Expression {
        type Output = Expression;

        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }

    impl Sub<RationalExpression> for Expression {
        type Output = RationalExpression;

        fn sub(self, rhs: RationalExpression) -> Self::Output {
            -(rhs - self)
        }
    }

    impl<T> Mul<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn mul(self, rhs: T) -> Self::Output {
            self * Term::from(rhs)
        }
    }

    impl Mul<Variable> for Expression {
        type Output = Expression;

        fn mul(self, rhs: Variable) -> Self::Output {
            self * Term::from(rhs)
        }
    }

    impl Mul<Term> for Expression {
        type Output = Expression;

        fn mul(self, rhs: Term) -> Self::Output {
            let mut expression = Expression::default();

            for (variables, multiplier) in self.terms {
                let term = Term::from_btreemap(variables, multiplier);
                expression += term * rhs.clone();
            }

            expression
        }
    }

    impl Mul for Expression {
        type Output = Expression;

        fn mul(self, rhs: Expression) -> Self::Output {
            let mut expression = Expression::default();

            for (variables, multiplier) in self.terms {
                let term = Term::from_btreemap(variables, multiplier);
                expression += rhs.clone() * term;
            }

            expression
        }
    }

    impl_commutative_op!(Mul::mul, *, Expression, RationalExpression, RationalExpression);

    impl<T> Div<T> for Expression
    where
        T: Into<TermMultiplierType>,
    {
        type Output = Expression;

        fn div(self, rhs: T) -> Self::Output {
            self / Term::from(rhs)
        }
    }

    impl Div<Variable> for Expression {
        type Output = Expression;

        fn div(self, rhs: Variable) -> Self::Output {
            self / Term::from(rhs)
        }
    }

    impl Div<Term> for Expression {
        type Output = Expression;

        fn div(self, rhs: Term) -> Self::Output {
            let mut expression = Expression::default();

            for (variables, multiplier) in self.terms {
                let term = Term::from_btreemap(variables, multiplier);
                expression += term / rhs.clone();
            }

            expression
        }
    }

    impl Div<Expression> for Expression {
        type Output = RationalExpression;

        fn div(self, rhs: Expression) -> Self::Output {
            RationalExpression::from(self) / RationalExpression::from(rhs)
        }
    }

    impl Div<RationalExpression> for Expression {
        type Output = RationalExpression;

        fn div(self, rhs: RationalExpression) -> Self::Output {
            (rhs / self).inverse()
        }
    }

    impl Neg for RationalExpression {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.numerator, self.denominator)
        }
    }

    impl<T> Add<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        type Output = Self;

        fn add(self, rhs: T) -> Self::Output {
            let rhs = RationalExpression::from(rhs.into());
            self + rhs
        }
    }

    impl Add for RationalExpression {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(
                self.numerator * rhs.denominator.clone() + rhs.numerator * self.denominator.clone(),
                self.denominator * rhs.denominator,
            )
        }
    }

    impl<T> Sub<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        type Output = Self;

        fn sub(self, rhs: T) -> Self::Output {
            let rhs = RationalExpression::from(rhs.into());
            self - rhs
        }
    }

    impl Sub for RationalExpression {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(
                self.numerator * rhs.denominator.clone() - rhs.numerator * self.denominator.clone(),
                self.denominator * rhs.denominator,
            )
        }
    }

    impl<T> Mul<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            let rhs = RationalExpression::from(rhs.into());
            self * rhs
        }
    }

    impl Mul for RationalExpression {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                self.numerator * rhs.numerator,
                self.denominator * rhs.denominator,
            )
        }
    }

    impl<T> Div<T> for RationalExpression
    where
        T: Into<Expression>,
    {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            let rhs = RationalExpression::from(rhs.into());
            self / rhs
        }
    }

    impl Div for RationalExpression {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self::new(
                self.numerator * rhs.denominator,
                self.denominator * rhs.numerator,
            )
        }
    }

    impl Neg for Polynomial {
        type Output = Polynomial;

        fn neg(mut self) -> Self::Output {
            self.coefficients = self.coefficients.into_iter().map(|c| -c).collect();
            self
        }
    }

    impl Add<Polynomial> for Polynomial {
        type Output = Polynomial;

        fn add(self, rhs: Polynomial) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot add polynomials with different variables!");
            }

            let max_order = usize::max(self.order, rhs.order);
            let mut new_coefficients = vec![Expression::default(); max_order + 1];

            for (i, coeff) in self.coefficients.into_iter().enumerate() {
                new_coefficients[i] += coeff;
            }

            for (i, coeff) in rhs.coefficients.into_iter().enumerate() {
                new_coefficients[i] += coeff;
            }

            while new_coefficients.last().is_some_and(|c| c.is_zero()) {
                new_coefficients.pop();
            }
            Polynomial::new(self.variable, new_coefficients)
        }
    }

    impl Sub<Polynomial> for Polynomial {
        type Output = Polynomial;

        fn sub(self, rhs: Polynomial) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot subtract polynomials with different variables!");
            }

            let max_order = usize::max(self.order, rhs.order);
            let mut new_coefficients = vec![Expression::default(); max_order + 1];

            for (i, coeff) in self.coefficients.into_iter().enumerate() {
                new_coefficients[i] += coeff;
            }

            for (i, coeff) in rhs.coefficients.into_iter().enumerate() {
                new_coefficients[i] -= coeff;
            }

            while new_coefficients.last().is_some_and(|c| c.is_zero()) {
                new_coefficients.pop();
            }
            Polynomial::new(self.variable, new_coefficients)
        }
    }

    impl Mul<Polynomial> for Polynomial {
        type Output = Polynomial;

        fn mul(self, rhs: Polynomial) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot multiply polynomials with different variables!");
            }

            let variable = self.variable;
            let expression1 = Expression::from(self);
            let expression2 = Expression::from(rhs);
            let product_expression = expression1 * expression2;

            Polynomial::from_expression(product_expression, variable)
        }
    }

    impl Div<Polynomial> for Polynomial {
        type Output = RationalExpression;

        fn div(self, rhs: Polynomial) -> Self::Output {
            if self.variable != rhs.variable {
                panic!("Cannot divide polynomials with different variables!");
            }

            let expression1 = Expression::from(self);
            let expression2 = Expression::from(rhs);

            expression1 / expression2
        }
    }

    macro_rules! impl_polynomial_expression_operation_for_multiplier {
        ($trait:ident, $method:ident, $operator:tt) => {
            impl<T> $trait<T> for Polynomial
            where T: Into<TermMultiplierType> {
                type Output = Polynomial;

                fn $method(self, rhs: T) -> Self::Output {
                    let variable = self.variable;
                    let expression = Expression::from(self) $operator rhs.into();
                    Polynomial::from_expression(expression, variable)
                }
            }
        };
    }

    macro_rules! impl_polynomial_expression_operation {
        ($trait:ident, $method:ident, $operator:tt, $rhs:ty) => {
            impl $trait<$rhs> for Polynomial {
                type Output = Polynomial;

                fn $method(self, rhs: $rhs) -> Self::Output {
                    let variable = self.variable;
                    let expression = Expression::from(self) $operator rhs;
                    Polynomial::from_expression(expression, variable)
                }
            }
        };
    }

    macro_rules! impl_polynomial_division {
        ($rhs:ty) => {
            impl Div<$rhs> for Polynomial {
                type Output = Polynomial;

                fn div(self, rhs: $rhs) -> Self::Output {
                    let variable = self.variable;
                    let divisor = Term::from(rhs);
                    if divisor.contains_variable(&variable) {
                        panic!("Divisor cannot contain the polynomial variable. Convert the polynomial to a Expression first!");
                    }
                    let expression = Expression::from(self) / divisor;
                    Polynomial::from_expression(expression, variable)
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

    macro_rules! implement_traits_on_numeric_types
    {
        ($t:ty) => {
            impl_commutative_op!(Add::add, +, $t, Variable, Expression);

            impl_commutative_op!(Add::add, +, $t, Term, Expression);

            impl_commutative_op!(Add::add, +, $t, Expression, Expression);

            impl_commutative_op!(Add::add, +, $t, RationalExpression, RationalExpression);

            impl Sub<Variable> for $t {
                type Output = Expression;

                fn sub(self, rhs: Variable) -> Self::Output {
                    Term::from(self) - Term::from(rhs)
                }
            }

            impl Sub<Term> for $t {
                type Output = Expression;

                fn sub(self, rhs: Term) -> Self::Output {
                    Term::from(self) - rhs
                }
            }

            impl Sub<Expression> for $t {
                type Output = Expression;

                fn sub(self, rhs: Expression) -> Self::Output {
                    Expression::from(self) - rhs
                }
            }

            impl Sub<RationalExpression> for $t {
                type Output = RationalExpression;

                fn sub(self, rhs: RationalExpression) -> Self::Output {
                    -(rhs - self)
                }
            }

            impl_commutative_op!(Mul::mul, *, $t, Variable, Term);

            impl_commutative_op!(Mul::mul, *, $t, Term, Term);

            impl_commutative_op!(Mul::mul, *, $t, Expression, Expression);

            impl_commutative_op!(Mul::mul, *, $t, RationalExpression, RationalExpression);

            impl Div<Variable> for $t {
                type Output = Term;

                fn div(self, rhs: Variable) -> Self::Output {
                    Term::from(self) / Term::from(rhs)
                }
            }

            impl Div<Term> for $t {
                type Output = Term;

                fn div(self, rhs: Term) -> Self::Output {
                    Term::from(self) / rhs
                }
            }

            impl Div<Expression> for $t {
                type Output = RationalExpression;

                fn div(self, rhs: Expression) -> Self::Output {
                    RationalExpression::from(self) / RationalExpression::from(rhs)
                }
            }

            impl Div<RationalExpression> for $t {
                type Output = RationalExpression;

                fn div(self, rhs: RationalExpression) -> Self::Output {
                    (rhs / self).inverse()
                }
            }
        }
    }

    implement_traits_on_numeric_types!(TermMultiplierType);
    implement_traits_on_numeric_types!(i64);

    #[cfg(test)]
    mod binary_operations_tests {
        use super::*;

        #[test]
        fn test_all_arithmetic_combinations() {
            let x = Variable::new('x', None);
            let y = Variable::new('y', None);
            let scalar = TermMultiplierType::new(2, 1);
            let term = Term::from(y).pow(2) * Term::from(x) * scalar;
            let expression = Expression::from(term.clone()) + y;
            let rational = RationalExpression::new(expression.clone(), 1 + expression.clone());
            let polynomial = Polynomial::new(x, vec![1, 1]);

            let _ = -x;
            let _ = x + scalar;
            let _ = x + y;
            let _ = x + term.clone();
            let _ = x + expression.clone();
            let _ = x + rational.clone();
            let _ = x - scalar;
            let _ = x - y;
            let _ = x - term.clone();
            let _ = x - expression.clone();
            let _ = x - rational.clone();
            let _ = x * scalar;
            let _ = x * y;
            let _ = x * term.clone();
            let _ = x * expression.clone();
            let _ = x * rational.clone();
            let _ = x / scalar;
            let _ = x / y;
            let _ = x / term.clone();
            let _ = x / expression.clone();
            let _ = x / rational.clone();

            let _ = -term.clone();
            let _ = term.clone() + scalar;
            let _ = term.clone() + x;
            let _ = term.clone() + term.clone();
            let _ = term.clone() + expression.clone();
            let _ = term.clone() + rational.clone();
            let _ = term.clone() - scalar;
            let _ = term.clone() - x;
            let _ = term.clone() - term.clone();
            let _ = term.clone() - expression.clone();
            let _ = term.clone() - rational.clone();
            let _ = term.clone() * scalar;
            let _ = term.clone() * x;
            let _ = term.clone() * term.clone();
            let _ = term.clone() * expression.clone();
            let _ = term.clone() * rational.clone();
            let _ = term.clone() / scalar;
            let _ = term.clone() / x;
            let _ = term.clone() / term.clone();
            let _ = term.clone() / expression.clone();
            let _ = term.clone() / rational.clone();

            let _ = -expression.clone();
            let _ = expression.clone() + scalar;
            let _ = expression.clone() + x;
            let _ = expression.clone() + term.clone();
            let _ = expression.clone() + expression.clone();
            let _ = expression.clone() + rational.clone();
            let _ = expression.clone() - scalar;
            let _ = expression.clone() - x;
            let _ = expression.clone() - term.clone();
            let _ = expression.clone() - expression.clone();
            let _ = expression.clone() - rational.clone();
            let _ = expression.clone() * scalar;
            let _ = expression.clone() * x;
            let _ = expression.clone() * term.clone();
            let _ = expression.clone() * expression.clone();
            let _ = expression.clone() * rational.clone();
            let _ = expression.clone() / scalar;
            let _ = expression.clone() / x;
            let _ = expression.clone() / term.clone();
            let _ = expression.clone() / expression.clone();
            let _ = expression.clone() / rational.clone();

            let _ = -rational.clone();
            let _ = rational.clone() + scalar;
            let _ = rational.clone() + x;
            let _ = rational.clone() + term.clone();
            let _ = rational.clone() + expression.clone();
            let _ = rational.clone() + rational.clone();
            let _ = rational.clone() - scalar;
            let _ = rational.clone() - x;
            let _ = rational.clone() - term.clone();
            let _ = rational.clone() - expression.clone();
            let _ = rational.clone() - rational.clone();
            let _ = rational.clone() * scalar;
            let _ = rational.clone() * x;
            let _ = rational.clone() * term.clone();
            let _ = rational.clone() * expression.clone();
            let _ = rational.clone() * rational.clone();
            let _ = rational.clone() / scalar;
            let _ = rational.clone() / x;
            let _ = rational.clone() / term.clone();
            let _ = rational.clone() / expression.clone();
            let _ = rational.clone() / rational.clone();

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

            let _ = -scalar;
            let _ = scalar + scalar;
            let _ = scalar + y;
            let _ = scalar + term.clone();
            let _ = scalar + expression.clone();
            let _ = scalar + rational.clone();
            let _ = scalar - scalar;
            let _ = scalar - y;
            let _ = scalar - term.clone();
            let _ = scalar - expression.clone();
            let _ = scalar - rational.clone();
            let _ = scalar * scalar;
            let _ = scalar * y;
            let _ = scalar * term.clone();
            let _ = scalar * expression.clone();
            let _ = scalar * rational.clone();
            let _ = scalar / scalar;
            let _ = scalar / y;
            let _ = scalar / term.clone();
            let _ = scalar / expression.clone();
            let _ = scalar / rational.clone();

            // let _ = 5 + scalar;
            let _ = 5 + y;
            let _ = 5 + term.clone();
            let _ = 5 + expression.clone();
            let _ = 5 + rational.clone();
            // let _ = 5 - scalar;
            let _ = 5 - y;
            let _ = 5 - term.clone();
            let _ = 5 - expression.clone();
            let _ = 5 - rational.clone();
            // let _ = 5 * scalar;
            let _ = 5 * y;
            let _ = 5 * term.clone();
            let _ = 5 * expression.clone();
            let _ = 5 * rational.clone();
            // let _ = 5 / scalar;
            let _ = 5 / y;
            let _ = 5 / term.clone();
            let _ = 5 / expression.clone();
            let _ = 5 / rational.clone();
        }

        #[test]
        #[should_panic(expected = "Polynomial divisor cannot contain the polynomial variable!")]
        fn test_polynomial_division_by_own_variable_panics() {
            let x = Variable::new('x', None);
            let polynomial = Polynomial::new(x, vec![1, 1]);

            let _ = polynomial / x;
        }
    }
}

mod cumulative_operations {
    use crate::*;
    use std::{
        iter::{Product, Sum},
        ops::Add,
        ops::Mul,
    };

    impl<T> Product<T> for Term
    where
        T: Into<Term>,
    {
        /// Multiplies all the terms.
        /// ```
        /// use tiny_symbolic::*;
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let terms = vec![x.pow(2), 2 * x, y.into()];
        /// assert_eq!(terms.into_iter().product::<Term>(), 2 * x.pow(3) * y);
        /// assert_eq!(std::iter::empty::<Term>().product::<Term>(), Term::default());
        /// ```
        fn product<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.map(|v| v.into()).fold(Self::default(), Mul::mul)
        }
    }

    impl<T> Sum<T> for Expression
    where
        T: Into<Expression>,
    {
        /// Adds all the expressions.
        /// ```
        /// use tiny_symbolic::*;
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let terms = vec![x.pow(2), 2 * x, y.into()];
        /// assert_eq!(terms.into_iter().sum::<Expression>(), x.pow(2) + 2 * x + y);
        /// assert_eq!(std::iter::empty::<Expression>().sum::<Expression>(), Expression::default());
        /// ```
        fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.map(|v| v.into()).fold(Expression::default(), Add::add)
        }
    }

    impl<T> Product<T> for Expression
    where
        T: Into<Expression>,
    {
        /// Multiplies all the expressions.
        /// ```
        /// use tiny_symbolic::*;
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let terms = vec![x.pow(2), 2 * x, y.into()];
        /// assert_eq!(terms.into_iter().product::<Expression>(), (2 * x.pow(3) * y).into());
        /// assert_eq!(std::iter::empty::<Expression>().product::<Expression>(), 1.into());
        /// ```
        fn product<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.map(|v| v.into()).fold(1.into(), Mul::mul)
        }
    }

    impl<T> Sum<T> for RationalExpression
    where
        T: Into<RationalExpression>,
    {
        /// Adds all the rational expressions.
        /// ```
        /// use tiny_symbolic::*;
        /// let x = Variable::new('x', None);
        /// let terms = vec![
        ///     RationalExpression::from(1),
        ///     RationalExpression::new(x.into(), x + 1),
        ///     RationalExpression::new(2.into(), x + 1),
        /// ];
        /// assert_eq!(terms.into_iter().sum::<RationalExpression>(), RationalExpression::new(2 * x + 3, x + 1));
        /// assert_eq!(std::iter::empty::<RationalExpression>().sum::<RationalExpression>(), RationalExpression::default());
        /// ```
        fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.map(|v| v.into())
                .fold(RationalExpression::default(), Add::add)
        }
    }

    impl<T> Product<T> for RationalExpression
    where
        T: Into<RationalExpression>,
    {
        /// Multiplies all the rational expressions.
        /// ```
        /// use tiny_symbolic::*;
        /// let x = Variable::new('x', None);
        /// let terms = vec![
        ///     RationalExpression::new(x.into(), x + 1),
        ///     RationalExpression::new(x + 1, 2.into()),
        ///     RationalExpression::from(3),
        /// ];
        /// assert_eq!(terms.into_iter().product::<RationalExpression>(), RationalExpression::new((3 * x).into(), 2.into()));
        /// assert_eq!(std::iter::empty::<RationalExpression>().product::<RationalExpression>(), 1.into());
        /// ```
        fn product<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.map(|v| v.into()).fold(1.into(), Mul::mul)
        }
    }

    impl Sum<Polynomial> for Polynomial {
        /// Adds polynomials defined over the same variable.
        ///
        /// # Panics
        ///
        /// Panics if the iterator is empty or contains polynomials defined over
        /// different variables.
        ///
        /// ```
        /// use tiny_symbolic::*;
        /// use std::iter::Sum;
        ///
        /// let x = Variable::new('x', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(x, vec![2, 0, 1]),
        /// ];
        ///
        /// assert_eq!(
        ///     polynomials.into_iter().sum::<Polynomial>(),
        ///     Polynomial::new(x, vec![3, 1, 1]),
        /// );
        /// ```
        ///
        /// ```should_panic
        /// use tiny_symbolic::*;
        /// use std::iter::Sum;
        ///
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(y, vec![2, 1]),
        /// ];
        ///
        /// let _: Polynomial = polynomials.into_iter().sum();
        /// ```
        fn sum<I: Iterator<Item = Polynomial>>(mut iter: I) -> Self {
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

    impl Product<Polynomial> for Polynomial {
        /// Multiplies polynomials defined over the same variable.
        ///
        /// # Panics
        ///
        /// Panics if the iterator is empty or contains polynomials defined over
        /// different variables.
        ///
        /// ```
        /// use tiny_symbolic::*;
        /// use std::iter::Product;
        ///
        /// let x = Variable::new('x', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(x, vec![2, 0, 1]),
        /// ];
        ///
        /// assert_eq!(
        ///     polynomials.into_iter().product::<Polynomial>(),
        ///     Polynomial::new(x, vec![2, 2, 1, 1]),
        /// );
        /// ```
        ///
        /// ```should_panic
        /// use tiny_symbolic::*;
        /// use std::iter::Product;
        ///
        /// let x = Variable::new('x', None);
        /// let y = Variable::new('y', None);
        /// let polynomials = vec![
        ///     Polynomial::new(x, vec![1, 1]),
        ///     Polynomial::new(y, vec![2, 1]),
        /// ];
        ///
        /// let _: Polynomial = polynomials.into_iter().product();
        /// ```
        fn product<I: Iterator<Item = Polynomial>>(mut iter: I) -> Self {
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
