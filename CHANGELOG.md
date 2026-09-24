# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-06

### Added
- Initial release of the library.
- Core data structures(Variable, Term, Expression) and basic helper functions.
- Algorithm for simplifying Elementary Symmetric Polynomials

## [0.2.0] - 2026-08-09

### Changed
- Export ElementarySymmetricPolynomials under algos instead of crate root.

## [0.3.0] - 2026-09-23

### Added
- Add RationalExpression and Polynomial data structure.
- Support all arithmetic combinations of Variable, Term, Expression, RationalExpression

### Fixed
- More checks for undefined operations.

### Changed
- Change type of power of variables in Term to i64

## [0.3.1] - 2026-09-24

### Added
- Sum and Product over Iterators for Term, Expression, RationalExpression and Polynomial