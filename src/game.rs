//! Taurus - game.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use taurus::util::uint;
use console::GameConsole;
use database::Database;
use dungeon::Dungeon;

/// Struct containing game-wide data such as the draw console,
/// the database, and the dungeon levels
pub struct Game {
    /// A reference to the console
    pub console: GameConsole,
    /// A reference to the main game database containing monster info, tile info, etc
    pub database: Database,

    dungeon_list: Vec<Dungeon>,

    /// Number of actors created, used for assigning unique id's
    num_actors: uint,
}

impl Game {
    pub fn new() -> Game {
        Game {
            console: GameConsole::init(),
            database: Database::init(),
            dungeon_list: Vec::new(),
            num_actors: 0,
        }
    }

    pub fn get_actor_id(&mut self) -> uint {
        self.num_actors += 1;
        self.num_actors - 1
    }
}
