/// Implements a simple commutative operator overload for two types.
#[macro_export]
macro_rules! impl_commutative_op {
    ($Trait:ident :: $method:ident, $symbol:tt, $A:ty, $B:ty, $Output:ty) => {
        impl std::ops::$Trait<$B> for $A {
            type Output = $Output;

            fn $method(self, rhs: $B) -> Self::Output {
                rhs $symbol self
            }
        }
    };
}

/// Creates a tuple of variables from a slice of characters.
///
/// This is mainly a convenience helper for examples and tests.
#[macro_export]
macro_rules! create_variables {
    ($names:expr; 2) => {
        (
            tiny_symbolic::forms::variable::Variable::new($names[0], None),
            tiny_symbolic::forms::variable::Variable::new($names[1], None),
        )
    };
    ($names:expr; 3) => {
        (
            tiny_symbolic::forms::variable::Variable::new($names[0], None),
            tiny_symbolic::forms::variable::Variable::new($names[1], None),
            tiny_symbolic::forms::variable::Variable::new($names[2], None),
        )
    };
    ($names:expr; 4) => {
        (
            tiny_symbolic::forms::variable::Variable::new($names[0], None),
            tiny_symbolic::forms::variable::Variable::new($names[1], None),
            tiny_symbolic::forms::variable::Variable::new($names[2], None),
            tiny_symbolic::forms::variable::Variable::new($names[3], None),
        )
    };
    ($names:expr; 5) => {
        (
            tiny_symbolic::forms::variable::Variable::new($names[0], None),
            tiny_symbolic::forms::variable::Variable::new($names[1], None),
            tiny_symbolic::forms::variable::Variable::new($names[2], None),
            tiny_symbolic::forms::variable::Variable::new($names[3], None),
            tiny_symbolic::forms::variable::Variable::new($names[4], None),
        )
    };
}
