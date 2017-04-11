//! Taurus - main.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

extern crate tcod;
extern crate taurus;

mod constants;
mod console;
mod database;
mod tile;
mod actor;
mod object;
mod item;
mod dungeon;
mod game;

use taurus::lang;
use taurus::coord;
use dungeon::*;
use game::Game;

fn main() {
    let mut game = Game::new();

    // TODO: move the following to game.run()

    let mut dungeon = Dungeon::new();

    // Main game loop
    match dungeon.run_loop(&mut game) {
        LoopResult::PlayerKilled => {}
        LoopResult::NoActors => {}
        LoopResult::None => {}
    }

    for _ in 1..100 {
        println!("{}", lang::name_gen(constants::MAX_NAME_LEN));
    }
}
