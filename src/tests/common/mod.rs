use dungeon::Dungeon;
use game_data::GameData;
use std::io;

pub fn setup_dungeon() -> io::Result<Dungeon> {
    let game_data = setup_game_data()?;
    let dungeon = Dungeon::new(0, game_data);

    Ok(dungeon)
}

pub fn setup_game_data() -> io::Result<GameData> {
    unimplemented!()
}
