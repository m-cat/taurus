//! Game actors.

use {GameError, GameResult};
use console::{Color, DrawConsole};
use coord::Coord;
use database::Database;
use defs::{GameRatio, big_to_u32, to_gameratio};
use dungeon::Dungeon;
use failure::ResultExt;
use game_data::GameData;
use player;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use std::str::FromStr;
use ui::Draw;
use util::direction::CompassDirection;

#[derive(Debug)]
struct ActorInner {
    // Generic name.
    name: String,
    // Unique id for this instance.
    id: u32,
    // Character to draw to the console with.
    c: char,
    // Color to draw with.
    color: Color,

    // Coordinate location in level.
    coord: Coord,
    // Current turn.
    turn: GameRatio,

    // STATS
    hp_cur: i32, // i32 because this value can be negative!
    hp_max: u32,
    speed: GameRatio,

    // COMBAT STATE

    // AI ATTRIBUTES
    behavior: Behavior,
}

/// Actor object.
///
/// An `Actor` is any entity which could conceivably "act", that is, have a turn. Only one `Actor`
/// can occupy a tile at a time. For things like doors and traps, we have a separate struct named
/// `Object`. An `Object` can share a tile with an `Actor`.
#[derive(Clone, Debug)]
pub struct Actor {
    inner: Rc<RefCell<ActorInner>>,
}

impl Actor {
    /// Creates a new actor at the given coordinates.
    pub fn new(game_data: &mut GameData, coord: Coord, data: &Database) -> GameResult<Actor> {

        // Load all data from the database.

        let name = data.get_str("name")?;
        let id = game_data.actor_id();
        let c = data.get_char("c")?;
        let color = Color::from_str(data.get_str("color")?.as_str())?;
        let turn = game_data.turn();

        let hp = big_to_u32(data.get_int("hp")?)?;
        let hp_cur = hp as i32;
        let hp_max = hp;
        let speed = to_gameratio(data.get_frac("speed")?)?;

        let behavior = Behavior::from_str(data.get_str("behavior")?.as_str())?;

        // Create the actor instance.

        let mut actor = Actor {
            inner: Rc::new(RefCell::new(ActorInner {
                name,
                id,
                c,
                color,

                coord,
                turn, // We update this after creating the actor.

                hp_cur,
                hp_max,
                speed,

                behavior,
            })),
        };
        actor.update_turn(); // Set the actor's turn.

        Ok(actor)
    }

    pub fn insert_new(
        dungeon: &mut Dungeon,
        coord: Coord,
        actor_data: &Database,
    ) -> GameResult<()> {
        let a = Self::new(dungeon.mut_game_data(), coord, actor_data)
            .context(format!("Could not load actor:\n{}", actor_data))?;
        dungeon.add_actor(a);
        Ok(())
    }

    /// Returns the actor id for this actor.
    pub fn id(&self) -> u32 {
        self.inner.borrow().id
    }

    /// Returns the character this actor is drawn with.
    pub fn c(&self) -> char {
        self.inner.borrow().c
    }

    /// Returns the color this actor is drawn with.
    pub fn color(&self) -> Color {
        self.inner.borrow().color
    }

    /// Returns the name associated with this actor.
    pub fn name(&self) -> String {
        self.inner.borrow().name.clone()
    }

    /// Generates and returns the description of this actor.
    pub fn description(&self) -> String {
        // TODO
        "test".to_string()
    }

    /// Returns this actor's base speed.
    pub fn speed(&self) -> GameRatio {
        self.inner.borrow().speed
    }

    /// Returns this actor's coordinates.
    pub fn coord(&self) -> Coord {
        self.inner.borrow().coord
    }

    /// Sets this actor's coordinates.
    pub fn set_coord(&mut self, coord: Coord) {
        self.inner.borrow_mut().coord = coord;
    }

    /// Returns this actor's next turn value.
    pub fn turn(&self) -> GameRatio {
        self.inner.borrow().turn
    }

    /// Sets this actor's turn to a new value.
    pub fn set_turn(&mut self, turn: GameRatio) {
        self.inner.borrow_mut().turn = turn;
    }

    /// Updates this actor's turn.
    pub fn update_turn(&mut self) {
        self.inner.borrow_mut().turn = self.turn() + self.speed();
    }

    /// Returns this actor's behavior value.
    pub fn behavior(&self) -> Behavior {
        self.inner.borrow().behavior
    }

    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.inner.borrow_mut().behavior = behavior;
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
            let tile = &dungeon[coord];

            if let Some(ref _actor) = tile.actor {
                false
            } else if let Some(ref object) = tile.object {
                if !object.passable() {
                    false
                } else {
                    tile.passable()
                }
            } else {
                tile.passable()
            }
        };

        if passable {
            self.move_to(dungeon, coord);
            (ActResult::None, false)
        } else {
            (ActResult::None, false)
        }
    }

    // Moves to the specified coordinate unconditionally.
    pub fn move_to(&mut self, dungeon: &mut Dungeon, coord: Coord) {
        if dungeon[coord].actor.is_some() {
            panic!("Moving to an occupied tile");
        }

        let mut a = dungeon[self.coord()].actor.take().unwrap();
        a.set_coord(coord);
        dungeon[coord].actor = Some(a);
    }
}

impl Draw for Actor {
    fn draw_c(&self) -> char {
        self.c()
    }
    fn draw_color(&self) -> Color {
        self.color()
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
        Ok(match s {
            "player" => Behavior::Player,
            "friendly" => Behavior::Friendly,
            "wary" => Behavior::Wary,
            "defensive" => Behavior::Defensive,
            "hostile" => Behavior::Hostile,
            "hunting" => Behavior::Hunting,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid actor behavior",
                })
            }
        })
    }
}

/// Enum of possible results of an actor action.
#[derive(Debug, PartialEq)]
pub enum ActResult {
    WindowClosed,
    QuitGame,
    None,
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
    /// Compares CoordTurns (and actors by proxy) by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &Self) -> Ordering {
        // Since we're comparing floating values here, we have to use partial_cmp.
        // We should never do an invalid comparison here, so this is okay
        other.turn().partial_cmp(&self.turn()).unwrap()
    }
}
impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
