//! Taurus - constants.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

#![allow(dead_code)]

use taurus::util::uint;

pub const TITLE: &'static str = "Taurus";
pub const SCR_WIDTH: uint = 80;
pub const SCR_HEIGHT: uint = 50;
pub const FPS: uint = 60;

pub const MAX_NAME_LEN: usize = 20;

// GAME DATA LOCATIONS

pub const ORG_DIRECTORY: &'static str = "org";
pub const ORG_MANUAL: &'static str = "manual.org";

// DEFAULT SETTINGS

pub const FONT_DEFAULT: &'static str = "data/fonts/terminal8x12_gs_tc.png";
// pub const FONT_DEFAULT: &'static str = "data/fonts/arial10x10.png";
