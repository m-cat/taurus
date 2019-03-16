//! Game tiles.

use crate::actor::Actor;
use crate::console::Color;
use crate::error::GameError;
use crate::game_data::GameData;
use crate::item::ItemStash;
use crate::material::MaterialInfo;
use crate::object::Object;
use crate::ui::Draw;
use crate::{GameResult, GAMEDATA};
use over::Obj;
use std::cell::{Cell, RefCell};
use std::str::FromStr;
use std::sync::Arc;

/// Struct containing the tile information for a class of tiles.
#[derive(Debug)]
pub struct TileInfo {
    pub material: Arc<MaterialInfo>,

    pub name: String,
    pub c: char,

    /// Can the tile be walked on?
    pub passable: bool,
    /// Is the tile see-through?
    pub transparent: bool,

    pub staircase: Staircase,
}

impl TileInfo {
    pub fn new(game_data: &GameData, tile_data: &Obj) -> GameResult<TileInfo> {
        let _material = tile_data.get_obj("material")?;
        let mname = _material.get_str("name")?;
        let id = _material.id();
        let material = game_data.material_info(id);

        let name = tile_data.get_str("name")?;
        let c = tile_data.get_char("c")?;

        let passable = tile_data.get_bool("passable")?;
        let transparent = tile_data.get_bool("transparent")?;

        let staircase = Staircase::from_str(&tile_data.get_str("staircase")?)?;

        // Create the `TileInfo`.
        let tile_info = TileInfo {
            material,

            name,
            c,

            passable,
            transparent,

            staircase,
        };

        Ok(tile_info)
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.material.adjective, self.name)
    }
}

impl Draw for TileInfo {
    fn draw_c(&self) -> char {
        self.c
    }
    fn draw_color(&self) -> Color {
        self.material.color
    }
}

/// Tile object.
///
/// Tiles are represented differently from objects and actors. Because tiles are so common, we try
/// to save space by only storing a reference to a unique `TileInfo` object. Any two tiles of the
/// same type will store a reference to the same `TileInfo`. This has a slight negative impact on
/// performance but saves a considerable amount of memory.
#[derive(Clone, Debug)]
pub struct Tile {
    /// A reference to the `TileInfo`.
    pub info: Arc<TileInfo>,
    /// Last seen tile here. This also tells us whether this tile has been seen before.
    pub last_seen: Cell<Option<(char, Color)>>,

    /// Elevation of this tile in the heightmap. 0 is ground level, < 0 is below ground.
    pub height: i32,

    // Not serialized:
    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub item_stash: Option<Box<ItemStash>>,
}

impl Tile {
    /// Returns a new `Tile` object.
    pub fn new(tile_data: &Obj) -> GameResult<Tile> {
        let id = tile_data.id();
        let info = GAMEDATA.read().unwrap().tile_info(id);

        Ok(Tile {
            info,
            last_seen: Cell::new(None),

            height: 0,

            actor: None,
            object: None,
            item_stash: None,
        })
    }

    pub fn set_tile_info(&mut self, tile_data: &Obj) -> GameResult<()> {
        let id = tile_data.id();

        self.info = GAMEDATA.read().unwrap().tile_info(id);

        Ok(())
    }
}

impl Tile {
    pub fn passable(&self) -> bool {
        self.info.passable
    }

    pub fn transparent(&self) -> bool {
        self.info.transparent
    }

    pub fn staircase(&self) -> Staircase {
        self.info.staircase
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Staircase {
    Up,
    Down,
    UpDown,
    None,
}

impl FromStr for Staircase {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "up" => Staircase::Up,
            "down" => Staircase::Down,
            "updown" => Staircase::UpDown,
            "none" => Staircase::None,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid staircase value",
                });
            }
        })
    }
}
