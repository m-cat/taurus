#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::ops::Index;
use std::ops::IndexMut;

use GameLoopResult;
use util::{int, Choose};
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
    tile_grid: Option<Vec<Vec<Tile>>>, // indexed x,y

    actor_map: HashMap<Coord, Actor>,
    actor_queue: BinaryHeap<CoordTurn>,
    object_map: HashMap<Coord, Object>,
    stack_map: HashMap<Coord, ItemStack>,
}

impl Dungeon {
    pub fn new(depth: usize) -> Dungeon {
        Dungeon {
            depth: depth,
            tile_grid: None,

            actor_map: HashMap::new(),
            actor_queue: BinaryHeap::new(),
            object_map: HashMap::new(),
            stack_map: HashMap::new(),
        }
    }

    /// Returns the width of the tile grid.
    ///
    /// # Panics
    /// If the tile grid hasn't been initialized.
    pub fn width(&self) -> usize {
        self.tile_grid.as_ref().unwrap().len()
    }

    /// Returns the height of the tile grid.
    ///
    /// # Panics
    /// If the tile grid hasn't been initialized.
    pub fn height(&self) -> usize {
        let column_list = self.tile_grid.as_ref().unwrap();
        column_list[0].len()
    }

    /// Returns the number of actors in the dungeon.
    ///
    /// # Panics
    /// If the map size doesn't equal the queue size.
    pub fn num_actors(&self) -> usize {
        debug_assert_eq!(self.actor_queue.len(), self.actor_map.len());

        self.actor_queue.len()
    }

    /// Initializes the tile grid, should only be called in generation functions
    fn create_grid(&mut self, width: usize, height: usize) {
        self.tile_grid = Some(Vec::with_capacity(width));

        for _ in 0..width {
            let mut column: Vec<Tile> = Vec::with_capacity(height);

            for _ in 0..height {
                column.push(Default::default());
            }
            self.tile_grid.as_mut().unwrap().push(column);
        }
    }

    /// Adds actor to both the coordinate map and the priority queue.
    ///
    /// # Panics
    /// Panics if the actor's coordinates are unavailable.
    pub fn add_actor(&mut self, xy: Coord, mut a: Actor) {
        debug_assert!(!self.actor_map.contains_key(&xy)); // actors can't share tiles

        a.set_coord(xy);
        let coordt = CoordTurn {
            xy: xy,
            turn: a.turn,
            id: a.id,
        };
        self.actor_map.insert(xy, a); // add actor to map
        self.actor_queue.push(coordt); // add actor to queue
    }

    /// Returns true if there is an actor at `xy`.
    pub fn has_actor(&self, xy: Coord) -> bool {
        self.actor_map.contains_key(&xy)
    }

    /// Gets an immutable reference to an actor.
    pub fn get_actor(&self, xy: Coord) -> Option<&Actor> {
        self.actor_map.get(&xy)
    }

    /// Gets a mutable reference to an actor.
    pub fn get_mut_actor(&mut self, xy: Coord) -> Option<&mut Actor> {
        self.actor_map.get_mut(&xy)
    }

    /// Sets an actor's coordinates.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    ///
    /// # Panics
    /// If the new coordinates are unavailable.
    /// If the actor could not be found at the given coordinates.
    pub fn set_actor_coord(&mut self, xy: Coord, new_xy: Coord) {
        assert!(!self.actor_map.contains_key(&new_xy));

        let (mut actor_list, option) = self.unroll_queue_get_actor(xy);
        let mut actor = option.expect("Dungeon::set_actor_coord failed: could not find actor.");

        actor.set_coord(new_xy);
        actor_list.push(actor);

        self.rebuild_queue(actor_list);
    }

    /// Sets an actor's turn.
    /// Note that this is rather inefficient due to the need to rebuild the priority queue.
    ///
    /// # Panics
    /// If the actor could not be found at the given coordinates.
    pub fn set_actor_turn(&mut self, xy: Coord, new_turn: Fraction) {
        let (mut actor_list, option) = self.unroll_queue_get_actor(xy);
        let mut actor = option.expect("Dungeon::set_actor_coord failed: could not find actor.");

        actor.set_turn(new_turn);
        actor_list.push(actor);

        self.rebuild_queue(actor_list);
    }

    /// Unrolls the actor queue looking for a specific actor.
    fn unroll_queue_get_actor(&mut self, xy: Coord) -> (Vec<Actor>, Option<Actor>) {
        let mut coordt_list: Vec<CoordTurn> = Vec::new();
        let mut actor_list: Vec<Actor> = Vec::new();
        let mut option = None;

        for coordt in self.actor_queue.drain() {
            coordt_list.push(coordt);
        }

        for coordt in coordt_list {
            let actor_temp = self.remove_actor(coordt.xy);

            if actor_temp.coord() == xy {
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
            self.add_actor(actor.coord(), actor);
        }
    }

    /// Effectively removes an actor by taking it out of the actor map.
    /// The priority queue will know it's gone when it gets to it.
    /// Passes in the actor's coordinates to find it.
    pub fn remove_actor(&mut self, xy: Coord) -> Actor {
        self.actor_map
            .remove(&xy)
            .expect("Dungeon::remove_actor failed: invalid coordinate.")
    }

    /// Inserts an object into the object hash map.
    ///
    /// # Panics
    /// Panics if the tile already contains an object.
    pub fn add_object(&mut self, xy: Coord, o: Object) {
        debug_assert!(!self.object_map.contains_key(&xy));
        self.object_map.insert(xy, o);
    }

    /// Removes an object from the map
    pub fn remove_object(&mut self, xy: Coord) -> Object {
        self.object_map
            .remove(&xy)
            .expect("Dungeon::remove_object failed, invalid coordinate")
    }

    /// Inserts an item into the stack hash map.
    pub fn add_item(&mut self, xy: Coord, i: Item) {
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
    pub fn remove_item(&mut self, xy: Coord, index: usize) -> Item {
        let mut stack = self.stack_map
            .get_mut(&xy)
            .expect("Dungeon::remove_item failed, invalid coordinate");

        stack.remove(index)
    }

    /// Returns the amount of items in a stack.
    pub fn stack_size(&self, xy: Coord) -> usize {
        match self.stack_map.get(&xy) {
            Some(s) => s.len(),
            None => 0,
        }
    }

    /// Returns a random coordinate.
    pub fn random_coord(&self) -> Coord {
        let grid = self.tile_grid.as_ref().unwrap();
        let (x, column) = grid.choose_enumerate().unwrap();
        let y = column.choose_i().unwrap();
        Coord::new(x as int, y as int)
    }

    /// Returns an available coordinate, not currently occupied by any actors.
    pub fn random_avail_coord_actor(&self) -> Coord {
        let mut found = false;
        let mut xy: Coord = Default::default();

        while !found {
            xy = self.random_coord();
            found = self.has_actor(xy);
        }

        xy
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
            coordt.xy = a.coord();
            coordt.turn = a.turn;
            self.actor_queue.push(coordt);
        }
    }
}

/// Makes the dungeon indexable like an array
impl Index<usize> for Dungeon {
    type Output = Vec<Tile>;

    /// # Panics
    /// Panics if the tile grid hasn't been initialized.
    fn index(&self, index: usize) -> &Vec<Tile> {
        &self.tile_grid.as_ref().unwrap()[index]
        // let temp: Vec<Vec<Tile>> = self.tile_grid.unwrap();
        // &temp[index]
    }
}
impl IndexMut<usize> for Dungeon {
    /// # Panics
    /// Panics if the tile grid hasn't been initialized.
    fn index_mut(&mut self, index: usize) -> &mut Vec<Tile> {
        &mut self.tile_grid.as_mut().unwrap()[index]
    }
}
