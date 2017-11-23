extern crate taurus;
extern crate failure;

fn main() {
    if let Err(error) = taurus::run_game() {
        // Handle errors.
        // Just display them for now.
        let mut fail = error.cause();
        println!("Error: {}", fail);
        while let Some(cause) = fail.cause() {
            println!("Caused by: {}", cause);
            fail = cause;
        }
    }
}
