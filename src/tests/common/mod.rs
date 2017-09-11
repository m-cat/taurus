use database::Database;
use dungeon::Dungeon;
use game::Game;

pub fn setup_game_test() -> (Game, Dungeon) {
    let mut game = Game::new();
    let dungeon = Dungeon::new(0);

    (game, dungeon)
}
