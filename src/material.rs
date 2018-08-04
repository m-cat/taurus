//! Game materials.

use console::Color;
use database::Database;
use defs::bigr_to_f32;
use game_data::GameData;
use std::str::FromStr;
use GameResult;

#[derive(Debug)]
pub struct MaterialInfo {
    pub name: String,
    pub adjective: String,

    pub color: Color,
    pub density: f32, // g/cm^3
}

impl MaterialInfo {
    pub fn new(game_data: &GameData, material_data: &Database) -> GameResult<MaterialInfo> {
        let name = material_data.get_str("name")?;
        let adjective = material_data.get_str("adjective")?;

        let color = Color::from_str(&material_data.get_str("color")?)?;
        let density = bigr_to_f32(material_data.get_frac("density")?)?;

        Ok(MaterialInfo {
            name,
            adjective,

            color,
            density,
        })
    }
}
