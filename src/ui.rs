//! User interface module.

use GameResult;
use console::{CONSOLE, Color, DrawConsole};
use constants;
use database::Database;
use defs::big_to_usize;
use dungeon::Dungeon;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct UiSettings {
    pub game_width: usize,
    pub game_height: usize,
}

impl UiSettings {
    pub fn new(data: &Database) -> GameResult<UiSettings> {

        // Load all data from the database.

        let game_width = big_to_usize(data.get_int("game_width")?)?;
        let game_height = big_to_usize(data.get_int("game_height")?)?;

        // Create the struct.

        Ok(UiSettings {
            game_width,
            game_height,
        })
    }
}

/// Trait for any object that can be drawn.
pub trait Draw {
    fn draw_c(&self) -> char;
    fn draw_color(&self) -> Color;
}

pub fn draw_all(dungeon: &Dungeon) {
    CONSOLE.lock().unwrap().clear();

    draw_game(dungeon);

    let mut console = CONSOLE.lock().unwrap();
    let fps = console.get_fps();
    console.put_str(
        0,
        0,
        &format!("{}", fps),
        Color::from_str("#0000FF").unwrap(),
    );
    console.flush();
}

pub fn draw_game(dungeon: &Dungeon) {
    let mut console = CONSOLE.lock().unwrap();

    let settings = dungeon.game_data().ui_settings();
    let game_width = settings.game_width;
    let game_height = settings.game_height;

    let player = dungeon.game_data().player().coord();

    let width_radius = (game_width / 2) as i32;
    let height_radius = (game_height / 2) as i32;

    let view_left = player.x - width_radius;
    let view_top = player.y - height_radius;
    let view_right = player.x + width_radius;
    let view_bottom = player.y + height_radius;

    let dungeon_width = dungeon.width() as i32;
    let dungeon_height = dungeon.height() as i32;

    for x in 0.max(view_left)..dungeon_width.min(view_right + 1) {
        for y in 0.max(view_top)..dungeon_height.min(view_bottom + 1) {
            debug_assert!(x >= 0);
            debug_assert!(y >= 0);

            let tile = &dungeon[x as usize][y as usize];

            let draw_x = x - view_left;
            let draw_y = y - view_top;

            // Common case is that we draw a tile, so initialize with tile's values.
            let mut draw_c = tile.draw_c();
            let mut draw_color = tile.draw_color();

            let mut foreground_drawn = false;

            // Draw actor.
            if let Some(ref actor) = tile.actor {
                if actor.visible() {
                    draw_c = actor.draw_c();
                    draw_color = actor.draw_color();
                    foreground_drawn = true;
                }
            }

            // Draw item stash.
            if let Some(ref stash) = tile.stash {
                if !foreground_drawn {
                    draw_c = stash.draw_c();
                    draw_color = stash.draw_color();
                    foreground_drawn = true;
                }
            }

            // Draw object.
            if let Some(ref object) = tile.object {
                let object = object.inner.borrow();
                if !foreground_drawn && object.visible() {
                    draw_c = object.draw_c();
                    draw_color = object.draw_color();
                }
            }

            console.draw_char(draw_x, draw_y, draw_c, draw_color);
        }
    }
}
