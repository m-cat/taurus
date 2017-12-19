//! Game objects.

use {GameError, GameResult};
use console::Color;
use coord::Coord;
use database::Database;
use defs::{GameRatio, to_gameratio};
use dungeon::{ActResult, Dungeon};
use failure::ResultExt;
use game_data::GameData;
use std::cell::Cell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use std::str::FromStr;
use ui::Draw;
use util::rand;

#[derive(Debug)]
pub struct ObjectInner {
    object_type: ObjectType,
    material: Material,
    active: bool,

    name: String, // Generic name.
    c: char,
    color: Color,

    coord: Coord,
    turn: GameRatio,
    speed: GameRatio,
}

impl ObjectInner {
    /// Returns a copy of this object's coordinates.
    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord
    }

    /// Returns this object's next turn value.
    pub fn turn(&self) -> GameRatio {
        self.turn
    }

    pub fn visible(&self) -> bool {
        if self.object_type == ObjectType::Door && !self.active {
            false
        } else {
            true
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active
    }

    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
}

impl Draw for ObjectInner {
    fn draw_c(&self) -> char {
        self.c
    }
    fn draw_color(&self) -> Color {
        self.color
    }
}

/// An `Object` object.
///
/// A data structure for things like doors and traps which can be interacted with. For more about
/// the differences between `Object`s and `Actor`s, see module `actor`.
#[derive(Clone, Debug)]
pub struct Object {
    pub inner: Rc<RefCell<ObjectInner>>,
}

impl Object {
    /// Creates a new `Object` at the given coordinates.
    pub fn new(
        game_data: &GameData,
        coord: Coord,
        object_data: &Database,
        active: bool,
    ) -> GameResult<Object> {
        // Load all data from the database.

        let object_type = ObjectType::from_str(object_data.get_str("type")?.as_str())?;
        let material = Material::from_str(object_data.get_str("material")?.as_str())?;

        let name = object_data.get_str("name")?;
        let c = object_data.get_char("c")?;
        let color = Color::from_str(object_data.get_str("color")?.as_str())?;

        let speed = to_gameratio(object_data.get_frac("speed")?)?;

        // Create the object instance.

        let mut object = Object {
            inner: Rc::new(RefCell::new(ObjectInner {
                object_type,
                material,
                active,

                name,
                c,
                color,

                coord: coord,
                turn: game_data.turn(),
                speed,
            })),
        };
        object.update_turn();

        Ok(object)
    }

    pub fn insert_new(
        dungeon: &mut Dungeon,
        coord: Coord,
        object_data: &Database,
        active: bool,
    ) -> GameResult<()> {
        let o = Self::new(dungeon.game_data(), coord, object_data, active)
            .context(format!("Could not load object:\n{}", object_data))?;
        dungeon.add_object(o);
        Ok(())
    }

    pub fn name(&self) -> String {
        self.inner.borrow().name.clone()
    }

    /// Returns a copy of this object's coordinates.
    pub fn coord(&self) -> Coord {
        self.inner.borrow().coord()
    }

    pub fn set_coord(&mut self, coord: Coord) {
        self.inner.borrow_mut().set_coord(coord);
    }

    /// Returns this object's next turn value.
    pub fn turn(&self) -> GameRatio {
        self.inner.borrow().turn
    }

    /// Updates this object's turn based on its speed.
    pub fn update_turn(&mut self) {
        let mut inner = self.inner.borrow_mut();
        let (turn, speed) = (inner.turn, inner.speed);
        inner.turn = turn + speed;
    }

    /// Returns this object's base speed.
    pub fn speed(&self) -> GameRatio {
        self.inner.borrow().speed
    }

    pub fn passable(&self) -> bool {
        let inner = self.inner.borrow();
        match inner.object_type {
            ObjectType::Door => inner.active, // For doors, active means open
            ObjectType::Trap => true,
        }
    }

    pub fn visible(&self) -> bool {
        self.inner.borrow().visible()
    }

    /// Acts out the object's turn. Yes, objects can act, too.
    /// Could change itself or the dungeon as a side effect.
    pub fn act(&mut self, dungeon: &Dungeon) -> ActResult {
        let mut inner = self.inner.borrow_mut();
        // TODO: rework this
        if !inner.active && inner.object_type == ObjectType::Door {
            if dungeon[inner.coord()].actor.is_none() && rand::dice(1, 2) {
                inner.active = true;
            }
        }

        ActResult::None
    }
}

// Traits for priority queue

impl Eq for Object {}
impl PartialEq for Object {
    /// Returns a1 == a2 iff their `turn` values are equal.
    fn eq(&self, other: &Self) -> bool {
        self.turn() == other.turn()
    }
}

impl Ord for Object {
    /// Compares objects by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        // Since we're comparing floating values here, we have to use partial_cmp.
        // We should never do an invalid comparison here, so this is okay.
        other.turn().partial_cmp(&self.turn()).unwrap()
    }
}
impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Enum listing possible object types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectType {
    /// Door type. A value of `true` for `active` means the door is closed.
    Door,
    Trap,
}

impl FromStr for ObjectType {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "door" => ObjectType::Door,
            "trap" => ObjectType::Trap,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid object type.",
                })
            }
        })
    }
}

/// Enum listing possible object materials.
#[derive(Clone, Copy, Debug, PartialEq)]
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
