//! Game logic for Taurus.

// TODO: Get to the point where we can uncomment this.
// #![deny(missing_docs)]

// TODO: Get to the point where we can remove this without it causing an avalanche of warnings.
#![allow(unknown_lints)]
#![allow(dead_code, doc_markdown, unused_imports, unused_variables)]

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
use dungeon::{Dungeon, DungeonList};
use error::GameError;
use game_data::{GameData, GameLoopOutcome};

/// A generic result type used throughout the game.
pub type GameResult<T> = Result<T, failure::Error>;

/// Runs the main game loop.
pub fn run_game() -> GameResult<()> {
    // Load all game data.
    let game_data = time_if_verbose!(GameData::new()?, "Loading game data...");

    let name_profile = game_data.database().get_obj("name_profiles")?.get_obj(
        "default",
    )?;

    // Display random names. TODO: remove this
    for _ in 1..10 {
        println!(
            "{}",
            name_gen::name_gen(&name_profile, constants::MAX_NAME_LEN)?
        );
    }

    // Initialize a brand new game.
    let mut dungeon_list = init_new_game(game_data)?;

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
                println!("Window closed. Goodbye!");
                return Ok(());
            }
            GameLoopOutcome::PlayerDead => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::QuitGame => {
                println!("Quitting. Goodbye!");
                return Ok(());
            }
            GameLoopOutcome::NoActors => {
                unreachable!();
            }
            GameLoopOutcome::None => {
                unimplemented!(); // TODO
            }
        }
    }
}

fn init_new_game(game_data: GameData) -> GameResult<DungeonList> {
    lazy_static::initialize(&console::CONSOLE);

    // Generate game.
    let mut dungeon_list = time_if_verbose!(
        DungeonList::new(constants::NUM_DUNGEONS, game_data)?,
        "Generating game world..."
    );

    Ok(dungeon_list)
}
