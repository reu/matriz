use criterion::{criterion_group, criterion_main, Criterion};
use matriz::Matrix;
use nalgebra::matrix;

fn add(c: &mut Criterion) {
    let mut g = c.benchmark_group("add");

    #[rustfmt::skip]
    let matriz_m1 = Matrix::from_rows([
        [1, 2, 3],
        [4, 5, 6],
    ]);

    #[rustfmt::skip]
    let matriz_m2 = Matrix::from_rows([
        [2,  4,  6],
        [8, 10, 12],
    ]);

    g.bench_function("matriz", |b| b.iter(|| matriz_m1 + matriz_m2));

    let nalgebra_m1 = matrix![
        1, 2, 3;
        4, 5, 6;
    ];

    let nalgebra_m2 = matrix![
        2,  4,  6;
        8, 10, 12;
    ];
    g.bench_function("nalgebra", |b| b.iter(|| nalgebra_m1 + nalgebra_m2));
}

fn mul(c: &mut Criterion) {
    let mut mul = c.benchmark_group("mul");

    #[rustfmt::skip]
    let matriz_m1 = Matrix::from_rows([
        [1, -2, 4],
        [5,  0, 3],
        [0,  2, 9],
    ]);

    #[rustfmt::skip]
    let matriz_m2 = Matrix::from_rows([
        [ 1, 0],
        [ 5, 3],
        [-1, 0],
    ]);

    mul.bench_function("matriz", |b| b.iter(|| matriz_m1 * matriz_m2));

    let nalgebra_m1 = matrix![
        1, -2, 4;
        5,  0, 3;
        0,  2, 9;
    ];

    let nalgebra_m2 = matrix![
         1, 0;
         5, 3;
        -1, 0;
    ];
    mul.bench_function("nalgebra", |b| b.iter(|| nalgebra_m1 * nalgebra_m2));
}

criterion_group!(benches, add, mul);
criterion_main!(benches);
