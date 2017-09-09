//! Math utility functions.

#![allow(unknown_lints)]

use num::Integer;
use std::cmp::{max, min};

/// Returns the max of all given elements.
#[macro_export]
macro_rules! max {
    ( $x:expr, $( $e:expr ),+ ) => {
        {
            let mut max = $x;
            $(
                if $e > max {
                    max = $e;
                }
            )+
            max
        }
    }
}

/// Returns the min of all given elements.
#[macro_export]
macro_rules! min {
    ( $x:expr, $( $e:expr ),+ ) => {
        {
            let mut min = $x;
            $(
                if $e < min {
                    min = $e;
                }
            )+
            min
        }
    }
}

/// Returns a tuple (min, max) of `a` and `b`.
pub fn min_max<T>(a: T, b: T) -> (T, T)
where
    T: Integer + Copy,
{
    (min(a, b), max(a, b))
}

/// Returns true if two inclusive ranges `[a1, a2]` and `[b1, b2]` overlap.
pub fn overlaps<T>(a1: T, a2: T, b1: T, b2: T) -> bool
where
    T: Integer + Copy,
{
    let (x1, x2) = min_max(a1, a2);
    let (y1, y2) = min_max(b1, b2);
    x1 <= y2 && y1 <= x2
}

/// Returns the absolute difference between `a` and `b`.
pub fn diff<T>(a: T, b: T) -> T
where
    T: Integer,
{
    // Note that something like (b-a).abs() wouldn't work for unsigned types.
    if b >= a { b - a } else { a - b }
}

/// Returns true if `n` is between `a` and `b`, inclusive.
#[allow(needless_pass_by_value)]
pub fn between<T>(n: T, a: T, b: T) -> bool
where
    T: Integer,
{
    if b >= a {
        n >= a && n <= b
    } else {
        n >= b && n <= a
    }
}

/// Returns true if `a` and `b` are within `n` units of each other.
#[allow(needless_pass_by_value)]
pub fn in_range<T>(a: T, b: T, n: T) -> bool
where
    T: Integer,
{
    diff(a, b) <= n
}

/// Returns true if `a` and `b` are within one unit of each other.
pub fn in_one<T>(a: T, b: T) -> bool
where
    T: Integer,
{
    in_range(a, b, T::one())
}

#[cfg(test)]
mod tests {
    use util::math::*;
    use util::rand::rand_range;

    #[test]
    fn test_max() {
        assert_eq!(max!(0, 1, 2), 2);
        assert_eq!(max!(2, 1, 0), 2);
        assert_eq!(max!(-1, -2), -1);
    }

    #[test]
    fn test_min() {
        assert_eq!(min!(0, 1, 2), 0);
        assert_eq!(min!(2, 1, 0), 0);
        assert_eq!(min!(-1, -2), -2);
    }

    #[test]
    fn test_overlaps() {
        assert!(overlaps(1, 1, 1, 1));
        assert!(overlaps(1, 2, 0, 1));
        assert!(overlaps(0, 1, 2, 1));
        assert!(overlaps(1, 5, 2, 3));
        assert!(overlaps(2, 3, 1, 5));
        assert!(overlaps(1, 5, 2, 6));
        assert!(overlaps(6, 2, 5, 1));
        assert!(overlaps(-1, -1, -2, 0));

        assert!(!overlaps(0, 1, 2, 4));
        assert!(!overlaps(4, 2, 0, 1));
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(1, 2), 1);
        assert_eq!(diff(4, 0), 4);
        assert_eq!(diff(-1, 1), 2);
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
}