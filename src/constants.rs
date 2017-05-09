#![allow(dead_code)]

use util::uint;

pub const TITLE: &'static str = "Taurus";
pub const SCR_WIDTH: uint = 80;
pub const SCR_HEIGHT: uint = 50;
pub const FPS: uint = 60;

pub const NUM_DUNGEONS: usize = 27;

pub const DUNGEON_WIDTH_DEFAULT: usize = 200;
pub const DUNGEON_HEIGHT_DEFAULT: usize = 200;

pub const MESSAGE_DEQUE_SIZE: usize = 1000;

pub const MAX_NAME_LEN: usize = 20;

// GAME DATA LOCATIONS

pub const ORG_DIRECTORY: &'static str = "org";
pub const ORG_MANUAL: &'static str = "manual.org";

// DEFAULT SETTINGS

pub const FONT_DEFAULT: &'static str = "data/fonts/terminal8x12_gs_tc.png";
// pub const FONT_DEFAULT: &'static str = "data/fonts/arial10x10.png";
