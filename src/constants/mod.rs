//! Game constants.

pub mod colors;

pub const TITLE: &'static str = "Taurus";
pub const SCR_WIDTH: usize = 80;
pub const SCR_HEIGHT: usize = 50;
pub const FPS: usize = 60;

pub const NUM_DUNGEONS: usize = 27;

pub const MESSAGE_DEQUE_SIZE: usize = 1000;

pub const MAX_NAME_LEN: usize = 20;

// GAME DATA LOCATIONS

pub const ORG_DIRECTORY: &'static str = "data/manuals";
pub const ORG_MANUAL: &'static str = "game.org";

// DEFAULT SETTINGS

pub const FONT_DEFAULT: &'static str = "data/fonts/terminal10x16_gs_tc.png";
// pub const FONT_DEFAULT: &'static str = "data/fonts/arial10x10.png";
