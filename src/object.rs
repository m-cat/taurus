//! Game objects.

use {GameError, GameResult};
use console::Color;
use coord::Coord;
use database::Database;
use dungeon::Dungeon;
use failure::ResultExt;
use game_data::GameData;
use std::str::FromStr;
use ui::Draw;

/// An `Object` object.
///
/// A data structure for things like doors and traps which can be interacted with. For more about
/// the differences between `Object`s and `Actor`s, see module `actor`.
#[derive(Debug)]
pub struct Object {
    c: char,
    color: Color,

    coord: Coord,

    class: ObjectClass,
    material: Material,
    active: bool,
}

impl Object {
    /// Creates a new `Object` at the given coordinates.
    pub fn new(
        game_data: &GameData,
        coord: Coord,
        data: &Database,
        active: bool,
    ) -> GameResult<Box<Object>> {
        // Load all data from the database.

        let c = data.get_char("c")?;
        let color = Color::from_str(data.get_str("color")?.as_str())?;

        let class = ObjectClass::from_str(data.get_str("class")?.as_str())?;
        let material = Material::from_str(data.get_str("material")?.as_str())?;

        Ok(Box::new(Object {
            c,
            color,
            coord,

            class,
            material,
            active,
        }))
    }

    /// Returns this object's coordinates.
    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord
    }

    /// Returns the character this object is drawn with.
    pub fn draw_c(&self) -> char {
        self.c
    }

    /// Returns the color this object is drawn with.
    pub fn draw_color(&self) -> Color {
        self.color
    }

    pub fn passable(&self) -> bool {
        match self.class {
            ObjectClass::Door => self.active, // For doors, active means open
            ObjectClass::Trap => true,
        }
    }
}

impl Draw for Object {
    fn draw_c(&self) -> char {
        self.c
    }
    fn draw_color(&self) -> Color {
        self.color
    }
}

/// Enum listing possible object classes.
#[derive(Debug)]
pub enum ObjectClass {
    Door,
    Trap,
}

impl FromStr for ObjectClass {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "door" => ObjectClass::Door,
            "trap" => ObjectClass::Trap,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid object class.",
                })
            }
        })
    }
}

/// Enum listing possible object materials.
#[derive(Debug)]
pub enum Material {
    Wood,
    Iron,
}

impl FromStr for Material {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "wood" => Material::Wood,
            "iron" => Material::Iron,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid object material.",
                })
            }
        })
    }
}
