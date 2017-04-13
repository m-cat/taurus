//! Taurus - tile.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use taurus::util::{int, uint};

pub struct Tile {
    /// The class of tile, containing the tile kind
    class: TileClass,
    /// Depth of this tile in the heightmap. 0 is ground level
    depth: int,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            class: TileClass::Floor(FloorKind::Stone),
            depth: 0, // default is ground level
        }
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
