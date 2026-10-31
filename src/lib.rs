#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(warnings)]

/// Type alias for the power of a term variables within Term
pub type TermVariablePowerType = i64;
/// Type alias for the multiplier of a term within Term
pub type TermMultiplierType = Rational64;

/// Module for algorithms related to symbolic mathematics.
pub mod algos;
/// Module for the fundamental structs for symbolic computation
pub mod forms;
/// Module for the common functions.
pub mod functions;
/// Module for fundamental mathematical operations
pub mod operations;

pub use forms::{
    expression::Expression, rational_expression::RationalExpression, term::Term, variable::Variable,
};

pub use forms::basic_operations::{
    arithmetic_operations::assignment_operations, arithmetic_operations::binary_operations,
    arithmetic_operations::cumulative_operations, substitution_operations,
};

pub use forms::basic_traits::{Form, IsStandardScalar};

pub use num::*;
