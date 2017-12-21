//! Dungeon object.

use {GameLoopOutcome, GameResult};
use actor::*;
use console::DrawConsole;
use coord::Coord;
use database::Database;
use defs::{GameRatio, gameratio_max};
use error::GameError;
use failure::ResultExt;
use game_data::GameData;
use generate::{gen_dungeon, gen_dungeon_list};
use item::{Item, ItemStack};
use object::Object;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::rc::Rc;
use std::str::FromStr;
use tile::Tile;
use util::rand::Choose;

/// Struct containing a single depth of the dungeon.
/// This struct is also responsible for running the actor priority queue.
pub struct Dungeon {
    game_data: GameData,

    pub danger_level: u32,
    pub dungeon_type: DungeonType,

    tile_grid: Vec<Vec<Tile>>, // indexed x,y
    width: usize,
    height: usize,

    // Not serialized.
    actor_queue: BinaryHeap<Actor>,
    object_queue: BinaryHeap<Object>,
}

impl Dungeon {
    pub fn new(
        game_data: &GameData,
        danger_level: u32,
        profile_data: &Database,
    ) -> GameResult<Dungeon> {
        let dungeon_type = DungeonType::from_str(&profile_data.get_str("type")?)?;

        let mut dungeon = Dungeon {
            game_data: game_data.clone(),

            danger_level,
            dungeon_type,

            tile_grid: Vec::with_capacity(0),
            width: 0,
            height: 0,

            actor_queue: BinaryHeap::new(),
            object_queue: BinaryHeap::new(),
        };

        gen_dungeon(&mut dungeon, &profile_data)?;

        Ok(dungeon)
    }

    pub fn init_grid(
        &mut self,
        width: usize,
        height: usize,
        tile_data: &Database,
    ) -> GameResult<()> {
        self.width = width;
        self.height = height;
        self.tile_grid = vec![
            vec![
                Tile::new(self.game_data(), tile_data).context(format!(
            "Could not load tile:\n{}",
            tile_data
        ))?;
                height
            ];
            width
        ];

        Ok(())
    }

    /// Returns a reference to the game data.
    pub fn game_data(&self) -> &GameData {
        &self.game_data
    }

    /// Returns the width of the tile grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the tile grid.
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }

    /// Returns the number of actors in the dungeon.
    pub fn num_actors(&self) -> usize {
        self.actor_queue.len()
    }

    /// Adds actor to both the tile grid and the priority queue.
    pub fn add_actor(&mut self, actor: Actor) {
        let coord = actor.coord();
        debug_assert!(self[coord].actor.is_none()); // Actors can't share tiles.

        self.actor_queue.push(actor.clone()); // Add actor to queue.
        self[coord].actor = Some(actor); // Add actor to grid.
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
    pub fn add_object(&mut self, object: Object) {
        let coord = object.coord();
        debug_assert!(self[coord].object.is_none()); // Objects can't share tiles.

        self.object_queue.push(object.clone());
        self[coord].object = Some(object);
    }

    pub fn move_object(&mut self, old_coord: Coord, new_coord: Coord) {
        let mut object = self[old_coord].object.take().unwrap();
        object.set_coord(new_coord);

        assert!(self[new_coord].object.is_none());
        self[new_coord].object = Some(object);
    }

    /// Removes an object from the tile grid.
    pub fn remove_object(&mut self, coord: Coord) -> Object {
        self[coord].object.take().unwrap()
    }

    /// Returns the amount of stacks in a stash.
    pub fn stash_size(&self, coord: Coord) -> usize {
        match self[coord].item_stash {
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
    // TODO: Avoid picking staircases.
    // TODO: Fail after some number of tries.
    pub fn random_open_coord_actor(&self) -> Option<Coord> {
        let mut available = false;
        let mut coord: Coord = Default::default();

        while !available {
            coord = match self.random_coord() {
                Some(t) => t,
                None => return None,
            };
            let tile = &self[coord];
            // Don't put any actors on top of objects.
            // Should use some other function for that.
            available = tile.actor.is_none() && tile.object.is_none() && tile.passable();
        }

        Some(coord)
    }

    /// Runs the main game loop by iterating over the actor/object priority queues.
    pub fn run_loop(&mut self) -> GameLoopOutcome {
        let mut actor_turn = None;
        let mut object_turn = None;

        loop {
            match self.step_turn(&mut actor_turn, &mut object_turn) {
                GameLoopOutcome::None => (), // Continue game loop.
                outcome => return outcome,
            }
        }
    }

    /// Do a single iteration over either the actor or the object priority queue.
    pub fn step_turn(
        &mut self,
        actor_turn: &mut Option<GameRatio>,
        object_turn: &mut Option<GameRatio>,
    ) -> GameLoopOutcome {
        // Determine whether an actor or an object is about to move.
        if actor_turn.is_none() {
            *actor_turn = Some(match self.actor_queue.peek() {
                Some(ref actor) => actor.turn(),
                None => return GameLoopOutcome::NoActors,
            })
        }
        if object_turn.is_none() {
            *object_turn = Some(match self.object_queue.peek() {
                Some(ref object) => object.turn(),
                None => gameratio_max(),
            })
        }

        if actor_turn.unwrap() <= object_turn.unwrap() {
            // Actor acting.

            // Get the next actor.
            let mut actor = match self.actor_queue.pop() {
                Some(a) => a,
                None => return GameLoopOutcome::NoActors,
            };

            // Update the global game turn.
            self.game_data().set_turn(actor_turn.unwrap());

            match actor.act(self) {
                ActResult::WindowClosed => return GameLoopOutcome::WindowClosed,
                ActResult::QuitGame => return GameLoopOutcome::QuitGame,
                ActResult::None => (),
            };

            actor.update_turn();
            *actor_turn = None;

            // Push the actor back on the queue.
            self.actor_queue.push(actor);
        } else {
            // Object acting.

            let mut object = match self.object_queue.pop() {
                Some(o) => o,
                None => panic!("Logic error"), // No objects, yet this branch was selected.
            };

            // Update the global game turn.
            self.game_data().set_turn(object_turn.unwrap());

            match object.act(self) {
                _ => (),
            };

            object.update_turn();
            *object_turn = None;

            // Push the object back on the queue.
            self.object_queue.push(object);
        }

        GameLoopOutcome::None
    }

    pub fn peek_actor(&self) -> Actor {
        self.actor_queue.peek().unwrap().clone()
    }

    pub fn peek_object(&self) -> Object {
        self.object_queue.peek().unwrap().clone()
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
impl<'a> Index<&'a Coord> for Dungeon {
    type Output = Tile;

    fn index(&self, coord: &Coord) -> &Tile {
        &self[coord.x as usize][coord.y as usize]
    }
}

impl IndexMut<Coord> for Dungeon {
    fn index_mut(&mut self, coord: Coord) -> &mut Tile {
        &mut self[coord.x as usize][coord.y as usize]
    }
}

impl<'a> IndexMut<&'a Coord> for Dungeon {
    fn index_mut(&mut self, coord: &Coord) -> &mut Tile {
        &mut self[coord.x as usize][coord.y as usize]
    }
}

/// List of dungeons.
pub struct DungeonList {
    dungeon_list: Vec<Dungeon>,
    pub current_depth: usize,
}

impl DungeonList {
    /// Creates a new `DungeonList` with `n` dungeons.
    pub fn new(num_dungeons: usize, game_data: GameData) -> GameResult<DungeonList> {
        let mut dungeon_list = DungeonList {
            dungeon_list: Vec::with_capacity(num_dungeons),
            current_depth: 0,
        };

        gen_dungeon_list(&mut dungeon_list, game_data, num_dungeons)?;

        Ok(dungeon_list)
    }

    /// Returns a reference to the game data.
    pub fn game_data(&self) -> &GameData {
        self.dungeon_list[0].game_data()
    }

    /// Returns a mutable reference to the current dungeon.
    pub fn current_dungeon(&mut self) -> &mut Dungeon {
        let index = self.current_depth;
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

pub enum DungeonType {
    Room,
    /// Used in tests.
    Empty,
}

impl FromStr for DungeonType {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::DungeonType::*;

        Ok(match s {
            "room" => Room,
            "empty" => Empty,
            _ => {
                return Err(GameError::ConversionError {
                    val: s.into(),
                    msg: "Invalid dungeon type",
                })
            }
        })
    }
}

/// Enum of possible results of an action.
#[derive(Debug, PartialEq)]
pub enum ActResult {
    WindowClosed,
    QuitGame,
    None,
}
