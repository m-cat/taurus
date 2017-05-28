use taurus::game::Game;
use taurus::dungeon::Dungeon;
use taurus::database::Database;

pub fn setup_game_test() -> (Game, Dungeon) {
    let mut game = Game::new();
    let dungeon = Dungeon::new(0);

    setup_database_test(&mut game.database);

    (game, dungeon)
}

fn setup_database_test(database: &mut Database) {
    database.sub("Actor").sub("test").sub("hp").add_uint(0);
    database.sub("Actor").sub("test").sub("c").add_char('?');
}
