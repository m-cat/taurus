//! User interface module.

use console::{Color, Console};
use dungeon::Dungeon;

/// Trait for any object that can be drawn.
pub trait Draw {
    fn draw_c(&self) -> char;
    fn draw_color(&self) -> Color;
}

pub fn game_draw(dungeon: &Dungeon, console: &Console) {
    unimplemented!();
}
