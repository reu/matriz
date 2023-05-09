use core::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use crate::{array::ArrayBuilder, Scalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize>([[T; COLS]; ROWS]);

impl<T: Default, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Self::try_from_iter((0..(R * C)).map(|_| T::default())).unwrap()
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(rows: [[T; C]; R]) -> Self {
        Self(rows)
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

impl<T: Scalar, const R: usize, const C: usize> Add<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        self.zip(rhs).map(|(a, b)| a + b)
    }
}

impl<T: Scalar, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Matrix<T, R, C>) -> Self::Output {
        self.zip(rhs).map(|(a, b)| a - b)
    }
}

impl<T: Scalar, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|item| item * rhs)
    }
}

impl<T: Scalar, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn div(self, rhs: T) -> Self::Output {
        self.map(|item| item / rhs)
    }
}

impl<T: Scalar, const R: usize, const C: usize, const X: usize> Mul<Matrix<T, C, X>>
    for Matrix<T, R, C>
{
    type Output = Matrix<T, R, X>;

    fn mul(self, rhs: Matrix<T, C, X>) -> Self::Output {
        let mut res = Matrix::zero();
        for r in 0..R {
            for c in 0..C {
                for x in 0..X {
                    res[(r, x)] = res[(r, x)] + (self[(r, c)] * rhs[(c, x)])
                }
            }
        }
        res
    }
}

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn zero() -> Self {
        Self([[T::zero(); C]; R])
    }

    pub fn splat(item: T) -> Self {
        Self([[item; C]; R])
    }
}

impl<T: Scalar, const N: usize> Matrix<T, N, N> {
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..N {
            m[(i, i)] = T::one();
        }
        m
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromIterError {
    Overflowed,
    Underflowed,
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn from_rows(rows: [[T; C]; R]) -> Self {
        Self(rows)
    }

    pub fn try_from_iter(iter: impl IntoIterator<Item = T>) -> Result<Self, TryFromIterError> {
        let mut iter = iter.into_iter();
        let mut rows = ArrayBuilder::<_, R>::new();

        for _ in 0..R {
            let mut cols = ArrayBuilder::<_, C>::new();

            for _ in 0..C {
                if let Some(item) = iter.next() {
                    cols.push(item).map_err(|_| TryFromIterError::Overflowed)?;
                }
            }

            let cols = cols.build().map_err(|_| TryFromIterError::Underflowed)?;

            rows.push(cols).map_err(|_| TryFromIterError::Overflowed)?;
        }

        rows.build()
            .map(Matrix::from_rows)
            .map_err(|_| TryFromIterError::Underflowed)
    }

    pub fn map<F: FnMut(T) -> U, U>(self, mut f: F) -> Matrix<U, R, C> {
        Matrix::from_rows(self.0.map(move |row| row.map(&mut f)))
    }

    pub fn zip<U>(self, m: Matrix<U, R, C>) -> Matrix<(T, U), R, C> {
        let items = self
            .0
            .into_iter()
            .zip(m.0.into_iter())
            .flat_map(|(ra, rb)| ra.into_iter().zip(rb.into_iter()));

        Matrix::try_from_iter(items).unwrap()
    }

    pub fn row_major_iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.0.iter().flatten()
    }

    pub fn col_major_iter(&self) -> impl Iterator<Item = &T> + '_ {
        (0..C).flat_map(move |c| (0..R).map(move |r| &self.0[r][c]))
    }

    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> + '_ {
        self.0.iter().skip(i).take(1).flatten()
    }

    pub fn iter_col(&self, i: usize) -> impl Iterator<Item = &T> + '_ {
        self.0.iter().filter_map(move |row| row.get(i))
    }
}

impl<T: Clone, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn from_cols(cols: [[T; R]; C]) -> Self {
        Matrix::from_rows(cols).transpose()
    }

    pub fn transpose(self) -> Matrix<T, C, R> {
        Matrix::try_from_iter(self.col_major_iter().cloned()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_default() {
        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [0, 0, 0],
            [0, 0, 0],
        ]);

        assert_eq!(Matrix::default(), expected);
    }

    #[test]
    fn test_matrix_from_cols() {
        #[rustfmt::skip]
        let m = Matrix::from_cols([
            [1, 2, 3],
            [4, 5, 6],
        ]);
        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [1, 4],
            [2, 5],
            [3, 6],
        ]);

        assert_eq!(m, expected);
    }

    #[test]
    fn test_matrix_zero() {
        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [0, 0, 0],
            [0, 0, 0],
        ]);

        assert_eq!(Matrix::zero(), expected);
    }

    #[test]
    fn test_matrix_splat() {
        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [3, 3, 3],
            [3, 3, 3],
        ]);

        assert_eq!(Matrix::splat(3), expected);
    }

    #[test]
    fn test_matrix_identity() {
        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);

        assert_eq!(Matrix::identity(), expected);
    }

    #[test]
    fn test_matrix_try_from_iter() {
        let items = [1, 2, 3, 4, 5, 6];

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        assert_eq!(Matrix::try_from_iter(items), Ok(expected));
    }

    #[test]
    fn test_matrix_addition() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [2,  4,  6],
            [8, 10, 12],
        ]);

        assert_eq!(m + m, expected);
    }

    #[test]
    fn test_matrix_subtraction() {
        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::from_rows([
            [3, 2, 1],
            [6, 5, 4],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [-2, 0, 2],
            [-2, 0, 2],
        ]);

        assert_eq!(m1 - m2, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [2,  4,  6],
            [8, 10, 12],
        ]);

        assert_eq!(m1 * 2, expected);
    }

    #[test]
    fn test_matrix_scalar_division() {
        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [0.5, 1.0, 1.5],
            [2.0, 2.5, 3.0],
        ]);

        assert_eq!(m1 / 2.0, expected);
    }

    #[test]
    fn test_matrix_zip() {
        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::from_rows([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [(1, 1.0), (2, 2.0), (3, 3.0)],
            [(4, 4.0), (5, 5.0), (6, 6.0)],
        ]);

        assert_eq!(m1.zip(m2), expected);
    }

    #[test]
    fn test_matrix_iter_row() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        let mut iter = m.iter_row(0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        let mut iter = m.iter_row(1);
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_matrix_iter_col() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        let mut iter = m.iter_col(0);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);

        let mut iter = m.iter_col(1);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);

        let mut iter = m.iter_col(2);
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_matrix_row_major_iter() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        let mut iter = m.row_major_iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_matrix_col_major_iter() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        let mut iter = m.col_major_iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_matrix_transpose() {
        #[rustfmt::skip]
        let m = Matrix::from_rows([
            [1, 2, 3],
            [4, 5, 6],
        ]);

        #[rustfmt::skip]
        let expected = Matrix::from_rows([
            [1, 4],
            [2, 5],
            [3, 6],
        ]);

        assert_eq!(m.transpose(), expected);
    }

    #[test]
    fn test_matrix_mult() {
        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1, -2, 4],
            [5,  0, 3],
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::from_rows([
            [ 1],
            [ 5],
            [-1],
        ]);

        let m3 = m1 * m2;

        #[rustfmt::skip]
        let expect = Matrix::from_rows([
            [-13],
            [  2],
        ]);

        assert_eq!(m3, expect);

        #[rustfmt::skip]
        let m1 = Matrix::from_rows([
            [1, -2, 4],
            [5,  0, 3],
            [0,  2, 9],
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::from_rows([
            [ 1, 0],
            [ 5, 3],
            [-1, 0],
        ]);

        let m3 = m1 * m2;

        #[rustfmt::skip]
        let expect = Matrix::from_rows([
            [-13, -6],
            [  2,  0],
            [  1,  6],
        ]);

        assert_eq!(m3, expect);
    }
}
