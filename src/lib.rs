#![no_std]
#![doc = include_str!("../README.md")]

mod array;
mod matrix;
mod scalar;
mod vector;

pub use matrix::*;
pub use scalar::*;
pub use vector::*;
