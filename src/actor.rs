//! Taurus - actor.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use std::cmp::Ordering;

use taurus::util::{uint, int};
use coord::Coord;
use dungeon::Dungeon;
use game::Game;

/// An actor is any entity which could conceivably `act`, that is, have a turn.
/// Only one actor can occupy a tile at a time.
/// For things like doors and traps, we have a separate struct named Object.
/// An object can share a tile with an actor.
pub struct Actor {
    pub id: uint,
    // kind: ActorEnum,
    /// Character to draw with
    c: char,
    /// Coordinate location in level
    pub xy: Coord,
    /// Current turn
    pub turn: uint,

    // STATS
    hp_cur: int,
    hp_max: uint,

    // COMBAT STATE
    poison_amt: uint,

    // AI ATTRIBUTES
    aggression: AggrEnum,
}

// enum ActorEnum {
//     Player,
//     GiantRat,
// } TODO: keep this?

enum AggrEnum {
}

// Actor methods

impl Actor {
    // fn new(game: &Game, xy: Coord) -> Actor {
    //     Actor {
    //         id: game.get_actor_id(),
    //     }
    // }

    /// Act out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    /// Actor should update its own `turn` value
    pub fn act(&mut self, game: &mut Game, dungeon: &mut Dungeon) -> ActResult {
        if game.console.window_closed() {
            return ActResult::None;
        } // TODO: how does window_closed work?
        // TODO #2: return WindowClosed result

        game.console.put_char(1, 1, '@');
        game.console.wait_for_keypress(true);

        self.turn += 1;

        ActResult::None
    }

    /// Draw the actor at given screen (not game) position.
    /// Called from ui_draw
    pub fn draw(&self, scr_y: uint, scr_x: uint) {} // TODO: y and x in right order?
}

pub enum ActResult {
    None,
}

/// A struct that combines Coord + turn.
/// Why? So we can use coords instead of actors in the priority queue and sort the queue by turn.
/// This allows the actor map to fully own the actors. We always get actors from one place,
/// the actor map, keyed by Coord.
pub struct CoordTurn {
    pub xy: Coord,
    pub turn: uint,
    /// id of the actor
    pub id: uint,
}

// Traits for priority queue

impl Eq for CoordTurn {}
impl PartialEq for CoordTurn {
    /// a1 == a2 iff their `turn` values are equal
    fn eq(&self, other: &CoordTurn) -> bool {
        self.turn == other.turn
    }
}

impl Ord for CoordTurn {
    /// Compare CoordTurns (and actors by proxy) by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &CoordTurn) -> Ordering {
        other.turn.cmp(&self.turn)
    }
}
impl PartialOrd for CoordTurn {
    fn partial_cmp(&self, other: &CoordTurn) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
