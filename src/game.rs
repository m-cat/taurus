//! Taurus - game.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
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
    /// A reference to the console, wrapped around Rc<Refcell<>>.
    pub console: Rc<RefCell<GameConsole>>,
    /// A reference to the main game database containing monster info, tile info, etc
    pub database: Database,

    /// Current depth that the player is on, indexed starting at 1
    pub depth: usize,
    /// Current global game turn
    pub turn: Cell<Fraction>,

    /// Number of actors created, used for assigning unique id's
    pub num_actors: Cell<uint>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            console: Rc::new(RefCell::new(GameConsole::init())), // initialize the console
            database: Database::init(), // initialize the database
            depth: 1,
            turn: Cell::new(Fraction::zero()),
            num_actors: Cell::new(0),
        }
    }

    pub fn get_actor_id(&self) -> uint {
        let n = self.num_actors.get();
        self.num_actors.set(n+1);
        n
    }

    pub fn run(&mut self) {
        let mut dungeon_list: Vec<Dungeon> = Vec::new();
        generate::generate_game(self, &mut dungeon_list);

        let depth = self.depth;
        let mut dungeon = dungeon_list.get_mut(depth)
            .expect("Game::run failed, invalid index");

        // Main game loop
        match dungeon.run_loop(self) {
            LoopResult::WindowClosed => {
                println!("Window closed, exiting!"); // TODO
            }
            LoopResult::PlayerKilled => {} // TODO
            LoopResult::NoActors => {} // TODO
            LoopResult::None => {} // TODO
        }
    }
}
