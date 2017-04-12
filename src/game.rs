//! Taurus - game.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use num_traits::identities::Zero;
use fraction::Fraction;

use taurus::util::uint;
use console::GameConsole;
use database::Database;
use dungeon::Dungeon;
use dungeon::LoopResult;
use generate;

/// Struct containing game-wide data such as the draw console,
/// the database, and the dungeon levels
pub struct Game {
    /// A reference to the console
    pub console: GameConsole,
    /// A reference to the main game database containing monster info, tile info, etc
    pub database: Database,

    pub dungeon_list: Vec<Dungeon>,

    /// Current depth that the player is on, indexed starting at 1
    pub depth: usize,
    /// Current global game turn
    pub turn: Fraction,

    /// Number of actors created, used for assigning unique id's
    pub num_actors: uint,
}

impl Game {
    pub fn new() -> Game {
        Game {
            console: GameConsole::init(),
            database: Database::init(),
            dungeon_list: Vec::new(),
            depth: 1,
            turn: Fraction::zero(),
            num_actors: 0,
        }
    }

    pub fn get_actor_id(&mut self) -> uint {
        self.num_actors += 1;
        self.num_actors - 1
    }

    pub fn run(&mut self) {
        generate::generate_game(self);

        let mut dungeon = match self.dungeon_list.get_mut(self.depth) {
            Some(d) => d,
            None => panic!("Game::run failed: index out of bounds"),
        };

        // Main game loop
        match dungeon.run_loop(self) {
            LoopResult::WindowClosed => {
                println!("Window closed, exiting!");
            }
            LoopResult::PlayerKilled => {}
            LoopResult::NoActors => {}
            LoopResult::None => {}
        }
    }
}
