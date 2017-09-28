//! Crate for project-wide definitions.

use GameResult;
use error::err_convert;
use num::bigint::BigInt;
use num::rational::{BigRational, Ratio};
use num_traits::ToPrimitive;

/// Default type for signed ints.
#[allow(non_camel_case_types)]
pub type int = i32;

/// Default type for unsigned ints.
#[allow(non_camel_case_types)]
pub type uint = u32;

/// Rational to store current turn.
pub type TurnRatio = Ratio<u32>;

/// Convert a `BigRational` into a `TurnRatio`.
pub fn to_turnratio(value: BigRational) -> GameResult<TurnRatio> {
    let numer = match value.numer().to_u32() {
        Some(n) => n,
        None => return err_convert(value, "Numerator too big"),
    };
    let denom = match value.denom().to_u32() {
        Some(n) => n,
        None => return err_convert(value, "Denominator too big"),
    };

    Ok(Ratio::new(numer, denom))
}

pub fn big_to_u32(value: BigInt) -> GameResult<u32> {
    match value.to_u32() {
        Some(n) => Ok(n),
        None => err_convert(value, "Value is too large"),
    }
}
