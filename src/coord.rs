use util;
use util::int;

// Very simple but indispensible struct
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
        util::in_one(self.x, other.x) && util::in_one(self.y, other.y) && self != other
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
        let xy1 = Coord::new(1, 1);
        let xy2 = Coord::new(0, 0);
        let xy3 = Coord::new(2, 2);
        let xy4 = Coord::new(2, 2);
        let xy5 = Coord::new(1, 0);

        assert!(xy1.adjacent(&xy2));
        assert!(xy1.adjacent(&xy3));
        assert!(xy1.adjacent(&xy5));
        assert!(!xy2.adjacent(&xy3));
        assert!(!xy3.adjacent(&xy4));
    }
}
