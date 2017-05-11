extern crate rand;
extern crate num;
extern crate num_traits;
extern crate fraction;
extern crate tcod;

pub mod util;
pub mod lang;
pub mod coord;
pub mod org;
pub mod constants;
pub mod console;
pub mod database;
pub mod tile;
pub mod actor;
pub mod player;
pub mod object;
pub mod item;
pub mod dungeon;
pub mod game;
pub mod generate;

use dungeon::Dungeon;
use game::Game;
use console::GameConsole;

/// Runs the main game loop
pub fn run_game() {
    let mut console = GameConsole::init(); // initialize the console
    let mut game = Game::new();

    let mut dungeon_list: Vec<Dungeon> = Vec::new();
    generate::generate_game(&mut game, &mut dungeon_list);

    let depth = game.depth();
    let mut dungeon = dungeon_list
        .get_mut(depth)
        .expect("Game::run failed, invalid index");

    // Main game loop
    match dungeon.run_loop(&game, &mut console) {
        GameLoopResult::WindowClosed => {
            println!("Window closed, exiting!"); // TODO
        }
        GameLoopResult::PlayerDead => {} // TODO
        GameLoopResult::NoActors => {} // TODO
        GameLoopResult::None => {} // TODO
    }
}

pub enum GameLoopResult {
    /// Game window was closed by player
    WindowClosed,
    /// Player died and we need to return
    PlayerDead,
    /// No actors remaining in queue
    NoActors, // should never happen!
    /// Nothing special happened
    None,
}
