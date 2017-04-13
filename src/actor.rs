//! Taurus - actor.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use std::cmp::Ordering;
use fraction::Fraction;

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
    pub turn: Fraction,

    // STATS
    hp_cur: int, // int because this value can be negative!
    hp_max: uint,
    speed: Fraction,

    // COMBAT STATE
    poison_amt: uint,

    // AI ATTRIBUTES
    aggression: Aggression,
}

// Actor methods

impl Actor {
    pub fn new(game: &Game) -> Actor {
        let hp: uint = 0; // TODO

        let mut a = Actor {
            id: game.get_actor_id(),
            c: '@', // TODO
            xy: Coord { x: 0, y: 0 },
            turn: game.turn.get(), // we update this after the actor is created

            hp_cur: hp as int,
            hp_max: hp,
            speed: Fraction::from(1),

            poison_amt: 0,

            aggression: Aggression::Hostile, // the default is a hostile monster!
        };

        a.turn += a.speed();
        a
    }

    /// Get this actor's base speed
    pub fn speed(&self) -> Fraction {
        self.speed
    }

    fn update_turn(&mut self) {
        self.turn += self.speed();
    }

    /// Act out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    /// Actor should update its own `turn` value
    pub fn act(&mut self, game: &Game, dungeon: &mut Dungeon) -> ActResult {
        let mut console = game.console.borrow_mut();
        if console.window_closed() {
            return ActResult::WindowClosed;
        } // TODO: how does window_closed work?

        console.put_char(1, 1, '@');
        console.wait_for_keypress(true);

        self.update_turn();

        ActResult::None
    }

    /// Draw the actor at given screen (not game) position.
    /// Called from ui_draw
    pub fn draw(&self, scr_x: uint, scr_y: uint) {}
}

// enum ActorEnum {
//     Player,
//     GiantRat,
// } TODO: keep this?

enum Aggression {
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
