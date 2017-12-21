//! Utility functions.

// Allow dead code for utility functions.
#![allow(dead_code, unused_macros)]

#[macro_use]
pub mod math;

pub mod convert;
pub mod direction;
pub mod file;
pub mod rand;
pub mod rectangle;
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

/// Returns `true` if this is a debug build.
pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

/// Returns `true` if the `TAURUS_VERBOSE` environment variable is set to 1.
pub fn is_verbose() -> bool {
    match ::std::env::var("TAURUS_VERBOSE") {
        Ok(val) => val == "1",
        Err(_) => false,
    }
}

#[macro_export]
macro_rules! time_if_verbose {
    ($expr:expr, $msg:expr) => {
        {
            use $crate::util;

            if util::is_verbose() {
                use std::time::Instant;

                println!("{}", $msg);
                let timer_start = Instant::now();

                let res = $expr;

                let timer_end = Instant::now();
                let duration = timer_end.duration_since(timer_start);
                println!("Finished. Time elapsed: {}s, {}ms",
                         duration.as_secs(), duration.subsec_nanos() as f64 / 1_000_000f64);

                res
            } else {
                $expr
            }
        }
    }
}
