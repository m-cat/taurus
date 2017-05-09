use util;

// Very simple but indispensible struct
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Return true if two Coords are adjacent and NOT equal
    pub fn adjacent(&self, other: &Self) -> bool {
        util::in_one(self.x, other.x) && util::in_one(self.y, other.y) && self != other
    }
}

#[cfg(test)]
mod tests {
    use coord::*;

    #[test]
    fn test_equals() {
        assert_eq!(Coord { x: 2, y: 2 }, Coord { x: 2, y: 2 });
        assert_ne!(Coord { x: 2, y: 2 }, Coord { x: 2, y: 3 });
        assert_ne!(Coord { x: 2, y: 2 }, Coord { x: 3, y: 2 });
    }

    #[test]
    fn test_adjacent() {
        let xy1 = Coord { x: 1, y: 1 };
        let xy2 = Coord { x: 0, y: 0 };
        let xy3 = Coord { x: 2, y: 2 };
        let xy4 = Coord { x: 2, y: 2 };
        let xy5 = Coord { x: 1, y: 0 };

        assert!(xy1.adjacent(&xy2));
        assert!(xy1.adjacent(&xy3));
        assert!(xy1.adjacent(&xy5));
        assert!(!xy2.adjacent(&xy3));
        assert!(!xy3.adjacent(&xy4));
    }
}
