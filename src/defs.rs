//! Crate for project-wide definitions.

use error::err_convert;
use num::bigint::BigInt;
use num::rational::{BigRational, Ratio};
use num_traits::ToPrimitive;
use GameResult;

/// Rational to store current turn.
pub type GameRatio = Ratio<u32>;

/// Returns maximum GameRatio.
pub fn gameratio_max() -> GameRatio {
    Ratio::new(u32::max_value(), 1)
}

pub fn bigr_to_f32(value: BigRational) -> GameResult<f32> {
    let numer = match value.numer().to_f32() {
        Some(n) => n,
        None => return err_convert(value, "Numerator too big"),
    };
    let denom = match value.denom().to_f32() {
        Some(n) => n,
        None => return err_convert(value, "Denominator too big"),
    };

    Ok(numer / denom)
}

/// Converts a `BigRational` into a `GameRatio`.
pub fn bigr_to_gamer(value: BigRational) -> GameResult<GameRatio> {
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

pub fn big_to_i32(value: BigInt) -> GameResult<i32> {
    match value.to_i32() {
        Some(n) => Ok(n),
        None => err_convert(value, "Value is too large"),
    }
}

pub fn big_to_usize(value: BigInt) -> GameResult<usize> {
    match value.to_usize() {
        Some(n) => Ok(n),
        None => err_convert(value, "Value is too large"),
    }
}
