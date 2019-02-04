//! Console I/O module.
//!
//! This module is designed so that it can be replaced with a different console
//! implementation if necessary - it is an abstraction over tcod.

pub use tcod::input::{self, Event, EventFlags, Key, KeyCode};

use crate::constants;
use crate::database::Database;
use crate::defs::*;
use crate::util::convert::color_code_to_rgb;
use crate::{GameError, GameResult};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Mutex;
use tcod;
use tcod::input::check_for_event;
use tcod::Color as TcodColor;
use tcod::{Console, FontLayout, FontType, Renderer, RootConsole};

/// Color struct.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Converts this `Color` to a tcod Color.
    pub fn to_tcod(&self) -> TcodColor {
        let Color { r, g, b } = *self;
        TcodColor::new(r, g, b)
    }
}

impl FromStr for Color {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match color_code_to_rgb(s) {
            Some((r, g, b)) => Ok(Color { r, g, b }),
            None => Err(GameError::ConversionError {
                val: s.into(),
                msg: "Make sure the string is in the format \"#FFFFFF\".",
            }),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

#[derive(Debug)]
pub struct ConsoleSettings {
    title: String,
    width: i32,
    height: i32,
    fps: i32,
    font: String,
    background_color: Color,
}

impl ConsoleSettings {
    pub fn new(data: &Database) -> GameResult<ConsoleSettings> {
        // Load all data from the database.

        let title = data.get_str("title")?;
        let width = big_to_i32(data.get_int("console_width")?)?;
        let height = big_to_i32(data.get_int("console_height")?)?;
        let fps = big_to_i32(data.get_int("fps")?)?;
        let font = data.get_str("default_font")?;
        let background_color = Color::from_str(&data.get_str("background_color")?)?;

        Ok(ConsoleSettings {
            title,
            width,
            height,
            fps,
            font,
            background_color,
        })
    }
}

/// Console object responsible for display and input.
pub struct DrawConsole {
    root: RootConsole,
}

impl DrawConsole {
    /// Initialize a console.
    pub fn new(settings: &ConsoleSettings) -> DrawConsole {
        tcod::system::set_fps(settings.fps);

        let mut console = DrawConsole {
            root: RootConsole::initializer()
                .size(settings.width, settings.height)
                .title(settings.title.clone())
                .font(settings.font.clone(), FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .renderer(Renderer::OpenGL)
                .init(),
        };
        console.set_default_background(settings.background_color);

        console
    }

    pub fn set_default_background(&mut self, color: Color) {
        self.root.set_default_background(color.to_tcod());
    }

    /// Returns true if the Root console has been closed.
    pub fn window_closed(&self) -> bool {
        self.root.window_closed()
    }

    /// Puts given char `c` to tile at `x` and `y` with `color`.
    pub fn draw_char(&mut self, x: i32, y: i32, c: char, color: Color) {
        self.root.set_char(x, y, c);
        self.root.set_char_foreground(x, y, color.to_tcod());
    }

    // TODO: Stop printing if edge of console is hit
    // TODO: Add version of this with max x and y and with wrapping
    pub fn put_str(&mut self, x: i32, y: i32, s: &str, color: Color) {
        for (j, c) in s.chars().enumerate() {
            self.draw_char(j as i32, y, c, color);
        }
    }

    /// This function will wait for a keypress event from the user, returning the `KeyState` that
    /// represents the event.
    /// If `flush` is true, all pending keypresses are flushed from the keyboard buffer.
    /// If `flush` is false, it returns the first element from it.
    pub fn wait_for_keypress(&mut self, flush: bool) -> Key {
        self.root.wait_for_keypress(flush)
    }

    pub fn check_for_event(&self, event_mask: EventFlags) -> Option<(EventFlags, Event)> {
        check_for_event(event_mask)
    }

    /// Sets the main window's title to `title`.
    pub fn set_window_title<T>(&mut self, title: T)
    where
        T: AsRef<str>,
    {
        self.root.set_window_title(title);
    }

    pub fn flush(&mut self) {
        self.root.flush();
    }

    pub fn clear(&mut self) {
        self.root.clear();
    }

    pub fn get_fps(&self) -> i32 {
        tcod::system::get_fps()
    }
}

impl fmt::Debug for DrawConsole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Console")
    }
}
