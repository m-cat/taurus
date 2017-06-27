#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![macro_use]

use std::fmt::Display;

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::{min, max};

use rand;
use rand::Rng;
use rand::distributions::range::SampleRange;

use num::Integer;

// DEFINE TYPES

/// Defines the standard type for signed ints
pub type int = i32;

/// Defines the standard type for unsigned ints
pub type uint = u32;

// MATH FUNCTIONS

/// Returns a tuple (min, max) of `a` and `b`.
pub fn min_max<T>(a: T, b: T) -> (T, T)
    where T: Integer + Copy
{
    (min(a, b), max(a, b))
}

/// Returns true if two inclusive ranges `[a1, a2]` and `[b1, b2]` overlap.
pub fn overlaps<T>(a1: T, a2: T, b1: T, b2: T) -> bool
    where T: Integer + Copy
{
    let (x1, x2) = min_max(a1, a2);
    let (y1, y2) = min_max(b1, b2);
    x1 <= y2 && y1 <= x2
}

/// Returns the absolute difference between `a` and `b`.
pub fn diff<T>(a: T, b: T) -> T
    where T: Integer
{
    // Note that something like (b-a).abs() wouldn't work for unsigned types.
    if b >= a { b - a } else { a - b }
}

/// Returns true if `n` is between `a` and `b`, inclusive.
#[allow(needless_pass_by_value)]
pub fn between<T>(n: T, a: T, b: T) -> bool
    where T: Integer
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
    where T: Integer
{
    diff(a, b) <= n
}

/// Returns true if `a` and `b` are within one unit of each other.
pub fn in_one<T>(a: T, b: T) -> bool
    where T: Integer
{
    in_range(a, b, T::one())
}

// RAND FUNCTIONS

/// Returns a random usize in the range `[x, y]` inclusive.
pub fn rand_range<T>(x: T, y: T) -> T
    where T: Integer + SampleRange
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
    where T: Integer + SampleRange + Display
{
    debug_assert!(x <= y, format!("Assert failed: dice({}, {})", x, y));
    rand_range(T::one(), y) <= x
}

/// A trait allowing access to random elements and/or indices in implementing containers.
pub trait Choose<T> {
    /// Returns an element picked randomly from `&self`, so `None` if no elements exist.
    fn choose(&self) -> Option<&T>;

    /// Returns a valid index picked randomly from `&self`, or `None` if no index exists.
    fn choose_index(&self) -> Option<usize>;

    /// Returns a valid (value, index) tuple picked randomly, or `None` if none exist.
    fn choose_enumerate(&self) -> Option<(usize, &T)>;
}

// VECTOR FUNCTIONS

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

// PATH FUNCTIONS

/// Makes a path by joining a filename to a directory.
///
/// # Examples
/// ```
/// use taurus::util;
///
/// let path = util::make_path_join("a/b", "test.txt");
/// assert_eq!(path.to_str(), Some("a/b/test.txt"));
/// ```
pub fn make_path_join(dir: &str, s1: &str) -> PathBuf {
    let dir_path = Path::new(dir);
    dir_path.join(Path::new(s1))
}

/// Returns the path created by concatenating two strings.
pub fn make_path_cat(s1: &str, s2: &str) -> PathBuf {
    let name = format!("{}{}", s1, s2);
    Path::new(name.as_str()).to_path_buf()
}

/// Makes a path by concatenating two filenames, then joining to a directory.
pub fn make_path_cat_join(dir: &str, s1: &str, s2: &str) -> PathBuf {
    let dir_path = Path::new(dir);
    dir_path.join(make_path_cat(s1, s2))
}

// FILE IO FUNCTIONS

/// Reads a file and returns its contents in a string.
pub fn read_file_str(path: &Path) -> io::Result<String> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path)?;

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Reads a file and returns its contents as lines in Vec<String>.
/// Each string returned will not have an ending newline.
pub fn read_file_vec(path: &Path) -> io::Result<Vec<String>> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();

    // Add each line to the output Vec
    for line in reader.lines() {
        match line {
            Ok(line) => vec.push(line),
            Err(e) => return Err(e),
        }
    }

    Ok(vec)
}

/// Writes a string to a file with a given path.
pub fn write_file_str(path: &Path, contents: &str) -> io::Result<()> {
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;

    // Write the string to `file`, returns `io::Result<()>`
    file.write_all(contents.as_bytes())?;

    Ok(())
}

/// Writes a Vec<String> to a file with a given path.
pub fn write_file_vec(path: &Path, contents: &[String]) -> io::Result<()> {
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;
    let newline = b"\n";

    // Write each string to `file`, returns `io::Result<()>`
    for line in contents {
        file.write_all(line.as_bytes())?;
        file.write_all(newline)?;
    }

    Ok(())
}

/// Compares two files for equality.
pub fn files_equal(path1: &Path, path2: &Path) -> io::Result<bool> {
    let s1 = match read_file_str(path1) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let s2 = match read_file_str(path2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    Ok(s1 == s2)
}

// ENUMS

/// Enum delineating the 8 possible cardinal directions.
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

// MACROS

/// Returns the max of all given elements.
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

/// Tries evaluating `$e` `$n` times, returning `Some(s)` the first time `$e` evaluates to `Some`
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

// UNIT TESTS

#[cfg(test)]
mod tests {
    use util::*;

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

    #[test]
    fn test_dice() {
        for _ in 1..100 {
            assert!(!dice(0, rand_range(1, 100)));
        }
    }

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

}
