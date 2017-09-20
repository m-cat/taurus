extern crate taurus;

fn main() {
    if let Err(e) = taurus::run_game() {
        panic!("{}", e); // TODO: handle this better
    }
}
