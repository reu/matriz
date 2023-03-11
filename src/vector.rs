use crate::{Matrix, Scalar};

pub type Vector<T, const DIMENSION: usize> = Matrix<T, DIMENSION, 1>;

impl<T, const D: usize> From<[T; D]> for Vector<T, D> {
    fn from(items: [T; D]) -> Self {
        Self::from(items.map(|item| [item]))
    }
}

impl<T: Scalar, const D: usize> Vector<T, D> {
    pub fn len_squared(&self) -> T {
        self.row_major_iter()
            .map(|&item| item * item)
            .fold(T::zero(), |t, i| t + i)
    }
}

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
impl<const D: usize> Vector<f32, D> {
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.len();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }
}

#[cfg(feature = "std")]
impl<const D: usize> Vector<f64, D> {
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.len();
        if len > 0.0 {
            self / len
        } else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_len_squared() {
        assert_eq!(Vector::from([2, 3]).len_squared(), 2 * 2 + 3 * 3);
        assert_eq!(Vector::from([4, 5, 6]).len_squared(), 4 * 4 + 5 * 5 + 6 * 6);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_normalize_vector() {
        macro_rules! assert_approx_eq {
            ($a:expr, $b:expr) => {{
                let epsilon = f32::EPSILON.into();
                let (a, b) = (&$a, &$b);
                assert!(
                    (*a - *b).abs() < epsilon,
                    "assertion failed: `(left !== right)` (left: `{a:?}`, right: `{b:?}`)",
                );
            }};
        }

        assert_approx_eq!(Vector::from([10.0f32, 6.0]).normalized().len(), 1.0);
        assert_eq!(
            Vector::from([0.0f32, 0.0]).normalized(),
            Vector::from([0.0, 0.0])
        );
        assert_eq!(
            Vector::from([1.0f32, 0.0]).normalized(),
            Vector::from([1.0, 0.0])
        );
        assert_eq!(
            Vector::from([-3.0f32, 4.0]).normalized(),
            Vector::from([-0.6, 0.8])
        );

        assert_approx_eq!(Vector::from([10.0f64, 5.0]).normalized().len(), 1.0);
        assert_eq!(
            Vector::from([0.0f64, 0.0]).normalized(),
            Vector::from([0.0, 0.0])
        );
        assert_eq!(
            Vector::from([1.0f64, 0.0]).normalized(),
            Vector::from([1.0, 0.0])
        );
        assert_eq!(
            Vector::from([-3.0f64, -4.0]).normalized(),
            Vector::from([-0.6, -0.8])
        );
    }
}
