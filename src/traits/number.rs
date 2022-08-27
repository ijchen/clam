//! A `Number` is a general numeric type.
//!
//! We calculate distances over collections of `Numbers`.

use std::convert::TryInto;
use std::fmt::Debug;
use std::fmt::Display;
use std::iter::Sum;

use easy_cast::Conv;
use ndarray_npy::ReadableElement;
use ndarray_npy::WritableElement;
use num_traits::Num;
use num_traits::NumCast;

/// Collections of `Numbers` can be used to calculate distances.
pub trait Number:
    Num
    + NumCast
    + Sum
    + Copy
    + Clone
    + PartialOrd
    + Send
    + Sync
    + Debug
    + Display
    + ReadableElement
    + WritableElement
    + Conv<Self>
{
    /// Returns the number of bytes used to store this number
    fn num_bytes() -> u8;

    /// Returns the number as a vec of bytes.
    ///
    /// This must be the inverse of from_bytes
    fn to_bytes(&self) -> Vec<u8>;

    /// Reconstructs the Number from its vec of bytes.
    ///
    /// This must be the inverse of to_bytes.
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Convers thhe number to an f64 for some helpful functions.
    fn as_f64(&self) -> f64;
}

macro_rules! impl_number {
    ($($ty:ty),*) => {
        $(
            impl Number for $ty {
                fn num_bytes() -> u8 {
                    (0 as $ty).to_be_bytes().to_vec().len() as u8
                }

                fn to_bytes(&self) -> Vec<u8> {
                    <$ty>::to_be_bytes(*self).to_vec()
                }

                fn from_bytes(bytes: &[u8]) -> $ty {
                    let (value, _) = bytes.split_at(std::mem::size_of::<$ty>());
                    <$ty>::from_be_bytes(value.try_into().unwrap())
                }

                fn as_f64(&self) -> f64 {
                    f64::conv(*self)
                }
            }
        )*
    }
}

impl_number!(f32, f64, u8, i8, u16, i16, u32, i32, u64, i64);

// Bools are not traditionally a numeric type, but can be treated
// as such if definitions are made. Here, we've defined false is 0,
// true is 1, and any other number does not map to bools or vice
// versa.
// TODO: Number cannot be implemented as-is on bool because of the
// orphan rule. Three trait bounds on Number are unimplemented
// for bool and come from external crate (Num, NumCast, and Sum).
// If it is necessary to get around this, I would suggest creating
// new traits local to this crate that are identical to the three
// external traits (Num, NumCast, and Sum) and using those as trait
// bounds on Number. This would allow us to implement Number for bool,
// since we could implement those custom traits on bool as well. The
// custom traits could have auto trait implementations for types that
// implement the original traits so we'd only have to actually manually
// implement it for bools.
impl Number for bool {
    fn num_bytes() -> u8 {
        1
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![match self { false => 0, true => 1 }]
    }

    fn from_bytes(bytes: &[u8]) -> bool {
        assert!(bytes.len() == 1);

        match bytes[0] {
            0 => false,
            1 => true,
            val => panic!("Cannot convert non-[0, 1] u8 into bool (attempted to convert {val} to bool)")
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            true => 1.0,
            false => 0.0,
        }
    }
}