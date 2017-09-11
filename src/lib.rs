//! Game logic for Taurus.

// TODO: Get to the point where we can uncomment this.
// #![deny(missing_docs)]

// TODO: Get to the point where we can remove this without it causing an avalanche of warnings.
#![allow(dead_code, unused_variables)]

extern crate fraction;
extern crate num;
extern crate num_traits;
extern crate rand;
extern crate tcod;

// Game logic
// TODO: refactor this out into a different crate?

#[macro_use]
mod util;
pub mod actor;
pub mod console;
pub mod coord;
pub mod database;
pub mod dungeon;
pub mod game;
pub mod generate;
pub mod item;
pub mod object;
pub mod player;
pub mod tile;
pub mod ui;
pub mod lang;

mod data;
mod constants;

use console::GameConsole;
use dungeon::Dungeon;
use game::Game;
use game::GameLoopResult;

/// Runs the main game loop
pub fn run_game() {
    for _ in 1..100 {
        println!("{}", lang::name_gen(constants::MAX_NAME_LEN));
    }

    // Initialize a brand new game
    let (mut console, game, mut dungeon_list) = init_new_game();

    loop {
        // Get the current dungeon from the list
        let depth = game.player_depth();
        let mut dungeon = dungeon_list.get_mut(depth).expect(
            "Game::run failed, invalid index",
        );

        // Main game loop
        match dungeon.run_loop(&game, &mut console) {
            GameLoopResult::DepthChanged(depth) => {
                // TODO
            }
            GameLoopResult::WindowClosed => {
                println!("Window closed, exiting!"); // TODO
            }
            GameLoopResult::PlayerDead => {} // TODO
            GameLoopResult::NoActors => {} // TODO
            GameLoopResult::None => {} // TODO
        }
    }
}

fn init_new_game() -> (GameConsole, Game, Vec<Dungeon>) {
    let console = GameConsole::init(); // initialize the console
    let mut game = Game::new();
    let mut dungeon_list: Vec<Dungeon> = Vec::new();

    // Generate game
    generate::gen_game(&mut game, &mut dungeon_list); // TODO: add piecemeal generation

    (console, game, dungeon_list)
}
