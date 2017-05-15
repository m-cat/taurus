#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::ops::IndexMut;

use GameLoopResult;
use fraction::Fraction;
use coord::Coord;
use tile::Tile;
use actor::*;
use player;
use object::Object;
use item::Item;
use item::ItemStack;
use game::Game;
use console::GameConsole;

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

    /// Returns the width of the tile grid.
    pub fn width(&self) -> usize {
        self.tile_grid.len()
    }

    /// Returns the height of the tile grid.
    pub fn height(&self) -> usize {
        self.tile_grid[0].len()
    }

    /// Returns the number of actors in the dungeon.
    ///
    /// # Panics
    /// If the map size doesn't equal the queue size.
    pub fn num_actors(&self) -> usize {
        debug_assert_eq!(self.actor_queue.len(), self.actor_map.len());

        self.actor_queue.len()
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
    ///
    /// # Panics
    /// Panics if the actor's coordinates are unavailable.
    pub fn add_actor(&mut self, x: usize, y: usize, mut a: Actor) {
        let xy = Coord::new(x, y);
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

    /// Gets an immutable reference to an actor.
    pub fn get_actor(&self, x: usize, y: usize) -> Option<&Actor> {
        self.actor_map.get(&Coord::new(x, y))
    }

    /// Gets a mutable reference to an actor.
    pub fn get_mut_actor(&mut self, x: usize, y: usize) -> Option<&mut Actor> {
        self.actor_map.get_mut(&Coord::new(x, y))
    }

    /// Sets an actor's coordinates.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    ///
    /// # Panics
    /// If the new coordinates are unavailable.
    /// If the actor could not be found at the given coordinates.
    pub fn set_actor_coord(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
        assert!(!self.actor_map.contains_key(&Coord::new(new_x, new_y)));

        let (mut actor_list, option) = self.unroll_queue_get_actor(x, y);
        let mut actor = option.expect("Dungeon::set_actor_coord failed: could not find actor.");

        actor.set_coord(new_x, new_y);
        actor_list.push(actor);

        self.rebuild_queue(actor_list);
    }

    /// Sets an actor's turn.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    ///
    /// # Panics
    /// If the actor could not be found at the given coordinates.
    pub fn set_actor_turn(&mut self, x: usize, y: usize, new_turn: Fraction) {
        let (mut actor_list, option) = self.unroll_queue_get_actor(x, y);
        let mut actor = option.expect("Dungeon::set_actor_coord failed: could not find actor.");

        actor.set_turn(new_turn);
        actor_list.push(actor);

        self.rebuild_queue(actor_list);
    }

    /// Unrolls the actor queue looking for a specific actor.
    fn unroll_queue_get_actor(&mut self, x: usize, y: usize) -> (Vec<Actor>, Option<Actor>) {
        let mut coordt_list: Vec<CoordTurn> = Vec::new();
        let mut actor_list: Vec<Actor> = Vec::new();
        let mut option = None;

        for coordt in self.actor_queue.drain() {
            coordt_list.push(coordt);
        }

        for coordt in coordt_list {
            let actor_temp = self.remove_actor(coordt.xy.x, coordt.xy.y);

            if actor_temp.xy == Coord::new(x, y) {
                option = Some(actor_temp);
            } else {
                actor_list.push(actor_temp);
            }
        }

        (actor_list, option)
    }

    /// Builds the actor queue from a list of actors.
    fn rebuild_queue(&mut self, actor_list: Vec<Actor>) {
        for actor in actor_list {
            self.add_actor(actor.xy.x, actor.xy.y, actor);
        }
    }

    /// Effectively removes an actor by taking it out of the actor map.
    /// The priority queue will know it's gone when it gets to it.
    /// Passes in the actor's coordinates to find it.
    pub fn remove_actor(&mut self, x: usize, y: usize) -> Actor {
        self.actor_map
            .remove(&Coord::new(x, y))
            .expect("Dungeon::remove_actor failed: invalid coordinate.")
    }

    /// Inserts an object into the object hash map.
    ///
    /// # Panics
    /// Panics if the tile already contains an object.
    pub fn add_object(&mut self, x: usize, y: usize, o: Object) {
        let xy = Coord::new(x, y);
        debug_assert!(!self.object_map.contains_key(&xy));
        self.object_map.insert(xy, o);
    }

    /// Removes an object from the map
    pub fn remove_object(&mut self, x: usize, y: usize) -> Object {
        self.object_map
            .remove(&Coord::new(x, y))
            .expect("Dungeon::remove_object failed, invalid coordinate")
    }

    /// Inserts an item into the stack hash map.
    pub fn add_item(&mut self, x: usize, y: usize, i: Item) {
        let xy = Coord::new(x, y);
        let mut stack = match self.stack_map.remove(&xy) {
            Some(s) => s,
            None => ItemStack::new(), // create new stack if one doesn't exist
        };

        stack.add(i);
        self.stack_map.insert(xy, stack);
    }

    /// Removes an item with given index from the stack.
    ///
    /// # Panics
    /// Panics if the passed in index is invalid.
    pub fn remove_item(&mut self, x: usize, y: usize, index: usize) -> Item {
        let mut stack = self.stack_map
            .get_mut(&Coord::new(x, y))
            .expect("Dungeon::remove_item failed, invalid coordinate");

        stack.remove(index)
    }

    /// Returns the amount of items in a stack.
    pub fn stack_size(&self, x: usize, y: usize) -> usize {
        match self.stack_map.get(&Coord::new(x, y)) {
            Some(s) => s.len(),
            None => 0,
        }
    }

    /// Runs the main game loop by iterating over the actor priority queue
    pub fn run_loop(&mut self, game: &Game, console: &mut GameConsole) -> GameLoopResult {
        loop {
            // Get the coordinate of the next actor to move
            let mut coordt = match self.actor_queue.pop() {
                Some(coordt) => coordt,
                None => return GameLoopResult::NoActors, // bad!
            };

            // If there is no actor at the coordinates or the id doesn't match,
            // this actor has been removed and we simply continue without reinserting
            // it into the queue.
            let mut a = match self.actor_map.remove(&coordt.xy) {
                Some(a) => {
                    if a.id != coordt.id {
                        continue;
                    }
                    a
                }
                None => continue,
            };

            // Update the global game turn.
            game.set_turn(a.turn);

            // Let the actor do its thing.
            let result = match a.behavior {
                Behavior::Player => player::player_act(&mut a, game, self, console),
                _ => a.act(game, self),
            };
            a.update_turn();

            match result {
                ActResult::WindowClosed => return GameLoopResult::WindowClosed,
                ActResult::None => {}
            };

            // Push the actor's associated CoordTurn back on the queue.
            coordt.xy = a.xy;
            coordt.turn = a.turn;
            self.actor_queue.push(coordt);
        }
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
