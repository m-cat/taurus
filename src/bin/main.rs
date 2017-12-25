extern crate taurus;
extern crate failure;

use taurus::handle_error;

fn main() {
    handle_error(taurus::run_game());
}
