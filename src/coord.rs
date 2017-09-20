use util::int;
use util::math::in_one;

/// Simple coordinate struct.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
pub struct Coord {
    pub x: int,
    pub y: int,
}

impl Coord {
    pub fn new(x: int, y: int) -> Coord {
        Coord { x: x, y: y }
    }

    /// Returns true if two Coords are adjacent and NOT equal.
    pub fn adjacent(&self, other: &Self) -> bool {
        in_one(self.x, other.x) && in_one(self.y, other.y) && self != other
    }
}

#[cfg(test)]
mod tests {
    use coord::*;

    #[test]
    fn test_equals() {
        assert_eq!(Coord::new(2, 2), Coord::new(2, 2));
        assert_ne!(Coord::new(2, 2), Coord::new(2, 3));
        assert_ne!(Coord::new(2, 2), Coord::new(3, 2));
    }

    #[test]
    fn test_adjacent() {
        let coord1 = Coord::new(1, 1);
        let coord2 = Coord::new(0, 0);
        let coord3 = Coord::new(2, 2);
        let coord4 = Coord::new(2, 2);
        let coord5 = Coord::new(1, 0);

        assert!(coord1.adjacent(&coord2));
        assert!(coord1.adjacent(&coord3));
        assert!(coord1.adjacent(&coord5));
        assert!(!coord2.adjacent(&coord3));
        assert!(!coord3.adjacent(&coord4));
    }
}
