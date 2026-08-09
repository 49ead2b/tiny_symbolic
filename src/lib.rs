#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(warnings)]

/// Module for algorithms related to symbolic mathematics.
pub mod algos;
/// Module for the fundamental structs for symbolic computation
pub mod forms;

pub use forms::{expression::Expression, term::Term, variable::Variable};
