use crate::*;

/*
// Term Conversion Operations
*/

impl From<Variable> for Term {
    fn from(value: Variable) -> Self {
        let mut term = Self::default();
        term.variables.insert(value, 1);
        term
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

/*
// Expression Conversion Operations
*/

impl<T> From<T> for Expression
where
    T: Into<Term>,
{
    fn from(value: T) -> Self {
        let value = value.into();
        let (variables, multiplier) = value.dissolve();
        let mut expression = Self::default();
        if multiplier != 0.into() {
            expression.terms.insert(variables, multiplier);
        }

        expression
    }
}

impl TryInto<Term> for Expression {
    type Error = String;

    fn try_into(self) -> Result<Term, Self::Error> {
        let mut terms = self.dissolve_into_terms();
        if terms.is_empty() {
            Ok(0.into())
        } else if terms.len() == 1 {
            Ok(terms.pop().unwrap())
        } else {
            Err("Cannot be reduced to a term!".to_string())
        }
    }
}

/*
// RationalExpression Conversion Operations
*/

impl<T> From<T> for RationalExpression
where
    T: Into<Expression>,
{
    fn from(value: T) -> Self {
        Self {
            numerator: value.into(),
            denominator: 1.into(),
        }
    }
}

impl TryInto<Expression> for RationalExpression {
    type Error = String;

    fn try_into(self) -> Result<Expression, Self::Error> {
        let (numerator, denominator) = self.dissolve_into_numerator_and_denominator();
        if denominator.is_term() && denominator != 0.into() {
            let denominator: Term = denominator.try_into().unwrap();
            Ok(numerator / denominator)
        } else {
            Err("Cannot be reduced to a expression!".to_string())
        }
    }
}

impl TryInto<Term> for RationalExpression {
    type Error = String;

    fn try_into(self) -> Result<Term, Self::Error> {
        if let Ok(expression) = TryInto::<Expression>::try_into(self)
            && let Ok(term) = TryInto::<Term>::try_into(expression)
        {
            return Ok(term);
        }

        Err("Cannot be reduced to a term!".to_string())
    }
}
