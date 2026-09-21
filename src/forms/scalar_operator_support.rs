use num::{Integer, Rational64};

/// Computes the least common multiple of two rational numbers.
pub fn rational_lcm(x: &Rational64, y: &Rational64) -> Rational64 {
    let num = x.numer().lcm(y.numer());
    let den = x.denom().gcd(y.denom());
    Rational64::new(num, den)
}

#[cfg(test)]
mod tests {
    use crate::Variable;

    use super::*;

    #[test]
    fn test_i64_scalar_operator_support_coverage() {
        let x = Variable::new('x', None);
        let y = Variable::new('y', None);
        let expression = x + y;
        let term = x.pow(2);

        let add_variable = 3i64 + x;
        let mul_variable = 2i64 * x;
        let add_term = 3i64 + term.clone();
        let mul_term = 2i64 * term.clone();
        let add_expression = 3i64 + expression.clone();
        let mul_expression = 2i64 * expression.clone();
        let sub_variable = 3i64 - x;
        let div_variable = 3i64 / x;
        let sub_term = 3i64 - term.clone();
        let div_term = 3i64 / term.clone();
        let sub_expression = 3i64 - expression.clone();

        assert_eq!(add_variable.to_string(), "3 + x");
        assert_eq!(mul_variable.to_string(), "2x");
        assert_eq!(add_term.to_string(), "3 + x²");
        assert_eq!(mul_term.to_string(), "2x²");
        assert_eq!(add_expression.to_string(), "3 + x + y");
        assert_eq!(mul_expression.to_string(), "2x + 2y");
        assert_eq!(sub_variable.to_string(), "3 - x");
        assert_eq!(div_variable.to_string(), "3x⁻¹");
        assert_eq!(sub_term.to_string(), "3 - x²");
        assert_eq!(div_term.to_string(), "3x⁻²");
        assert_eq!(sub_expression.to_string(), "3 - x - y");
    }

    #[test]
    fn test_rational_lcm() {
        let a = Rational64::new(2, 7);
        let b = Rational64::new(3, 14);
        let c = Rational64::new(5, 3);
        let lcm = rational_lcm(&rational_lcm(&a, &b), &c);
        assert_eq!(lcm, Rational64::new(30, 1));

        let b = Rational64::new(3, 4);
        let c = Rational64::new(3, 2);
        let lcm = rational_lcm(&b, &c);
        assert_eq!(lcm, Rational64::new(3, 2));
    }
}
