//! Taurus - dungeon.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::ops::IndexMut;

use taurus::coord::Coord;
use constants;
use tile::Tile;
use actor::*;
use object::Object;
use item::Item;
use item::ItemStack;
use game::Game;

/// Struct containing a single depth of the dungeon, including the depth layout.
/// This struct is also responsible for running the actor priority queue.
pub struct Dungeon {
    depth: usize,

    tile_grid: Vec<Vec<Tile>>, // indexed x,y

    actor_map: HashMap<Coord, Actor>,
    actor_queue: BinaryHeap<CoordTurn>,

    object_map: HashMap<Coord, Object>,

    stack_map: HashMap<Coord, ItemStack>,
}

impl Dungeon {
    pub fn new(depth: usize) -> Dungeon {
        Dungeon {
            depth: depth,

            tile_grid: Vec::new(), // TODO

            actor_map: HashMap::new(),
            actor_queue: BinaryHeap::new(),

            object_map: HashMap::new(),

            stack_map: HashMap::new(),
        }
    }

    /// Width of tile grid
    pub fn width(&self) -> usize {
        self.tile_grid.len()
    }

    /// Height of tile grid
    pub fn height(&self) -> usize {
        self.tile_grid[0].len()
    }

    /// Add actor to both the coordinate map and the priority queue.
    /// Asserts that the actor's coordinates are available.
    pub fn add_actor(&mut self, x: usize, y: usize, mut a: Actor) {
        let xy = Coord { x: x, y: y };
        assert!(!self.actor_map.contains_key(&xy)); // actors can't share tiles

        a.xy = xy;
        let coordt = CoordTurn {
            xy: xy,
            turn: a.turn,
            id: a.id,
        };
        self.actor_map.insert(xy, a); // add actor to map
        self.actor_queue.push(coordt); // add actor to queue
    }

    /// Get an immutable reference to an actor
    pub fn get_actor(&self, x: usize, y: usize) -> Option<&Actor> {
        self.actor_map.get(&Coord { x: x, y: y })
    }

    /// Get a mutable reference to an actor
    pub fn get_mut_actor(&mut self, x: usize, y: usize) -> Option<&mut Actor> {
        self.actor_map.get_mut(&Coord { x: x, y: y })
    }

    /// Effectively remove an actor by taking it out of the actor map.
    /// The priority queue will know it's gone when it gets to it.
    /// Pass in the actor's coordinates to find it.
    pub fn remove_actor(&mut self, x: usize, y: usize) -> Actor {
        match self.actor_map.remove(&Coord { x: x, y: y }) {
            Some(a) => a,
            None => panic!("remove_actor failed, this shouldn't happen..."), // TODO
        }
    }

    /// Insert object into the object hash map
    pub fn add_object(&mut self, x: usize, y: usize, o: Object) {
        self.object_map.insert(Coord { x: x, y: y }, o);
    }

    /// Remove object from the map
    pub fn remove_object(&mut self, x: usize, y: usize) -> Object {
        match self.object_map.remove(&Coord { x: x, y: y }) {
            Some(o) => o,
            Nome => panic!("remove_object failed, this shouldn't happen..."), // TODO
        }
    }

    /// Insert item into the stack hash map
    pub fn add_item(&mut self, x: usize, y: usize, i: Item) {
        let xy = Coord { x: x, y: y };
        let mut stack = match self.stack_map.remove(&xy) {
            Some(s) => s,
            None => ItemStack::new(), // create new stack if one doesn't exist
        };

        stack.add(i);
        self.stack_map.insert(xy, stack);
    }

    /// Remove item with given index from the stack
    /// This panics if the passed in index is invalid
    pub fn remove_item(&mut self, x: usize, y: usize, index: usize) -> Item {
        let mut stack = match self.stack_map.get_mut(&Coord { x: x, y: y }) {
            Some(s) => s,
            None => panic!("remove_item failed, invalid coord"), // TODO?
        };

        stack.remove(index)
    }

    /// Return the amount of items in a stack
    pub fn stack_size(&self, x: usize, y: usize) -> usize {
        match self.stack_map.get(&Coord { x: x, y: y }) {
            Some(s) => s.len(),
            None => 0,
        }
    }

    /// Run the main game loop by iterating over the actor priority queue
    pub fn run_loop(&mut self, game: &mut Game) -> LoopResult {
        loop {
            // Get the coordinate of the next actor to move
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

            // Update the global game turn
            game.turn = a.turn;

            match a.act(game, self) {
                ActResult::WindowClosed => return LoopResult::WindowClosed,
                ActResult::None => {}
            };

            coordt.xy = a.xy;
            coordt.turn = a.turn;
            self.actor_queue.push(coordt);
        }
        LoopResult::PlayerKilled
    }
}

impl Index<usize> for Dungeon {
    type Output = Vec<Tile>;

    fn index(&self, index: usize) -> &Vec<Tile> {
        &self.tile_grid[index]
    }
}

impl IndexMut<usize> for Dungeon {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Tile> {
        &mut self.tile_grid[index]
    }
}

pub enum LoopResult {
    /// Game window was closed by player
    WindowClosed,
    PlayerKilled,
    /// No actors remaining in queue
    NoActors, // should never happen!
    /// Nothing special happened
    None,
}
