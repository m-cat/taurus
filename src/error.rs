//! Error module.

use crate::GameResult;
use std::fmt::Display;

/// Error type used throughout the game.
#[derive(Debug, Fail)]
pub enum GameError {
    #[fail(display = "conversion error: {}. {}", val, msg)]
    ConversionError { val: String, msg: &'static str },
    #[fail(display = "unexpected error: {}", msg)]
    UnexpectedError { msg: &'static str },
}

pub fn err_convert<T, R>(val: T, msg: &'static str) -> GameResult<R>
where
    T: Display,
{
    Err(GameError::ConversionError {
        val: format!("{}", val),
        msg,
    }
    .into())
}

pub fn err_unexpected<R>(msg: &'static str) -> GameResult<R> {
    Err(GameError::UnexpectedError { msg }.into())
}
