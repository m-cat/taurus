use DATABASE;
use GameResult;
use dungeon::Dungeon;
use game_data::GameData;
use generate;
use std::io;

pub fn setup_dungeon() -> GameResult<Dungeon> {
    let profile = DATABASE
        .read()
        .unwrap()
        .get_obj("dungeon_profiles")?
        .get_obj("test")?;

    let dungeon = Dungeon::new(0, &profile)?;

    Ok(dungeon)
}
