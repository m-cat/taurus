//! Database module.

pub use over::Obj as Database;

use GameResult;

/// Loads all data for the game.
pub fn load_data() -> GameResult<Database> {
    Ok(Database::from_file("data/game/main.over")?)
}
