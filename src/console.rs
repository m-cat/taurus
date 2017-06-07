//! A wrapper around the tcod library.
//!
//! This module is designed so that it can be replaced with a different console
//! implementation if necessary - it is an abstraction over tcod.

use tcod;
use tcod::Console;
use tcod::console::*;
use tcod::colors;
use tcod::input::Key;

use constants::*;
use util::uint;

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

    // TODO
    pub fn put_char(&mut self, x: uint, y: uint, c: char) {
        // TODO
        self.root.set_default_foreground(colors::WHITE);
        self.root
            .put_char(x as i32, y as i32, c, BackgroundFlag::None);
        self.root.flush();
    }

    /// If `flush` is true, all pending keypresses are flushed from the keyboard buffer.
    /// If `flush` is false, it returns the first element from it.
    pub fn wait_for_keypress(&mut self, flush: bool) -> Key {
        self.root.wait_for_keypress(flush)
    }

    /// Sets the main window's title to `title`.
    pub fn set_window_title<T>(&mut self, title: T)
        where T: AsRef<str> {
        self.root.set_window_title(title);
    }
}
