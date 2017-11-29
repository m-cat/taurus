//! Coordinate utility.

use std::fmt;
use util::direction::Direction;
use util::math::in_one;

/// Simple coordinate struct.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    /// Returns true if two `Coord`s are adjacent and NOT equal.
    pub fn is_adjacent(&self, other: &Self) -> bool {
        in_one(self.x, other.x) && in_one(self.y, other.y) && self != other
    }

    /// Gets the `Coord` `n` steps in direction `dir`.
    pub fn coord_in_dir<D>(&self, dir: &D, n: i32) -> Coord
    where
        D: Direction,
    {
        let (dx, dy) = dir.unit_vec();

        Coord::new(self.x + dx * n, self.y + dy * n)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use coord::*;
    use util::direction::CompassDirection as Dir;

    #[test]
    fn equals() {
        assert_eq!(Coord::new(2, 2), Coord::new(2, 2));
        assert_ne!(Coord::new(2, 2), Coord::new(2, 3));
        assert_ne!(Coord::new(2, 2), Coord::new(3, 2));
    }

    #[test]
    fn is_adjacent() {
        let coord1 = Coord::new(1, 1);
        let coord2 = Coord::new(0, 0);
        let coord3 = Coord::new(2, 2);
        let coord4 = Coord::new(2, 2);
        let coord5 = Coord::new(1, 0);

        assert!(coord1.is_adjacent(&coord2));
        assert!(coord1.is_adjacent(&coord3));
        assert!(coord1.is_adjacent(&coord5));
        assert!(!coord2.is_adjacent(&coord3));
        assert!(!coord3.is_adjacent(&coord4));
    }

    #[test]
    fn coord_in_dir() {
        let coord = Coord::new(0, 0);

        assert_eq!(coord.coord_in_dir(&Dir::N, 1), Coord::new(0, -1));
        assert_eq!(coord.coord_in_dir(&Dir::E, 1), Coord::new(1, 0));
        assert_eq!(coord.coord_in_dir(&Dir::S, 1), Coord::new(0, 1));
        assert_eq!(coord.coord_in_dir(&Dir::W, 1), Coord::new(-1, 0));
        assert_eq!(coord.coord_in_dir(&Dir::NE, 1), Coord::new(1, -1));
        assert_eq!(coord.coord_in_dir(&Dir::SE, 1), Coord::new(1, 1));
        assert_eq!(coord.coord_in_dir(&Dir::NW, 1), Coord::new(-1, -1));
        assert_eq!(coord.coord_in_dir(&Dir::SW, 1), Coord::new(-1, 1));

        assert_eq!(coord.coord_in_dir(&Dir::N, -2), Coord::new(0, 2));
        assert_eq!(coord.coord_in_dir(&Dir::SW, -2), Coord::new(2, -2));
    }
}
