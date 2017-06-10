use database::Database;

/// Initializes all data for the game.
pub fn init_game(database: &mut Database) {
    init_actors(database);
}

/// Initializes data for all actors.
fn init_actors(database: &mut Database) {
    database
        .sub("Actor")
        .sub("Player")
        .sub("hp")
        .add_uint(10);
    database
        .sub("Actor")
        .sub("Player")
        .sub("c")
        .add_char('@');
}
