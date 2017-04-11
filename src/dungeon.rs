//! Taurus - dungeon.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use std::collections::HashMap;
use std::collections::BinaryHeap;

use taurus::coord::Coord;
use tile::Tile;
use actor::*;
use object::Object;
use item::ItemStack;
use game::Game;

/// Struct containing a single depth of the dungeon, including the depth layout.
/// This struct is also responsible for running the actor priority queue.
pub struct Dungeon {
    tile_grid: Vec<Vec<Tile>>,

    actor_map: HashMap<Coord, Actor>,
    actor_queue: BinaryHeap<CoordTurn>,

    object_map: HashMap<Coord, Object>,

    stack_map: HashMap<Coord, ItemStack>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            tile_grid: Vec::new(), // TODO

            actor_map: HashMap::new(),
            actor_queue: BinaryHeap::new(),

            object_map: HashMap::new(),

            stack_map: HashMap::new(),
        }
    }

    /// Add actor to both the coordinate map and the priority queue.
    /// Asserts that the actor's coordinates are available.
    pub fn add_actor(&mut self, a: Actor) {
        assert!(!self.actor_map.contains_key(&a.xy)); // actors can't share tiles

        let coordt = CoordTurn {
            xy: a.xy,
            turn: a.turn,
            id: a.id,
        };
        self.actor_map.insert(a.xy, a);
        self.actor_queue.push(coordt);
    }

    /// Effectively remove an actor by taking it out of the actor map.
    /// The priority queue will know it's gone when it gets to it.
    /// Pass in the actor's coordinates to find it.
    pub fn remove_actor(&mut self, xy: &Coord) {
        let a = match self.actor_map.remove(xy) {
            Some(a) => a,
            None => panic!("remove_actor failed, this shouldn't happen..."), // TODO
        };
    }

    pub fn add_object(&mut self, o: Object) {
        self.object_map.insert(o.xy, o);
    }

    pub fn run_loop(&mut self, game: &mut Game) -> LoopResult {
        loop {
            let mut coordt = match self.actor_queue.pop() {
                Some(ct) => ct,
                None => return LoopResult::NoActors, // bad!
            };

            // If there is no actor at the coordinates or the id doesn't match,
            // this actor has been removed and we simply continue without reinserting
            // it into the queue
            let mut a = match self.actor_map.remove(&coordt.xy) {
                Some(a) => {
                    if a.id != coordt.id {
                        continue;
                    }
                    a
                }
                None => continue,
            };

            match a.act(game, self) {
                ActResult::None => {}
            };

            coordt.xy = a.xy;
            coordt.turn = a.turn;
            self.actor_queue.push(coordt);
        }
        LoopResult::PlayerKilled
    }
}

pub enum LoopResult {
    PlayerKilled,
    /// No actors remaining in queue
    NoActors, // should never happen!
    /// Nothing special happened
    None,
}
