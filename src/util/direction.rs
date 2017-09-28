//! Direction utility.

pub trait Direction {
    fn unit_vec(&self) -> (i32, i32);
}

/// Enum for the eight possible compass directions.
#[derive(Clone, Copy, PartialEq)]
pub enum CompassDirection {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction for CompassDirection {
    fn unit_vec(&self) -> (i32, i32) {
        match *self {
            CompassDirection::N => (0, -1),
            CompassDirection::E => (1, 0),
            CompassDirection::S => (0, 1),
            CompassDirection::W => (-1, 0),
            CompassDirection::NE => (1, -1),
            CompassDirection::SE => (1, 1),
            CompassDirection::SW => (-1, 1),
            CompassDirection::NW => (-1, -1),
        }
    }
}

/// Enum for the four possible orthogonal directions.
#[derive(Clone, Copy, PartialEq)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
}

impl Direction for CardinalDirection {
    fn unit_vec(&self) -> (i32, i32) {
        match *self {
            CardinalDirection::N => (0, -1),
            CardinalDirection::E => (1, 0),
            CardinalDirection::S => (0, 1),
            CardinalDirection::W => (-1, 0),
        }
    }
}
