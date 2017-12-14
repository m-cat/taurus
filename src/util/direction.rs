//! Direction utility.

use std::fmt;

pub trait Direction {
    fn unit_vec(&self) -> (i32, i32);
}

/// Enum for the eight possible compass directions.
#[derive(Clone, Copy, PartialEq)]
pub enum CompassDirection {
    W,
    N,
    E,
    S,
    NW,
    NE,
    SE,
    SW,
}

impl Direction for CompassDirection {
    fn unit_vec(&self) -> (i32, i32) {
        use self::CompassDirection::*;

        match *self {
            W => (-1, 0),
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
            NW => (-1, -1),
            NE => (1, -1),
            SE => (1, 1),
            SW => (-1, 1),
        }
    }
}

impl fmt::Display for CompassDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CompassDirection::*;

        write!(
            f,
            "{}",
            match *self {
                W => "W",
                N => "N",
                E => "E",
                S => "S",
                NW => "NW",
                NE => "NE",
                SE => "SE",
                SW => "SW",
            }
        )
    }
}

/// Enum for the four possible orthogonal directions.
#[derive(Clone, Copy, PartialEq)]
pub enum CardinalDirection {
    W,
    N,
    E,
    S,
}

impl Direction for CardinalDirection {
    fn unit_vec(&self) -> (i32, i32) {
        use self::CardinalDirection::*;

        match *self {
            W => (-1, 0),
            N => (0, -1),
            E => (1, 0),
            S => (0, 1),
        }
    }
}

impl fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CardinalDirection::*;

        write!(
            f,
            "{}",
            match *self {
                W => "W",
                N => "N",
                E => "E",
                S => "S",
            }
        )
    }
}
