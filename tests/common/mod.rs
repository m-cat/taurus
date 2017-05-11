use taurus::game::Game;
use taurus::dungeon::Dungeon;

pub fn setup_game_test() -> (Game, Dungeon) {
    (Game::new(), Dungeon::new(0))
}
