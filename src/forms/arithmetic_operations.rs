mod assignment_operations {
    use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

    use crate::*;

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

    impl SubAssign for RationalExpression {
        fn sub_assign(&mut self, rhs: Self) {
            *self = self.clone() - rhs;
        }
    }

    impl MulAssign for RationalExpression {
        fn mul_assign(&mut self, rhs: Self) {
            *self = self.clone() * rhs;
        }
    }

    impl DivAssign for RationalExpression {
        fn div_assign(&mut self, rhs: Self) {
            *self = self.clone() / rhs;
        }
    }
}

mod binary_operations {
    use std::ops::{Add, Div, Mul, Neg, Sub};

    use crate::{impl_commutative_op, *};

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

    impl_commutative_op!(Add::add, +, TermMultiplierType, Variable, Expression);

    impl_commutative_op!(Mul::mul, *, TermMultiplierType, Variable, Term);

    impl Sub<Variable> for TermMultiplierType {
        type Output = Expression;

        fn sub(self, rhs: Variable) -> Self::Output {
            Term::from(self) - Term::from(rhs)
        }
    }

    impl Div<Variable> for TermMultiplierType {
        type Output = Term;

        fn div(self, rhs: Variable) -> Self::Output {
            Term::from(self) / Term::from(rhs)
        }
    }

    impl_commutative_op!(Add::add, +, TermMultiplierType, Term, Expression);

    impl_commutative_op!(Mul::mul, *, TermMultiplierType, Term, Term);

    impl Sub<Term> for TermMultiplierType {
        type Output = Expression;

        fn sub(self, rhs: Term) -> Self::Output {
            Term::from(self) - rhs
        }
    }

    impl Div<Term> for TermMultiplierType {
        type Output = Term;

        fn div(self, rhs: Term) -> Self::Output {
            Term::from(self) / rhs
        }
    }

    impl_commutative_op!(Add::add, +, TermMultiplierType, Expression, Expression);

    impl_commutative_op!(Mul::mul, *, TermMultiplierType, Expression, Expression);

    impl Sub<Expression> for TermMultiplierType {
        type Output = Expression;

        fn sub(self, rhs: Expression) -> Self::Output {
            Expression::from(self) - rhs
        }
    }

    impl_commutative_op!(Add::add, +, TermMultiplierType, RationalExpression, RationalExpression);

    impl_commutative_op!(Mul::mul, *, TermMultiplierType, RationalExpression, RationalExpression);

    impl_commutative_op!(Add::add, +, TermVariablePowerType, Variable, Expression);

    impl_commutative_op!(Mul::mul, *, TermVariablePowerType, Variable, Term);

    impl_commutative_op!(Add::add, +, TermVariablePowerType, Term, Expression);

    impl_commutative_op!(Mul::mul, *, TermVariablePowerType, Term, Term);

    impl_commutative_op!(Add::add, +, TermVariablePowerType, Expression, Expression);

    impl_commutative_op!(Mul::mul, *, TermVariablePowerType, Expression, Expression);

    impl_commutative_op!(Add::add, +, TermVariablePowerType, RationalExpression, RationalExpression);

    impl_commutative_op!(Mul::mul, *, TermVariablePowerType, RationalExpression, RationalExpression);

    impl Sub<Variable> for TermVariablePowerType {
        type Output = Expression;

        fn sub(self, rhs: Variable) -> Self::Output {
            Term::from(self) - rhs
        }
    }

    impl Div<Variable> for TermVariablePowerType {
        type Output = Term;

        fn div(self, rhs: Variable) -> Self::Output {
            Term::from(self) / rhs
        }
    }

    impl Sub<Term> for TermVariablePowerType {
        type Output = Expression;

        fn sub(self, rhs: Term) -> Self::Output {
            Term::from(self) - rhs
        }
    }

    impl Div<Term> for TermVariablePowerType {
        type Output = Term;

        fn div(self, rhs: Term) -> Self::Output {
            Term::from(self) / rhs
        }
    }

    impl Sub<Expression> for TermVariablePowerType {
        type Output = Expression;

        fn sub(self, rhs: Expression) -> Self::Output {
            Expression::from(self) - rhs
        }
    }
}
