use taurus::game::Game;
use taurus::dungeon::Dungeon;
use taurus::database::Database;

pub fn setup_game_test() -> (Game, Dungeon) {
    let mut game = Game::new();
    let dungeon = Dungeon::new(0);

    (game, dungeon)
}
