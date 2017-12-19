use GameResult;
use dungeon::Dungeon;
use game_data::GameData;
use generate;
use std::io;

pub fn setup_dungeon() -> GameResult<Dungeon> {
    let mut game_data = setup_game_data()?;
    let profile = game_data.database().get_obj("dungeon_profiles")?.get_obj(
        "test",
    )?;

    let dungeon = Dungeon::new(&mut game_data, 0, &profile)?;

    Ok(dungeon)
}

pub fn setup_game_data() -> GameResult<GameData> {
    GameData::new()
}
