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
/// Module for fundamental mathematical operations
pub mod operations;

pub use forms::{
    expression::Expression, polynomial::Polynomial, rational_expression::RationalExpression,
    term::Term, variable::Variable,
};

pub use num::*;
