//! Taurus - tile.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use taurus::utility::{int, uint};

pub struct Tile {
    /// The class of tile, containing the tile kind
    class: TileClass,
    /// Depth of this tile in the heightmap. 0 is ground level
    depth: int,
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

impl Tile {
    fn about(&self) -> &str {
        match TileClass {
            Floor(f) => match f {
                Stone => "a stone floor",
            },
            Wall(w) => match w {},
            Hole(h) => match h {},
    }
