use std::fs::File;
use std::path::Path;
use std::io;

use database::Database;

/// Initializes all data for the game.
pub fn init_game(database: &mut Database) -> io::Result<()> {
    init_actors(database)?;
    Ok(())
}

/// Initializes data for all actors.
fn init_actors(database: &mut Database) -> io::Result<()> {
    let mut db = database.sub("actor");

    let path = Path::new("data/database/actors.over");
    let file = File::open(&path)?;

    db.load_database(file);

    Ok(())
}
