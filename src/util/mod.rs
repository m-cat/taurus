//! Utility functions.

// Allow dead code for utility functions.
#![allow(dead_code)]

/// File IO utility functions.
pub mod file;
/// Math utility functions.
#[macro_use]
pub mod math;
/// Random number utility functions.
pub mod rand;

/// Standard type for signed ints
#[allow(non_camel_case_types)]
pub type int = i32;

/// Standard type for unsigned ints
#[allow(non_camel_case_types)]
pub type uint = u32;

/// Enum for the 8 possible cardinal directions.
#[derive(Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

/// Tries evaluating `e` `n` times, returning `Some(s)` the first time `e` evaluates to `Some`
#[macro_export]
macro_rules! try_some {
    ( $e:expr, $n:expr ) => {
        {
            let mut ret = None;
            for _ in 0..$n {
                if let Some(s) = $e {
                    ret = Some(s);
                    break;
                }
            }
            ret
        }
    }
}
