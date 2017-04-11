//! Taurus - console.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>
//!
//! A wrapper around the tcod library.
//!
//! This module is designed so that it can be replaced with a different console
//! implementation if necessary (and to abstract around tcod).

use tcod;
use tcod::Console;
use tcod::console::*;
use tcod::colors;

use constants::*;
use taurus::util::uint;

pub struct GameConsole {
    root: Root,
}

impl GameConsole {
    pub fn init() -> GameConsole {
        tcod::system::set_fps(FPS as i32);

        GameConsole {
            root: Root::initializer()
                .font(FONT_DEFAULT, FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .size(SCR_WIDTH as i32, SCR_HEIGHT as i32)
                .title(TITLE)
                .init(),
        }
    }

    pub fn window_closed(&self) -> bool {
        self.root.window_closed()
    }

    pub fn put_char(&mut self, y: uint, x: uint, c: char) {
        // TODO: is x/y order right here?
        self.root.set_default_foreground(colors::WHITE);
        self.root
            .put_char(y as i32, x as i32, c, BackgroundFlag::None);
        self.root.flush();
    }

    // TODO: what does this parameter do?
    pub fn wait_for_keypress(&mut self, truth: bool) {
        self.root.wait_for_keypress(truth);
    }
}
