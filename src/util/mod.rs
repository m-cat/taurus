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
    ($e:expr, $n:expr) => {{
        let mut ret = None;
        for _ in 0..$n {
            if let Some(s) = $e {
                ret = Some(s);
                break;
            }
        }
        ret
    }};
}

#[macro_export]
macro_rules! dev_time {
    ($expr:expr, $msg:expr) => {{
        use $crate::util;

        #[cfg(all(feature = "dev", not(test)))]
        {
            use std::time::Instant;

            println!("\n{}", $msg);
            let timer_start = Instant::now();

            let res = $expr;

            let timer_end = Instant::now();
            let duration = timer_end.duration_since(timer_start);
            println!(
                "Finished. Time elapsed: {}s, {}ms\n",
                duration.as_secs(),
                f64::from(duration.subsec_nanos()) / 1_000_000f64
            );

            res
        }
        #[cfg(not(all(feature = "dev", not(test))))]
        {
            $expr
        }
    }};
}
