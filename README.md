# Matriz
Typesafe and simple linear algebra library, with no-std support.

Disclaimer: this library is intendend for educational porpouses.
For production grade implementations just go with [nalgebra](https://docs.rs/nalgebra).

## Usage

Key feature is that matrices dimensions are encoded on the type, and transformations
generate proper typed results.

```rust
use matriz::Matrix;

let m1 = Matrix::from_rows([
    [1, -2, 4],
    [5,  0, 3],
]);

let m2 = Matrix::from_rows([
    [ 1],
    [ 5],
    [-1],
]);

let output = Matrix::from_rows([
    [-13],
    [  2],
]);

assert_eq!(m1 * m2, output);
```
