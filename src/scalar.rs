use core::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub trait Scalar:
    Debug + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_primitive_scalar {
    ($type:ty, zero = $zero:expr, one = $one:expr) => {
        impl Scalar for $type {
            fn zero() -> Self {
                $zero
            }

            fn one() -> Self {
                $one
            }
        }
    };

    ($($type:ty),*) => {
        $(impl_primitive_scalar!($type, zero = 0, one = 1);)*
    };

    ($($type:ty),*; zero = $zero:expr; one = $one:expr) => {
        $(impl_primitive_scalar!($type, zero = $zero, one = $one);)*
    };
}

impl_primitive_scalar!(f32, f64; zero = 0.0; one = 1.0);
impl_primitive_scalar!(u8, u16, u32, u64, u128, usize);
impl_primitive_scalar!(i8, i16, i32, i64, i128, isize);
