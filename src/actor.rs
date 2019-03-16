//! Game actors.

use crate::console::{Color, DrawConsole};
use crate::coord::Coord;
use crate::defs::*;
use crate::dungeon::{ActResult, Dungeon};
use crate::game_data::GameData;
use crate::object::ObjectType;
use crate::player;
use crate::ui::Draw;
use crate::util::direction::CompassDirection;
use crate::{GameError, GameResult, GAMEDATA};
use failure::ResultExt;
use over::Obj;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ActorInner {
    pub name: String, // Generic name.

    pub c: char,
    pub color: Color,

    pub coord: Coord, // Coordinate location in level.
    pub turn: GameRatio,
    pub speed: GameRatio,

    // STATS
    pub hp_cur: i32, // Current health. This value can be negative!
    pub hp_max: u32,
    pub fov_radius: u32,

    // COMBAT STATE
    pub visible: bool,

    // AI ATTRIBUTES
    pub behavior: Behavior,
}

/// Actor object.
///
/// The `Actor` struct is similar to the `Object` struct, with some key differences. Both can act;
/// for example, a door can close by itself. However, `Object`s do not have all the extra combat
/// attributes that `Actor`s do. Also, only one `Actor` can occupy a tile at a time, but an `Actor`
/// can coexist with an `Object` and will always be drawn on top of it.
#[derive(Clone, Debug)]
pub struct Actor {
    pub inner: Arc<Mutex<ActorInner>>,
}

impl Actor {
    /// Creates a new actor at the given coordinates.
    pub fn new(coord: Coord, data: &Obj) -> GameResult<Actor> {
        // Load all data from the database.

        let name = data.get_str("name")?;
        let c = data.get_char("c")?;
        let color = Color::from_str(data.get_str("color")?.as_str())?;

        let speed = bigr_to_gamer(data.get_frac("speed")?)?;

        let hp = big_to_u32(data.get_int("hp")?)?;
        let hp_cur = hp as i32;
        let hp_max = hp;
        let fov_radius = big_to_u32(data.get_int("fov_radius")?)?;

        let visible = data.get_bool("visible")?;

        let behavior = Behavior::from_str(data.get_str("behavior")?.as_str())?;

        // Create the actor instance.

        let mut actor = Actor {
            inner: Arc::new(Mutex::new(ActorInner {
                name,
                c,
                color,

                coord,
                // We update this after creating the actor.
                turn: GAMEDATA.read().unwrap().turn(),
                speed,

                hp_cur,
                hp_max,
                fov_radius,

                visible,

                behavior,
            })),
        };
        actor.update_turn(); // Set the actor's turn.

        Ok(actor)
    }

    pub fn insert_new(dungeon: &mut Dungeon, coord: Coord, actor_data: &Obj) -> GameResult<()> {
        let a = Self::new(coord, actor_data)
            .context(format!("Could not load actor:\n{}", actor_data))?;
        dungeon.add_actor(a);
        Ok(())
    }

    /// Returns the name associated with this actor.
    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
    }

    /// Generates and returns the description of this actor.
    pub fn description(&self) -> String {
        // TODO
        "test".to_string()
    }

    /// Returns this actor's coordinates.
    pub fn coord(&self) -> Coord {
        self.inner.lock().unwrap().coord
    }

    /// Sets this actor's coordinates.
    pub fn set_coord(&mut self, coord: Coord) {
        self.inner.lock().unwrap().coord = coord;
    }

    /// Returns this actor's next turn value.
    pub fn turn(&self) -> GameRatio {
        self.inner.lock().unwrap().turn
    }

    /// Updates this actor's turn based on its speed.
    pub fn update_turn(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        let (turn, speed) = (inner.turn, inner.speed);
        inner.turn = turn + speed;
    }

    /// Returns this actor's base speed.
    pub fn speed(&self) -> GameRatio {
        self.inner.lock().unwrap().speed
    }

    pub fn visible(&self) -> bool {
        self.inner.lock().unwrap().visible
    }

    /// Returns this actor's behavior value.
    pub fn behavior(&self) -> Behavior {
        self.inner.lock().unwrap().behavior
    }

    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.inner.lock().unwrap().behavior = behavior;
    }

    /// Acts out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    pub fn act(&mut self, dungeon: &mut Dungeon) -> ActResult {
        match self.behavior() {
            Behavior::Player => player::player_act(self, dungeon),
            _ => ActResult::None,
        }
    }

    /// Tries to move in the specified direction.
    pub fn try_move_dir(
        &mut self,
        dungeon: &mut Dungeon,
        dir: CompassDirection,
    ) -> (ActResult, bool) {
        let coord = self.coord().coord_in_dir(&dir, 1);
        self.try_move_to(dungeon, coord)
    }

    // Tries to move to the specified coordinate. Returns true if the actor uses up a turn.
    fn try_move_to(&mut self, dungeon: &mut Dungeon, coord: Coord) -> (ActResult, bool) {
        let passable = {
            let tile = &mut dungeon[coord];
            let passable = tile.passable();

            if let Some(ref _actor) = tile.actor {
                false
            } else if let Some(ref mut object) = tile.object {
                let mut object = object.inner.lock().unwrap();
                match object.object_type() {
                    ObjectType::Door => {
                        if object.active() {
                            // Open door.
                            object.set_active(false);
                            false
                        } else {
                            true
                        }
                    }
                    _ => passable,
                }
            } else {
                passable
            }
        };

        if passable {
            self.move_to(dungeon, coord);
            (ActResult::None, true)
        } else {
            (ActResult::None, false)
        }
    }

    // Moves to the specified coordinate unconditionally.
    pub fn move_to(&mut self, dungeon: &mut Dungeon, new_coord: Coord) {
        assert!(
            dungeon[new_coord].actor.is_none(),
            format!("Moving to an occupied tile at {}", new_coord)
        );

        dungeon.move_actor(self.coord(), new_coord);
    }
}

impl Draw for Actor {
    fn draw_c(&self) -> char {
        self.inner.lock().unwrap().c
    }
    fn draw_color(&self) -> Color {
        self.inner.lock().unwrap().color
    }
}

// Traits for priority queue

impl Eq for Actor {}
impl PartialEq for Actor {
    /// Returns a1 == a2 iff their `turn` values are equal.
    fn eq(&self, other: &Self) -> bool {
        self.turn() == other.turn()
    }
}

impl Ord for Actor {
    /// Compares actors by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        // Since we're comparing floating values here, we have to use partial_cmp.
        // We should never do an invalid comparison here, so this is okay.
        other.turn().partial_cmp(&self.turn()).unwrap()
    }
}
impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Enum listing possible AI states of an actor.
#[derive(Clone, Copy, Debug)]
pub enum Behavior {
    /// Behavior corresponding to the player itself.
    Player,
    Friendly,
    Wary,
    Defensive,
    Hostile,
    Hunting,
}

impl FromStr for Behavior {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Behavior::*;

        Ok(match s {
            "player" => Player,
            "friendly" => Friendly,
            "wary" => Wary,
            "defensive" => Defensive,
            "hostile" => Hostile,
            "hunting" => Hunting,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid behavior value",
                });
            }
        })
    }
}
