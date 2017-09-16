use dungeon::Dungeon;
use game::Game;
use std::io;

pub fn setup_game_test() -> io::Result<(Game, Dungeon)> {
    let game = Game::new()?;
    let dungeon = Dungeon::new(0);

    Ok((game, dungeon))
}
