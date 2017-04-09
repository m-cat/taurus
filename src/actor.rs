//! Taurus - actor.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use std::cmp::Ordering;

use taurus::utility::{uint, int};
use coord::Coord;

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

impl Ord for Actor {
    fn cmp(&self, other: &Actor) -> Ordering {

    }
}
