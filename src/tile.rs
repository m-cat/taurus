//! Game tiles.

use {GAMEDATA, GameResult};
use actor::Actor;
use console::Color;
use database::Database;
use error::GameError;
use game_data::GameData;
use item::ItemStash;
use material::MaterialInfo;
use object::Object;
use std::cell::RefCell;
use std::str::FromStr;
use std::sync::Arc;
use ui::Draw;

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
    pub fn new(game_data: &GameData, tile_data: &Database) -> GameResult<TileInfo> {
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

    pub fn color(&self) -> Color {
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
    pub last_seen: Option<Arc<TileInfo>>,

    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub item_stash: Option<Box<ItemStash>>,

    /// Elevation of this tile in the heightmap. 0 is ground level, < 0 is below ground.
    pub height: i32,
}

impl Tile {
    /// Returns a new `Tile` object.
    #[cfg_attr(feature = "dev", flame)]
    pub fn new(tile_data: &Database) -> GameResult<Tile> {
        let id = tile_data.id();
        let info = GAMEDATA.read().unwrap().tile_info(id);

        Ok(Tile {
            info,

            last_seen: None,

            actor: None,
            object: None,
            item_stash: None,

            height: 0,
        })
    }

    pub fn set_tile_info(&mut self, tile_data: &Database) -> GameResult<()> {
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

impl Draw for Tile {
    fn draw_c(&self) -> char {
        self.info.c
    }
    fn draw_color(&self) -> Color {
        self.info.color()
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
                })
            }
        })
    }
}
