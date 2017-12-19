//! Game tiles.

use GameResult;
use actor::Actor;
use console::Color;
use database::Database;
use error::GameError;
use game_data::GameData;
use item::ItemStash;
use object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use ui::Draw;

/// Tile object.
///
/// Tiles are represented differently from objects and actors. Because tiles are so common, we try
/// to save space by only storing a reference to a unique `TileInfo` object. Any two tiles of the
/// same type will store a reference to the same `TileInfo`. This has a slight negative impact on
/// performance but saves a considerable amount of memory.
#[derive(Debug)]
pub struct Tile {
    /// A reference to the `TileInfo`.
    info: Rc<TileInfo>,

    /// Last seen tile here. This also tells us whether this tile has been seen before.
    last_seen: Option<Rc<TileInfo>>,

    pub actor: Option<Actor>,
    pub object: Option<Object>,
    pub stash: Option<Box<ItemStash>>,

    /// Elevation of this tile in the heightmap. 0 is ground level, < 0 is below ground.
    pub height: i32,
}

impl Tile {
    /// Returns a new `Tile` object.
    pub fn new(game_data: &GameData, tile_data: &Database) -> GameResult<Tile> {
        let name = tile_data.get_str("name")?;

        // If this tile was already loaded, grab the existing `TileInfo`.
        // Otherwise, create a new one for this tile and set the id.
        let info = match game_data.tile_info(&name) {
            Some(info) => info,
            None => Self::new_tile_info(game_data, tile_data, name)?,
        };

        Ok(Tile {
            info: Rc::clone(&info),

            last_seen: None,

            actor: None,
            object: None,
            stash: None,

            height: 0,
        })
    }

    pub fn set_tile_info(&mut self, game_data: &GameData, tile_data: &Database) -> GameResult<()> {
        let name = tile_data.get_str("name")?;

        // If this tile was already loaded, grab the existing `TileInfo`.
        // Otherwise, create a new one for this tile and set the id.
        let info = match game_data.tile_info(&name) {
            Some(info) => info,
            None => Self::new_tile_info(game_data, tile_data, name)?,
        };

        self.info = Rc::clone(&info);

        Ok(())
    }

    fn new_tile_info(
        game_data: &GameData,
        tile_data: &Database,
        name: String,
    ) -> GameResult<Rc<TileInfo>> {
        let c = tile_data.get_char("c")?;
        let color = Color::from_str(&tile_data.get_str("color")?)?;

        let passable = tile_data.get_bool("passable")?;
        let transparent = tile_data.get_bool("transparent")?;

        let staircase = Staircase::from_str(&tile_data.get_str("staircase")?)?;

        // Create the `TileInfo`.
        let tile_info = TileInfo {
            c,
            color,

            passable,
            transparent,

            staircase,
        };

        Ok(game_data.add_tile_info(tile_info, name))
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
        self.info.c()
    }
    fn draw_color(&self) -> Color {
        self.info.color()
    }
}

/// Struct containing the tile information for a class of tiles.
#[derive(Debug)]
pub struct TileInfo {
    /// The tile character.
    c: char,
    /// The color of this tile.
    color: Color,

    /// Can the tile be walked on?
    passable: bool,
    /// Is the tile see-through?
    transparent: bool,

    staircase: Staircase,
}

impl TileInfo {
    pub fn c(&self) -> char {
        self.c
    }

    pub fn color(&self) -> Color {
        self.color
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
