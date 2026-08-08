# tiny_symbolic

A minimal symbolic computation library for Rust.

## Features

- Exact rational arithmetic
- Basic substitution and polynomial operations
- Elementary symmetric polynomial helpers
- Small, ergonomic operator-overloaded syntax for quick algebraic experiments
- SymPy-style optional string formatting for easier SymPy porting

## Quick start

The crate exposes the core types directly from the crate root:

```rust
use tiny_symbolic::{Expression, Variable};

fn main() {
    let x = Variable::new('x', None);
    let x_1 = Variable::new('x', Some(1));
    let expr: Expression = 2 * x.pow(2) + 3 * x_1 - 5;

    println!("{}", expr);
}
```

This builds a symbolic expression using `Variable` and `Expression` arithmetic.

`Variable::new(name, subscript)` also accepts an optional subscript, so symbolic names such as `x₁`, `p₂`, or `e₃` can be represented naturally in output.

## Building expressions

You can combine variables, powers, and scalars naturally:

```rust
use tiny_symbolic::{Expression, Variable};

fn main() {
    let x = Variable::new('x', None);
    let y = Variable::new('y', None);

    let expr: Expression = (x + y).pow(2) - 4 * x * y;
    println!("{}", expr);
}
```

The expression builder keeps terms in a normalized symbolic form, which makes it easy to inspect and transform algebraic objects.

## Substituting variables

Substitution is one of the most useful operations when working with symbolic formulas:

```rust
use tiny_symbolic::Variable;

fn main() {
    let x = Variable::new('x', None);
    let expr = x.pow(3) + 2 * x.pow(2) - x + 7;

    let substituted = expr.substitute_multiple_variables_with_expressions([(x, 3)]);
    println!("{}", substituted);   // Will output 49

    let a = Variable::new('A', None);
    let b = Variable::new('B', None);
    let c = Variable::new('C', None);
    let w = Variable::new('W', None);   // Suppose W is the cube root of unity

    let expr = (a + b + c) * (a + w * b + w.pow(2) * c) * (a + w.pow(2) * b + w * c);
    let substituted = expr
        .substitute_variable_power_with_expression(w, 3, 1_i64.into())
        .substitute_variable_power_with_expression(w, 2, -1 - w);

    println!("{substituted}");  //Will output -3ABC + A³ + B³ + C³
}
```

By cleverly controlling the order of substitutions, it is possible to make use of constraints like W³ = 1, 1 + W + W² = 0

## Printing for SymPy workflows

For SymPy workflows, the formatting helpers can render terms in SymPy compatible form, which makes it easier to port expressions.

```rust
use num::Rational64;
use tiny_symbolic::{Variable};

fn main() {
    let x = Variable::new('x', None);
    let y = Variable::new('y', None);
    let z = Variable::new('z', None);
    let expression = Rational64::new(4, 3) * x * y.pow(3) + z;

    println!("{}", expression.to_string_sympy());
}
```

This prints a SymPy-friendly form `(4/3)*x*(y**3) + z`.

## Elementary symmetric polynomials

The `ElementarySymmetricPolynomials` helper lets you reason about symmetric expressions in terms of elementary symmetric functions.

```rust
use tiny_symbolic::{ElementarySymmetricPolynomials, Variable};

fn main() {
    let a = Variable::new('A', None);
    let b = Variable::new('B', None);
    let c = Variable::new('C', None);

    let expr = a.pow(2) + b.pow(2) + c.pow(2) + a.pow(2) * b + a.pow(2) * c + b.pow(2) * a + b.pow(2) * c + c.pow(2) * a + c.pow(2) * b; 

    let mut esp = ElementarySymmetricPolynomials::from_variables(&vec![a, b, c]);
    esp.set_ek_substitution_as(1, Variable::new('p', Some(1)).into());
    esp.set_ek_substitution_as(2, Variable::new('p', Some(2)).into());
    esp.set_ek_substitution_as(3, Variable::new('p', Some(3)).into());

    let is_symmetric = esp.is_symmetric(&expr);
    println!("Is symmetric: {}", is_symmetric); // Will output true

    let simplified = esp.simplify_symmetric_expression(expr);
    println!("Simplified: {}", simplified); // Will output p₁p₂ + p₁² - 2p₂ - 3p₃
}
```

This is the intended pattern for turning a symmetric polynomial into a more conventional elementary-symmetric form.

## Crate API

- `tiny_symbolic::Variable`
- `tiny_symbolic::Term`
- `tiny_symbolic::Expression`
- `tiny_symbolic::ElementarySymmetricPolynomials`

## License

`MIT OR Apache-2.0`
