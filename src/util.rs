#![allow(non_camel_case_types)]
#![allow(unknown_lints)]

use std::fmt::Display;

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use rand;
use rand::Rng;
use rand::distributions::range::SampleRange;

use num::Integer;

pub type int = i32;
pub type uint = u32;

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

// MATH FUNCTIONS

/// Returns the absolute difference between a and b.
pub fn diff<T>(a: T, b: T) -> T
    where T: Integer
{
    // Note that something like (b-a).abs() doesn't work for unsigned types.
    if b >= a { b - a } else { a - b }
}

/// Returns true if n is between a and b.
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

/// Returns true if a and b are within n units of each other.
#[allow(needless_pass_by_value)]
pub fn in_range<T>(a: T, b: T, n: T) -> bool
    where T: Integer
{
    diff(a, b) <= n
}

/// Returns true if a and b are within one unit of each other.
pub fn in_one<T>(a: T, b: T) -> bool
    where T: Integer
{
    in_range(a, b, T::one())
}

// RAND FUNCTIONS

/// Returns a random usize in the range [x..y] inclusive.
pub fn rand_range<T>(x: T, y: T) -> T
    where T: Integer + SampleRange
{
    if y > x {
        rand::thread_rng().gen_range(x, y + T::one())
    } else {
        rand::thread_rng().gen_range(y, x + T::one())
    }
}

/// Returns true with x in y chance.
#[allow(needless_pass_by_value)]
pub fn dice<T>(x: T, y: T) -> bool
    where T: Integer + SampleRange + Display
{
    debug_assert!(x <= y, format!("Assert failed: dice({}, {})", x, y));
    rand_range(T::one(), y) <= x
}

pub trait Choose<T> {
    /// Returns an element picked randomly from `&self`, all elements having equal weighting.
    fn choose(&self) -> Option<&T>;
    fn choose_i(&self) -> Option<usize>;
    fn choose_enumerate(&self) -> Option<(usize, &T)>;
}

// VECTOR FUNCTIONS

impl<T> Choose<T> for Vec<T> {
    fn choose(&self) -> Option<&T> {
        rand::thread_rng().choose(self)
    }

    fn choose_i(&self) -> Option<usize> {
        if self.len() > 0 {
            Some(rand_range(0, self.len()-1))
        } else { None }
    }

    fn choose_enumerate(&self) -> Option<(usize, &T)> {
        let i = self.choose_i();
        match i {
            Some(i) => Some((i, &self[i])),
            None => None,
        }
    }
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

// UNIT TESTS

#[cfg(test)]
mod tests {
    use util::*;

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
