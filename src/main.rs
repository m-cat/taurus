//! Taurus - main.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

extern crate num_traits;
extern crate fraction;
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
mod generate;

use taurus::lang;
use taurus::coord;
use game::Game;

fn main() {
    let mut game = Game::new();

    game.run();

    for _ in 1..100 {
        println!("{}", lang::name_gen(constants::MAX_NAME_LEN));
    }
}
