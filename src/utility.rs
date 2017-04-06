#![allow(non_camel_case_types)]

use std::fmt::Display;

use rand;
use rand::Rng;
use rand::distributions::range::SampleRange;

use num::Integer;

pub type int = i32;
pub type uint = u32;

// MATH FUNCTIONS

/// Return the absolute difference between a and b
/// Note that something like (b-a).abs() doesn't work for unsigned types
pub fn diff<T>(a: T, b: T) -> T
    where T: Integer
{
    if b >= a { b - a } else { a - b }
}

/// Return true if n is between a and b
pub fn between<T>(n: T, a: T, b: T) -> bool
    where T: Integer
{
    if b >= a {
        n >= a && n <= b
    } else {
        n >= b && n <= a
    }
}

/// Return true if a and b are within n units of each other
pub fn in_range<T>(a: T, b: T, n: T) -> bool
    where T: Integer
{
    diff(a, b) <= n
}

/// Return true if a and b are within one unit of each other
pub fn in_one<T>(a: T, b: T) -> bool
    where T: Integer
{
    in_range(a, b, T::one())
}

// RANDOM FUNCTIONS

/// Return a random usize in the range [x..y] inclusive
pub fn rand_range<T>(x: T, y: T) -> T
    where T: Integer + SampleRange
{
    if y > x {
        rand::thread_rng().gen_range(x, y + T::one())
    } else {
        rand::thread_rng().gen_range(y, x + T::one())
    }
}

/// Return true with x in y chance
pub fn dice<T>(x: T, y: T) -> bool
    where T: Integer + SampleRange + Display
{
    debug_assert!(x <= y, format!("Assert failed: dice({}, {})", x, y));
    rand_range(T::one(), y) <= x
}

// UNIT TESTS

#[cfg(test)]
mod tests {
    use utility::*;

    #[test]
    fn test_diff() {
        assert_eq!(diff(1, 2), 1);
        assert_eq!(diff(4, 0), 4);
    }

    #[test]
    fn test_between() {
        assert!(between(1, 0, 1));
        assert!(between(0, 0, 1));
        assert!(between(0, 1, 0));
        assert!(!between(2, 0, 1));
    }

    #[test]
    fn test_in_range() {
        assert!(in_range(0, 1, 1));
        assert!(in_range(1, 0, 2));
        assert!(!in_range(0, 2, 1));
    }

    #[test]
    fn test_rand_range() {
        for _ in 1..100 {
            let a = rand_range(0, 100);
            let b = rand_range(0, 100);
            assert!(between(rand_range(a, b), a, b));
        }
        assert_eq!(rand_range(0, 0), 0);
    }

    #[test]
    fn test_dice() {
        for _ in 1..100 {
            assert!(!dice(0, rand_range(1, 100)));
        }
    }
}
