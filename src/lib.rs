#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(warnings)]

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

pub use num::Rational64;
