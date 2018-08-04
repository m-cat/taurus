extern crate failure;
extern crate taurus;

use taurus::handle_error;

fn main() {
    handle_error(taurus::run_game());
}
