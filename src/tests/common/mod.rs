use GameResult;
use dungeon::Dungeon;
use game_data::GameData;
use generate;
use std::io;

pub fn setup_dungeon() -> GameResult<Dungeon> {
    let game_data = setup_game_data()?;
    let mut dungeon = Dungeon::new(0, game_data.clone());
    let profile = game_data.database().get_obj("dungeon_profiles")?.get_obj(
        "test",
    )?;

    generate::gen_dungeon_room_method(&mut dungeon, 0, &profile)?;

    Ok(dungeon)
}

pub fn setup_game_data() -> GameResult<GameData> {
    GameData::new()
}
