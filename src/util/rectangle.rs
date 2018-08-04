use std::fmt;
use util::math::{min_max, overlaps};

/// A struct for storing data for a single `Room`, used in dungeon generation.
/// Note that the four bounding boxes correspond to the `Rectangle`'s interior
/// and do not include its walls.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rectangle {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rectangle {
    /// Returns a new `Rectangle` with given bounding boxes.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rectangle {
        let (left, right) = min_max(left, right);
        let (top, bottom) = min_max(top, bottom);

        Rectangle {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Returns a new `Rectangle` created from given `width` and `height`.
    pub fn from_dimensions(left: i32, top: i32, width: usize, height: usize) -> Rectangle {
        debug_assert!(width > 0 && height > 0);

        Rectangle {
            left,
            top,
            right: left + width as i32 - 1,
            bottom: top + height as i32 - 1,
        }
    }

    /// Returns true if `self` and `other` overlap.
    /// Note that we allow walls to overlap, but not so the interiors of the `Rectangle`s
    /// are connected.
    pub fn overlaps(&self, other: &Self) -> bool {
        overlaps(self.left - 1, self.right, other.left - 1, other.right)
            && overlaps(self.top - 1, self.bottom, other.top - 1, other.bottom)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {})",
            self.left, self.top, self.right, self.bottom
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn rectangle_new() {
        assert_eq!(
            Rectangle::new(1, 1, 2, 2),
            Rectangle::from_dimensions(1, 1, 2, 2)
        );

        assert_eq!(
            Rectangle::new(0, 0, 0, 0),
            Rectangle::from_dimensions(0, 0, 1, 1)
        );
    }

    #[test]
    fn rectangle_overlaps() {
        let rectangles = vec![
            Rectangle::new(0, 0, 1, 1),
            Rectangle::new(1, 0, 3, 3),
            Rectangle::new(-1, -1, 4, 4),
            Rectangle::new(-3, -3, -2, -2),
        ];

        assert!(rectangles[0].overlaps(&rectangles[1]));
        assert!(rectangles[0].overlaps(&rectangles[2]));
        assert!(rectangles[2].overlaps(&rectangles[3]));
        assert!(!rectangles[0].overlaps(&rectangles[3]));
        assert!(!rectangles[1].overlaps(&rectangles[3]));

        for (i, rectangle1) in rectangles.iter().enumerate() {
            for (j, rectangle2) in rectangles.iter().enumerate() {
                if i == j {
                    assert!(rectangle1.overlaps(rectangle2));
                }
                assert_eq!(
                    rectangle1.overlaps(rectangle2),
                    rectangle2.overlaps(rectangle1)
                );
            }
        }
    }
}
