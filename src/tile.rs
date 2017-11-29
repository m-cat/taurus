//! Game tiles.

use GameResult;
use actor::Actor;
use console::Color;
use database::Database;
use game_data::GameData;
use item::ItemStack;
use object::Object;
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

    pub actor: Option<Actor>,
    pub object: Option<Box<Object>>,
    pub stack: Option<Box<ItemStack>>,

    /// Elevation of this tile in the heightmap. 0 is ground level, <0 is below ground.
    pub height: i32,
}

impl Tile {
    /// Returns a new `Tile` object.
    pub fn new(game_data: &mut GameData, tile_data: &Database) -> GameResult<Tile> {
        let name = tile_data.get_str("name")?;

        // If this tile was already loaded, grab the existing `TileInfo`.
        // Otherwise, create a new one for this tile and set the id.
        let info = game_data.tile_info(&name).unwrap_or(Self::new_tile_info(
            game_data,
            tile_data,
            name,
        )?);

        Ok(Tile {
            info: Rc::clone(&info),

            actor: None,
            object: None,
            stack: None,

            height: 0,
        })
    }

    fn new_tile_info(
        game_data: &mut GameData,
        tile_data: &Database,
        name: String,
    ) -> GameResult<Rc<TileInfo>> {
        let c = tile_data.get_char("c")?;
        let color = Color::from_str(&tile_data.get_str("color")?)?;

        let passable = tile_data.get_bool("passable")?;
        let transparent = tile_data.get_bool("transparent")?;

        // Create the `TileInfo`.
        let tile_info = TileInfo {
            c,
            color,
            passable,
            transparent,
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
}

impl TileInfo {
    pub fn c(&self) -> char {
        self.c
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
