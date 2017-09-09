//! Random number utility functions.

#![allow(unknown_lints)]

use num::Integer;
use rand;
use rand::Rng;
use rand::distributions::range::SampleRange;
use std::fmt::Display;

/// A trait allowing access to random elements and/or indices in implementing containers.
pub trait Choose<T> {
    /// Returns an element picked randomly from `&self`, or `None` if no elements exist.
    fn choose(&self) -> Option<&T>;

    /// Returns a valid index picked randomly from `&self`, or `None` if no index exists.
    fn choose_index(&self) -> Option<usize>;

    /// Returns a valid (value, index) tuple picked randomly, or `None` if none exist.
    fn choose_enumerate(&self) -> Option<(usize, &T)>;
}

impl<T> Choose<T> for Vec<T> {
    fn choose(&self) -> Option<&T> {
        rand::thread_rng().choose(self)
    }

    fn choose_index(&self) -> Option<usize> {
        if !self.is_empty() {
            Some(rand_range(0, self.len() - 1))
        } else {
            None
        }
    }

    fn choose_enumerate(&self) -> Option<(usize, &T)> {
        let i = self.choose_index();
        match i {
            Some(i) => Some((i, &self[i])),
            None => None,
        }
    }
}

/// Returns a random usize in the range `[x, y]` inclusive.
pub fn rand_range<T>(x: T, y: T) -> T
where
    T: Integer + SampleRange,
{
    if y > x {
        rand::thread_rng().gen_range(x, y + T::one())
    } else {
        rand::thread_rng().gen_range(y, x + T::one())
    }
}

/// Returns true with `x` in `y` chance.
#[allow(needless_pass_by_value)]
pub fn dice<T>(x: T, y: T) -> bool
where
    T: Integer + SampleRange + Display,
{
    debug_assert!(x <= y, format!("Assert failed: dice({}, {})", x, y));
    rand_range(T::one(), y) <= x
}

#[cfg(test)]
mod tests {
    use util::rand::{dice, rand_range};

    #[test]
    fn test_dice() {
        for _ in 1..100 {
            assert!(!dice(0, rand_range(1, 100)));
        }
    }
}
