extern crate rand;
extern crate num;

extern crate tcod;

mod constants;
mod utility;
mod actor;

use constants::*;
use utility::language;
use utility::coord;
use tcod::console::*;
use tcod::colors;

fn main() {
    let mut root = Root::initializer()
        .font(FONT_DEFAULT, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCR_WIDTH as i32, SCR_HEIGHT as i32)
        .title(TITLE)
        .init();
    tcod::system::set_fps(FPS as i32);

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }

    for _ in 1..1000 {
        println!("{}", language::name_gen());
    }
}
