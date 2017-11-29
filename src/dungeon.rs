//! Dungeon object.

use GameLoopOutcome;
use actor::*;
use console::Console;
use coord::Coord;
use defs::GameRatio;
use game_data::GameData;
use item::{Item, ItemStack};
use object::Object;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::rc::Rc;
use tile::Tile;
use util::rand::Choose;

/// Struct containing a single depth of the dungeon.
/// This struct is also responsible for running the actor priority queue.
pub struct Dungeon {
    depth: usize,
    game_data: GameData,

    tile_grid: Vec<Vec<Tile>>, // indexed x,y
    width: usize,
    height: usize,

    actor_queue: BinaryHeap<Actor>,
}

impl Dungeon {
    pub fn new(depth: usize, game_data: GameData) -> Dungeon {
        Dungeon {
            depth: depth,
            game_data: game_data,

            tile_grid: Vec::with_capacity(0),
            width: 0,
            height: 0,

            actor_queue: BinaryHeap::new(),
        }
    }

    /// Returns the depth of the dungeon.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns a reference to the game data.
    pub fn game_data(&self) -> &GameData {
        &self.game_data
    }

    /// Returns a mutable reference to the game data.
    pub fn mut_game_data(&mut self) -> &mut GameData {
        &mut self.game_data
    }

    /// Returns the width of the tile grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the tile grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of actors in the dungeon.
    pub fn num_actors(&self) -> usize {
        self.actor_queue.len()
    }

    /// Adds actor to both the tile grid and the priority queue.
    pub fn add_actor(&mut self, a: Actor) {
        let coord = a.coord();
        debug_assert!(self[coord].actor.is_none()); // Actors can't share tiles.

        self.actor_queue.push(a.clone()); // Add actor to queue.
        self[coord].actor = Some(a); // Add actor to grid.
    }

    pub fn move_actor(&mut self, old_coord: Coord, new_coord: Coord) {
        let mut actor = self[old_coord].actor.take().unwrap();
        actor.set_coord(new_coord);
        assert!(self[new_coord].actor.is_none());
        self[new_coord].actor = Some(actor);
    }

    /// Removes an actor by taking it out of the tile grid and priority queue.
    pub fn remove_actor(&mut self, coord: Coord) -> Actor {
        let mut actor_list = Vec::new();

        while let Some(a) = self.actor_queue.pop() {
            if a.coord() == coord {
                break;
            }
            actor_list.push(a);
        }
        for actor in actor_list {
            self.actor_queue.push(actor);
        }

        self[coord].actor.take().unwrap()
    }

    /// Inserts an object into the tile grid.
    pub fn add_object(&mut self, object: Box<Object>) {
        let coord = object.coord();
        debug_assert!(self[coord].object.is_none());

        self[coord].object = Some(object);
    }

    /// Removes an object from the tile grid.
    pub fn remove_object(&mut self, coord: Coord) -> Option<Box<Object>> {
        self[coord].object.take()
    }

    /// Returns the amount of stashes in a stack.
    pub fn stack_size(&self, coord: Coord) -> usize {
        match self[coord].stack {
            Some(ref s) => s.len(),
            None => 0,
        }
    }

    /// Returns a random coordinate.
    pub fn random_coord(&self) -> Option<Coord> {
        let grid = &self.tile_grid;
        let (x, column) = match grid.choose_enumerate() {
            Some(t) => t,
            None => return None,
        };
        let y = match column.choose_index() {
            Some(t) => t,
            None => return None,
        };
        Some(Coord::new(x as i32, y as i32))
    }

    /// Returns a random available coordinate, not currently occupied by any actors.
    // TODO: Check for impassable tiles and avoid picking staircases.
    pub fn random_open_coord_actor(&self) -> Option<Coord> {
        let mut occupied = true;
        let mut coord: Coord = Default::default();

        while occupied {
            coord = match self.random_coord() {
                Some(t) => t,
                None => return None,
            };
            occupied = self[coord].actor.is_some();
        }

        Some(coord)
    }

    /// Runs the main game loop by iterating over the actor priority queue
    pub fn run_loop(&mut self) -> GameLoopOutcome {
        loop {
            if let Some(outcome) = self.step_turn() {
                return outcome;
            }
        }
    }

    pub fn step_turn(&mut self) -> Option<GameLoopOutcome> {
        // Get the coordinate of the next actor to move.
        let mut a = match self.actor_queue.pop() {
            Some(a) => a,
            None => return Some(GameLoopOutcome::NoActors), // bad!
        };

        // Update the global game turn.
        self.game_data().set_turn(a.turn());

        {
            match a.act(self) {
                ActResult::WindowClosed => return Some(GameLoopOutcome::WindowClosed),
                ActResult::QuitGame => return Some(GameLoopOutcome::QuitGame),
                ActResult::None => {}
            };

            a.update_turn();
        }

        // Push the actor back on the queue.
        self.actor_queue.push(a);

        None
    }

    pub fn next_actor(&self) -> Actor {
        self.actor_queue.peek().unwrap().clone()
    }
}

/// Makes the dungeon indexable like an array.
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

/// Makes the dungeon indexable by coord.
impl Index<Coord> for Dungeon {
    type Output = Tile;

    fn index(&self, coord: Coord) -> &Tile {
        &self[coord.x as usize][coord.y as usize]
    }
}
impl IndexMut<Coord> for Dungeon {
    fn index_mut(&mut self, coord: Coord) -> &mut Tile {
        &mut self[coord.x as usize][coord.y as usize]
    }
}

/// List of dungeons.
pub struct DungeonList {
    dungeon_list: Vec<Dungeon>,
}

impl DungeonList {
    /// Creates a new `DungeonList` with `n` dungeons.
    pub fn new(num_dungeons: usize, game_data: GameData) -> DungeonList {
        let mut dungeon_list = DungeonList { dungeon_list: Vec::new() };

        let game_data = game_data;

        for n in 0..num_dungeons {
            dungeon_list.push(Dungeon::new(n, game_data.clone()));
        }

        dungeon_list
    }

    /// Returns a reference to the game data.
    pub fn game_data(&self) -> &GameData {
        self.dungeon_list[0].game_data()
    }

    /// Returns a mutable reference to the game data.
    pub fn mut_game_data(&mut self) -> &mut GameData {
        self.dungeon_list[0].mut_game_data()
    }

    /// Returns a mutable reference to the current dungeon.
    pub fn current_dungeon(&mut self) -> &mut Dungeon {
        let index = self.game_data().player_depth();
        &mut self.dungeon_list[index]
    }
}

impl Deref for DungeonList {
    type Target = Vec<Dungeon>;

    fn deref(&self) -> &Self::Target {
        &self.dungeon_list
    }
}

impl DerefMut for DungeonList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dungeon_list
    }
}
