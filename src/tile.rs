use util::{int, uint};

pub struct Tile {
    /// The class of tile, containing the tile kind
    class: TileClass,
    /// Depth of this tile in the heightmap. 0 is ground level
    depth: int,
}

impl Tile {
    pub fn new(class: TileClass) -> Tile {
        Tile {
            class: class,
            depth: 0, // default is ground level
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::new(TileClass::Floor(FloorKind::Stone))
    }
}

pub enum TileClass {
    Floor(FloorKind),
    Wall(WallKind),
    Hole(HoleKind),
}

pub enum FloorKind {
    Stone,
}

pub enum WallKind {}
pub enum HoleKind {}
