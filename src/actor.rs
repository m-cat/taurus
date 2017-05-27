#![allow(dead_code)]

use std::cmp::Ordering;
use fraction::Fraction;

use util::{uint, int};
use coord::Coord;
use dungeon::Dungeon;
use game::Game;

/// An actor is any entity which could conceivably `act`, that is, have a turn.
/// Only one actor can occupy a tile at a time.
/// For things like doors and traps, we have a separate struct named Object.
/// An object can share a tile with an actor.
pub struct Actor {
    /// Unique id for this instance.
    pub id: uint,
    // kind: ActorEnum,
    /// Character to draw to the console with.
    c: char,
    /// Coordinate location in level.
    pub xy: Coord,
    /// Current turn.
    pub turn: Fraction,

    // STATS
    hp_cur: int, // int, because this value can be negative!
    hp_max: uint,
    speed: Fraction,

    // COMBAT STATE

    // AI ATTRIBUTES
    pub behavior: Behavior,
}

impl Actor {
    pub fn new(game: &Game, name: &str) -> Actor {
        let database = game.database.get("Actor").get(name);

        let hp = database.get("hp").get_uint();

        let mut a = Actor {
            id: game.actor_id(),
            c: database.get("c").get_char(),
            xy: Coord::new(0, 0),
            turn: game.turn(), // we update this later in this function

            hp_cur: hp as int,
            hp_max: hp,
            speed: Fraction::from(1),

            behavior: Behavior::Hostile, // the default is a hostile monster!
        };

        a.turn += a.speed(); // Set the actor's turn.

        a
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
    pub fn speed(&self) -> Fraction {
        self.speed
    }

    /// Sets this actor's coordinates.
    pub fn set_coord(&mut self, x: usize, y: usize) {
        self.xy.x = x;
        self.xy.y = y;
    }

    /// Sets this actor's turn to a new value.
    pub fn set_turn(&mut self, turn: Fraction) {
        self.turn = turn;
    }

    /// Updates this actor's turn.
    pub fn update_turn(&mut self) {
        self.turn += self.speed();
    }

    /// Acts out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    /// Actor should update its own `turn` value.
    pub fn act(&mut self, game: &Game, dungeon: &mut Dungeon) -> ActResult {
        // let result = match self.behavior {
        //     Behavior::Player => player::player_act(self, game, dungeon),
        //     _ => ActResult::None,
        // };

        // match result {
        //     ActResult::None => {}
        //     _ => return result,
        // }

        ActResult::None
    }

    /// Draws the actor at given screen (not game) position.
    pub fn draw(&self, scr_x: uint, scr_y: uint) {}
}

// enum ActorEnum {
//     Player,
//     GiantRat,
// } TODO: keep this?

pub enum Behavior {
    /// Behavior corresponding to the player itself.
    Player,
    Friendly,
    Wary,
    Defensive,
    Hostile,
    Hunting,
}

pub enum ActResult {
    WindowClosed,
    None,
}

/// A struct that combines Coord + turn.
/// Why? So we can use coords instead of actors in the priority queue and sort the queue by turn.
/// This allows the actor map to fully own the actors. We always get actors from one place,
/// the actor map, keyed by Coord.
pub struct CoordTurn {
    pub xy: Coord,
    pub turn: Fraction,
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
        match other.turn.partial_cmp(&self.turn) {
            Some(order) => order,
            None => panic!("cmp failed for CoordTurn"), // TODO
        }
    }
}
impl PartialOrd for CoordTurn {
    fn partial_cmp(&self, other: &CoordTurn) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
