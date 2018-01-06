//! Game logic for Taurus.

// TODO: Get to the point where we can uncomment this.
// #![deny(missing_docs)]

// TODO: Get to the point where we can remove this without it causing an avalanche of warnings.
#![allow(unknown_lints)]
#![allow(dead_code, doc_markdown, unused_imports, unused_variables)]

// // Non-lexical lifetimes
// #![feature(nll)]

// Quickcheck
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

// Flame
#![cfg_attr(feature="dev", feature(plugin, custom_attribute))]
#![cfg_attr(feature="dev", plugin(flamer))]

#[cfg(feature = "dev")]
extern crate flame;

#[cfg(test)]
extern crate quickcheck;

// Required dependencies
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate num_traits;
extern crate over;
extern crate rand;
extern crate tcod;

// These modules are public so that they can be used in external integration tests.

#[macro_use]
pub mod util;

pub mod actor;
pub mod console;
pub mod coord;
pub mod database;
pub mod defs;
pub mod dungeon;
pub mod error;
pub mod game_data;
pub mod generate;
pub mod item;
pub mod material;
pub mod name_gen;
pub mod object;
pub mod player;
pub mod tile;
pub mod ui;

mod constants;
#[cfg(test)]
mod tests;

use console::DrawConsole;
use database::{Database, load_data};
use dungeon::{Dungeon, DungeonList};
use error::GameError;
use game_data::{GameData, GameLoopOutcome};
use std::sync::{Arc, Mutex, RwLock};

/// A generic result type used throughout the game.
pub type GameResult<T> = Result<T, failure::Error>;

lazy_static! {
    /// Global database reference;
    static ref DATABASE: RwLock<Database> = {
        RwLock::new(handle_error(dev_time!(load_data(), "Loading database...")))
    };

    /// Global game data struct.
    static ref GAMEDATA: RwLock<GameData> = {
        RwLock::new(handle_error(GameData::new()))
    };

    /// Drawing console.
    pub static ref CONSOLE: Mutex<DrawConsole> = {
        let game_data = GAMEDATA.read().unwrap();
        Mutex::new(dev_time!(DrawConsole::new(&game_data.console_settings()),
                             "Initializing draw console..."))
    };
}

/// Runs the main game loop.
pub fn run_game() -> GameResult<()> {
    // Load database.
    lazy_static::initialize(&DATABASE);

    // Load global game data.
    lazy_static::initialize(&GAMEDATA);

    // Initialize the console.
    lazy_static::initialize(&CONSOLE);

    #[cfg(feature = "dev")]
    for race in &["human", "elf", "dwarf", "dragon"] {
        let name_profile = DATABASE.read().unwrap().get_obj("name_profiles")?.get_obj(
            race,
        )?;

        // Display random names. TODO: remove this
        println!("{} names:", util::string::capitalize(race));
        for _ in 1..10 {
            println!("{}", name_gen::name_gen(&name_profile)?);
        }
        println!();
    }

    // Initialize a brand new game.
    let mut dungeon_list = init_new_game()?;

    loop {
        // Get the current dungeon from the list.
        let dungeon = dungeon_list.current_dungeon();
        debug_assert!(dungeon.num_actors() > 0);

        // Main game loop.
        match dungeon.run_loop() {
            GameLoopOutcome::DepthChanged => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::WindowClosed => {
                println!("\nWindow closed. Goodbye!");
                break;
            }
            GameLoopOutcome::PlayerDead => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::QuitGame => {
                println!("\nQuitting. Goodbye!");
                break;
            }
            GameLoopOutcome::NoActors => {
                unreachable!();
            }
            GameLoopOutcome::None => {
                unimplemented!(); // TODO
            }
        }
    }

    Ok(())
}

#[cfg_attr(feature = "dev", flame)]
fn init_new_game() -> GameResult<DungeonList> {
    // Generate game.
    let dungeon_list = dev_time!(DungeonList::new()?, "Generating game world...");

    Ok(dungeon_list)
}

/// Process error if one is contained in `result`.
pub fn handle_error<T>(result: Result<T, failure::Error>) -> T {
    if let Err(error) = result {
        // Handle errors.
        // Just display them for now.
        println!("------");
        println!("Error:");
        let mut i = 1;
        for cause in error.causes() {
            println!("{}{}", "  ".repeat(i), cause);
            i += 1;
        }
        println!("------");

        ::std::process::exit(1);
    }

    result.unwrap()
}
