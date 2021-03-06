//! Random number utility functions.

use num::rational::Ratio;
use num::{Bounded, Integer};
use rand;
use rand::distributions::uniform::SampleUniform;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::Display;

/// Trait allowing access to random elements and/or indices in implementing containers.
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
        self.as_slice().choose(&mut rand::thread_rng())
    }

    fn choose_index(&self) -> Option<usize> {
        if !self.is_empty() {
            Some(rand_int(0, self.len() - 1))
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

/// Returns a random Integer in the range `[x, y]` inclusive.
pub fn rand_int<T>(x: T, y: T) -> T
where
    T: Integer + SampleUniform,
{
    rand::thread_rng().gen_range(x, y + T::one())
}

/// Returns a random Ratio in the inclusive range `[x, y]` with the given denominator.
///
/// # Panics
/// This function can result in an overflow - use only for known inputs.
pub fn rand_ratio<T>(x: T, y: T, d: T) -> Ratio<T>
where
    T: Clone + Copy + Integer + SampleUniform,
{
    Ratio::new(rand_int(x * d, y * d), d)
}

/// Returns true with `x` in `y` chance.
pub fn dice<T>(x: T, y: T) -> bool
where
    T: Copy + Display + Integer + SampleUniform,
{
    debug_assert!(x <= y, format!("Assert failed: dice({}, {})", x, y));
    rand_int(T::one(), y) <= x
}

/// Returns true with `x` chance, where 0 <= `x` <= 1.
pub fn chance<T>(x: Ratio<T>) -> bool
where
    T: Bounded + Clone + Copy + Integer + SampleUniform,
{
    let max = T::max_value() - T::one();
    Ratio::new(rand_int(T::min_value(), max), max) <= x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::math::between;

    #[test]
    fn test_rand_int() {
        for _ in 1..100 {
            let (a, b) = (rand_int(0, 1000), rand_int(0, 1000));
            assert!(between(rand_int(a.min(b), a.max(b)), a, b))
        }
    }

    #[test]
    fn test_dice() {
        for _ in 1..100 {
            assert!(!dice(0, rand_int(1, 100)));
        }
    }
}
