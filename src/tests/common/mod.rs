use crate::dungeon::Dungeon;
use crate::game_data::GameData;
use crate::generate;
use crate::GameResult;
use crate::DATABASE;
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
