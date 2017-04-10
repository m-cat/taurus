//! Taurus - actor.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use std::cmp::Ordering;

use taurus::utility::{uint, int};
use coord::Coord;
use tile::Tile;
use dungeon::Dungeon;

/// An actor is any entity which could conceivably `act`, that is, have a turn.
/// Only one actor can occupy a tile at a time.
/// For things like doors and traps, we have a separate struct named Object.
/// An object can share a tile with an actor.
pub struct Actor {
    kind: ActorEnum,
    /// Character to draw with
    c: char,
    /// Coordinate location in level
    xy: Coord,
    /// Current turn
    turn: uint,

    // STATS
    hp_cur: int,
    hp_max: uint,

    // COMBAT STATE
    poison_amt: uint,

    // AI ATTRIBUTES
    aggression: AggrEnum,
}

enum ActorEnum {
    Player,
    GiantRat,
}

enum AggrEnum {
}

// Traits for priority queue

impl Eq for Actor {}
impl PartialEq for Actor {
    /// a1 == a2 iff their `turn` values are equal
    fn eq(&self, other: &Actor) -> bool {
        self.turn == other.turn
    }
}

impl Ord for Actor {
    fn cmp(&self, other: &Actor) -> Ordering {
        other.turn.cmp(&self.turn)
    }
}
impl PartialOrd for Actor {
    fn partial_cmp(&self, other: &Actor) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Actor methods

impl Actor {
    /// Act out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    fn act(&mut self, dungeon: &mut Dungeon) {}
}
