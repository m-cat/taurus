//! Game actors.

use {GameError, GameResult};
use console::{Color, Console};
use coord::Coord;
use defs::{GameRatio, big_to_u32, to_gameratio};
use defs::{int, uint};
use dungeon::Dungeon;
use game_data::GameData;
use player;
use std::cmp::Ordering;
use std::str::FromStr;
use ui::Draw;
use util::direction::CompassDirection;

/// Actor object.
///
/// An `Actor` is any entity which could conceivably "act", that is, have a turn. Only one `Actor`
/// can occupy a tile at a time. For things like doors and traps, we have a separate struct named
/// `Object`. An `Object` can share a tile with an `Actor`.
pub struct Actor {
    // Generic name.
    name: String,
    // Unique id for this instance.
    id: uint,
    // Character to draw to the console with.
    c: char,
    // Color to draw with.
    color: Color,

    // Coordinate location in level.
    coord: Coord,
    // Current turn.
    turn: GameRatio,

    // STATS
    hp_cur: int, // int, because this value can be negative!
    hp_max: uint,
    speed: GameRatio,

    // COMBAT STATE

    // AI ATTRIBUTES
    behavior: Behavior,
}

impl Actor {
    /// Creates a new actor at the given coordinates.
    pub fn new(game_data: &mut GameData, coord: Coord, name: &str) -> GameResult<Actor> {
        let data = game_data.database().get_obj("actors")?.get_obj(name)?;

        // Load all data from the database.

        let name = String::from(name);
        let id = game_data.actor_id();
        let c = data.get_char("c")?;
        let color = Color::from_str(&data.get_str("color")?)?;
        let turn = game_data.turn();

        let hp = big_to_u32(data.get_int("hp")?)?;
        let hp_cur = hp as int;
        let hp_max = hp as uint;
        let speed = to_gameratio(data.get_frac("speed")?)?;

        let behavior = Behavior::from_str(&data.get_str("behavior")?)?;

        let mut actor = Actor {
            name,
            id,
            c,
            color,
            coord,
            turn, // We update this later in this function.

            hp_cur,
            hp_max,
            speed,

            behavior,
        };
        actor.update_turn(); // Set the actor's turn.

        Ok(actor)
    }

    /// Inserts the actor into the given dungeon.
    pub fn insert(a: Actor, dungeon: &mut Dungeon) {
        dungeon.add_actor(a);
    }

    pub fn insert_new(dungeon: &mut Dungeon, coord: Coord, name: &str) -> GameResult<()> {
        Self::insert(Self::new(dungeon.mut_game_data(), coord, name)?, dungeon);
        Ok(())
    }

    /// Returns the actor id for this actor.
    pub fn id(&self) -> uint {
        self.id
    }

    /// Returns the character this actor is drawn with.
    pub fn c(&self) -> char {
        self.c
    }

    /// Returns the color this actor is drawn with.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the name associated with this actor.
    pub fn name(&self) -> &str {
        // TODO
        "test"
    }

    /// Generates and returns the description of this actor.
    pub fn description(&self) -> String {
        // TODO
        "test".to_string()
    }

    /// Returns this actor's base speed.
    pub fn speed(&self) -> GameRatio {
        self.speed
    }

    /// Returns this actor's coordinates.
    pub fn coord(&self) -> Coord {
        self.coord
    }

    /// Sets this actor's coordinates.
    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord;
    }

    /// Returns this actor's next turn value.
    pub fn turn(&self) -> GameRatio {
        self.turn
    }

    /// Sets this actor's turn to a new value.
    pub fn set_turn(&mut self, turn: GameRatio) {
        self.turn = turn;
    }

    /// Updates this actor's turn.
    pub fn update_turn(&mut self) {
        self.turn = self.turn + self.speed();
    }

    /// Returns this actor's behavior value.
    pub fn behavior(&self) -> Behavior {
        self.behavior
    }

    /// Acts out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    pub fn act(&mut self, dungeon: &mut Dungeon, console: &mut Console) -> ActResult {
        match self.behavior {
            Behavior::Player => player::player_act(self, dungeon, console),
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

    // Tries to move to the specified coordinate.
    fn try_move_to(&mut self, dungeon: &mut Dungeon, coord: Coord) -> (ActResult, bool) {
        unimplemented!()
    }

    // Moves to the specified coordinate unconditionally.
    // Note: This function should remain private.
    fn move_to(&mut self, dungeon: &mut Dungeon, coord: Coord) {
        self.set_coord(coord);
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
#[derive(Clone, Copy)]
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
pub enum ActResult {
    WindowClosed,
    QuitGame,
    None,
}

/// A struct that combines Coord + turn.
/// Why? So we can use coords instead of actors in the priority queue and sort the queue by turn.
/// This allows the actor map to fully own the actors. We always get actors from one place,
/// the actor map, keyed by Coord.
pub struct CoordTurn {
    pub coord: Coord,
    pub turn: GameRatio,
    /// The id of the actor.
    pub id: uint,
}

// Traits for priority queue

impl Eq for CoordTurn {}
impl PartialEq for CoordTurn {
    /// Returns a1 == a2 iff their `turn` values are equal.
    fn eq(&self, other: &CoordTurn) -> bool {
        self.turn == other.turn
    }
}

impl Ord for CoordTurn {
    /// Compares CoordTurns (and actors by proxy) by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &CoordTurn) -> Ordering {
        // Since we're comparing floating values here, we have to use partial_cmp.
        // We should never do an invalid comparison here, so this is okay
        other.turn.partial_cmp(&self.turn).unwrap()
    }
}
impl PartialOrd for CoordTurn {
    fn partial_cmp(&self, other: &CoordTurn) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
