//! Utility functions.

// Allow dead code for utility functions.
#![allow(dead_code, unused_macros)]

#[macro_use]
pub mod math;

pub mod convert;
pub mod direction;
pub mod file;
pub mod rand;
pub mod string;

macro_rules! map {
    { } => {
        ::std::collections::HashMap::new()
    };
    { $( $key:expr => $value:expr ),+ , } => {
        // Rule with trailing comma.
        map!{ $( $key => $value),+ }
    };
    { $( $key:expr => $value:expr ),* } => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                let _ = _map.insert($key, $value);
            )*

            _map
        }
    }
}

/// Tries evaluating `e` `n` times, returning `Some(s)` the first time `e` evaluates to `Some`.
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
