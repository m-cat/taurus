//! Game logic for Taurus.

// TODO: Get to the point where we can uncomment this.
// #![deny(missing_docs)]

// TODO: Get to the point where we can remove this without it causing an avalanche of warnings.
#![allow(dead_code, unused_variables)]

extern crate failure;
#[macro_use]
extern crate failure_derive;

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
pub mod name_gen;
pub mod object;
pub mod player;
pub mod tile;
pub mod ui;

mod constants;
#[cfg(test)]
mod tests;

use console::Console;
use dungeon::DungeonList;
use error::GameError;
use game_data::{GameData, GameLoopOutcome};

/// A generic result type used throughout the game.
pub type GameResult<T> = Result<T, failure::Error>;

/// Runs the main game loop.
pub fn run_game() -> GameResult<()> {
    let game_data = GameData::new()?;
    let profile_data = game_data.database().get_obj("name_profiles")?.get_obj(
        "default",
    )?;
    let name_profile = name_gen::NameProfile::new(&profile_data)?;

    // Display random names. TODO: remove this
    for _ in 1..100 {
        println!(
            "{}",
            name_gen::name_gen(&name_profile, constants::MAX_NAME_LEN)
        );
    }

    // Initialize a brand new game.
    let (mut console, mut dungeon_list) = init_new_game(game_data)?;

    loop {
        // Get the current dungeon from the list.
        let dungeon = dungeon_list.current_dungeon();

        // Main game loop
        match dungeon.run_loop(&mut console) {
            GameLoopOutcome::DepthChanged => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::WindowClosed => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::PlayerDead => {
                unimplemented!(); // TODO
            }
            GameLoopOutcome::QuitGame => {
                unimplemented!();
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

fn init_new_game(game_data: GameData) -> GameResult<(Console, DungeonList)> {
    // Initialize the console.
    let console = Console::init(
        constants::SCR_WIDTH,
        constants::SCR_HEIGHT,
        constants::TITLE,
        constants::FONT_DEFAULT,
        constants::FPS,
    );

    let mut dungeon_list = DungeonList::new(constants::NUM_DUNGEONS, game_data);

    // Generate game
    generate::gen_game(&mut dungeon_list)?; // TODO: add piecemeal generation
    let depth = dungeon_list.game_data().player_depth();

    Ok((console, dungeon_list))
}
