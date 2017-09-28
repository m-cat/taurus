extern crate taurus;
extern crate failure;

fn main() {
    if let Err(error) = taurus::run_game() {
        // TODO: handle this better?
        println!("{}, {}", error.cause(), error.backtrace())
    }
}
