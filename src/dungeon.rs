#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::ops::IndexMut;

use GameLoopResult;
use fraction::Fraction;
use coord::Coord;
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

            tile_grid: Vec::with_capacity(0), // placeholder - allocated and filled when generating

            actor_map: HashMap::new(),
            actor_queue: BinaryHeap::new(),

            object_map: HashMap::new(),

            stack_map: HashMap::new(),
        }
    }

    /// Returns the width of tile grid
    pub fn width(&self) -> usize {
        self.tile_grid.len()
    }

    /// Returns the height of the tile grid
    pub fn height(&self) -> usize {
        self.tile_grid[0].len()
    }

    // /// Initialize the tile grid, should only be called in generation functions
    // fn init_grid() -> Vec<Vec<Tile>> {
    //     let w = constants::DUNGEON_WIDTH_DEFAULT;
    //     let h = constants::DUNGEON_HEIGHT_DEFAULT;
    //     let mut tile_grid = Vec::with_capacity(w);

    //     for j in 0..w {
    //         let mut column: Vec<Tile> = Vec::with_capacity(h);

    //         for i in 0..h {
    //             column.push(Tile::new());
    //         }
    //         tile_grid.push(column);
    //     }

    //     tile_grid
    // }


    /// Adds actor to both the coordinate map and the priority queue.
    /// Asserts that the actor's coordinates are available.
    pub fn add_actor(&mut self, x: usize, y: usize, mut a: Actor) {
        let xy = Coord { x: x, y: y };
        debug_assert!(!self.actor_map.contains_key(&xy)); // actors can't share tiles

        a.xy = xy;
        let coordt = CoordTurn {
            xy: xy,
            turn: a.turn,
            id: a.id,
        };
        self.actor_map.insert(xy, a); // add actor to map
        self.actor_queue.push(coordt); // add actor to queue
    }

    /// Gets an immutable reference to an actor
    pub fn get_actor(&self, x: usize, y: usize) -> Option<&Actor> {
        self.actor_map.get(&Coord { x: x, y: y })
    }

    /// Gets a mutable reference to an actor
    pub fn get_mut_actor(&mut self, x: usize, y: usize) -> Option<&mut Actor> {
        self.actor_map.get_mut(&Coord { x: x, y: y })
    }

    /// Modifies an actor's coordinates.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    /// Asserts that the new coordinates are available.
    pub fn update_actor_coord(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
        // TODO
    }

    /// Modifies an actor's turn.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    pub fn update_actor_turn(&mut self, x: usize, y: usize, new_turn: Fraction) {
        // TODO
    }

    /// Effectively removes an actor by taking it out of the actor map.
    /// The priority queue will know it's gone when it gets to it.
    /// Passes in the actor's coordinates to find it.
    pub fn remove_actor(&mut self, x: usize, y: usize) -> Actor {
        self.actor_map
            .remove(&Coord { x: x, y: y })
            .expect("Dungeon::remove_actor failed, invalid coordinate")
    }

    /// Inserts an object into the object hash map
    /// Asserts that the tile is free of objects
    pub fn add_object(&mut self, x: usize, y: usize, o: Object) {
        let xy = Coord { x: x, y: y };
        debug_assert!(!self.object_map.contains_key(&xy));
        self.object_map.insert(xy, o);
    }

    /// Removes an object from the map
    pub fn remove_object(&mut self, x: usize, y: usize) -> Object {
        self.object_map
            .remove(&Coord { x: x, y: y })
            .expect("Dungeon::remove_object failed, invalid coordinate")
    }

    /// Inserts an item into the stack hash map
    pub fn add_item(&mut self, x: usize, y: usize, i: Item) {
        let xy = Coord { x: x, y: y };
        let mut stack = match self.stack_map.remove(&xy) {
            Some(s) => s,
            None => ItemStack::new(), // create new stack if one doesn't exist
        };

        stack.add(i);
        self.stack_map.insert(xy, stack);
    }

    /// Removes an item with given index from the stack
    /// This panics if the passed in index is invalid
    pub fn remove_item(&mut self, x: usize, y: usize, index: usize) -> Item {
        let mut stack = self.stack_map
            .get_mut(&Coord { x: x, y: y })
            .expect("Dungeon::remove_item failed, invalid coordinate");

        stack.remove(index)
    }

    /// Returns the amount of items in a stack
    pub fn stack_size(&self, x: usize, y: usize) -> usize {
        match self.stack_map.get(&Coord { x: x, y: y }) {
            Some(s) => s.len(),
            None => 0,
        }
    }

    /// Runs the main game loop by iterating over the actor priority queue
    pub fn run_loop(&mut self, game: &Game) -> GameLoopResult {
        loop {
            // Get the coordinate of the next actor to move
            let mut coordt = match self.actor_queue.pop() {
                Some(ct) => ct,
                None => return GameLoopResult::NoActors, // bad!
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
            game.set_turn(a.turn);

            match a.act(game, self) {
                ActResult::WindowClosed => return GameLoopResult::WindowClosed,
                ActResult::None => {}
            };

            coordt.xy = a.xy;
            coordt.turn = a.turn;
            self.actor_queue.push(coordt);
        }
        GameLoopResult::PlayerDead
    }
}

/// Makes the dungeon indexable like an array
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
