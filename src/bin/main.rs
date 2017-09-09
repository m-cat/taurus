extern crate taurus;

use taurus::constants;
use taurus::lang;

fn main() {
    taurus::run_game();

    for _ in 1..100 {
        println!("{}", lang::name_gen(constants::MAX_NAME_LEN));
    }
}
