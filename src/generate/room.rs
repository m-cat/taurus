
use std::fmt;
use util::math::{min_max, overlaps};

/// A struct for storing data for a single `Room`, used in dungeon generation.
/// Note that the four bounding boxes correspond to the `Room`'s interior
/// and do not include its walls.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Room {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Room {
    /// Returns a new `Room` with given bounding boxes.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Room {
        let (left, right) = min_max(left, right);
        let (top, bottom) = min_max(top, bottom);

        Room {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    /// Returns a new `Room` created from given `width` and `height`.
    pub fn from_dimensions(left: i32, top: i32, width: usize, height: usize) -> Room {
        debug_assert!(width > 0 && height > 0);

        Room {
            left,
            top,
            right: left + width as i32 - 1,
            bottom: top + height as i32 - 1,
        }
    }

    /// Returns true if `self` and `other` overlap.
    /// Note that we allow walls to overlap, but not so the interiors of the `Room`s
    /// are connected.
    pub fn overlaps(&self, other: &Self) -> bool {
        overlaps(self.left - 1, self.right, other.left - 1, other.right) &&
            overlaps(self.top - 1, self.bottom, other.top - 1, other.bottom)
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {})",
            self.left,
            self.top,
            self.right,
            self.bottom
        )
    }
}

#[cfg(test)]
mod tests {
    use generate::Room;

    #[test]
    fn room_new() {
        assert_eq!(Room::new(1, 1, 2, 2), Room::from_dimensions(1, 1, 2, 2));
    }

    #[test]
    fn room_overlaps() {
        let rooms = vec![
            Room::new(0, 0, 1, 1),
            Room::new(1, 0, 3, 3),
            Room::new(-1, -1, 4, 4),
            Room::new(-3, -3, -2, -2),
        ];

        assert!(rooms[0].overlaps(&rooms[1]));
        assert!(rooms[0].overlaps(&rooms[2]));
        assert!(rooms[2].overlaps(&rooms[3]));
        assert!(!rooms[0].overlaps(&rooms[3]));
        assert!(!rooms[1].overlaps(&rooms[3]));

        for (i, room1) in rooms.iter().enumerate() {
            for (j, room2) in rooms.iter().enumerate() {
                if i == j {
                    assert!(room1.overlaps(room2));
                }
                assert_eq!(room1.overlaps(room2), room2.overlaps(room1));
            }
        }
    }
}
