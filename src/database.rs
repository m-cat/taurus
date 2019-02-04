//! Database module.

pub use over::arr::Arr;
pub use over::value::Value;
pub use over::Obj as Database;

use crate::GameResult;

/// Loads all data for the game.
pub fn load_data() -> GameResult<Database> {
    Ok(Database::from_file("data/game/main.over")?)
}
