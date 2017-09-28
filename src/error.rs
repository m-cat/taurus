//! Error module.

use GameResult;
use std::fmt::Display;

/// Error type used throughout the game.
#[derive(Debug, Fail)]
pub enum GameError {
    #[fail(display = "conversion error: {}. {}", val, msg)]
    ConversionError { val: String, msg: &'static str },
}

pub fn err_convert<T, R>(val: T, msg: &'static str) -> GameResult<R>
where
    T: Display,
{
    Err(
        GameError::ConversionError {
            val: format!("{}", val),
            msg,
        }.into(),
    )
}
