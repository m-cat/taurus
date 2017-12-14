extern crate taurus;
extern crate failure;

fn main() {
    if let Err(error) = taurus::run_game() {
        // Handle errors.
        // Just display them for now.
        println!("------");
        println!("Error:");
        let mut i = 1;
        for cause in error.causes() {
            println!("{}{}", "  ".repeat(i), cause);
            i += 1;
        }
        println!("------");
    }
}
