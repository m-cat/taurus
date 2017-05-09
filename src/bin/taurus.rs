extern crate taurus;

use taurus::lang;
use taurus::constants;
use taurus::game::Game;

fn main() {
    let mut game = Game::new();

    game.run();

    for _ in 1..100 {
        println!("{}", lang::name_gen(constants::MAX_NAME_LEN));
    }
}
