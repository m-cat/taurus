//! Taurus - generate.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>
//!
//! Module containing dungeon generation algorithms

use taurus::util::*;
use constants;
use actor::Actor;
use dungeon::Dungeon;
use game::Game;

// Todo: generate the dungeon piecemeal
/// Responsible for generating the entire dungeon
pub fn generate_game(game: &mut Game) {
    for n in 0..constants::NUM_DUNGEONS {
        game.dungeon_list.push(Dungeon::new(n+1));
    }

    for mut dungeon in game.dungeon_list.iter_mut() {
        generate_depth(game, &mut dungeon);
    }
}

/// Generate a single depth of the dungeon
fn generate_depth(game: &mut Game, dungeon: &mut Dungeon) {
    let a = Actor::new(game);
    add_actor_random_coord(dungeon, a);
}

/// Add the given actor to a random available tile in the dungeon
fn add_actor_random_coord(dungeon: &mut Dungeon, a: Actor) {
    let x = rand_range(0, dungeon.width()-1);
    let y = rand_range(0, dungeon.height()-1);
    dungeon.add_actor(x, y, a);
}
