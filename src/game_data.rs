//! Module for game-wide data.

use crate::actor::Actor;
use crate::console::{ConsoleSettings, DrawConsole};
use crate::constants;
use crate::coord::Coord;
use crate::database::{self, Database, Value};
use crate::defs::GameRatio;
use crate::material::MaterialInfo;
use crate::tile::TileInfo;
use crate::ui::UiSettings;
use crate::{handle_error, GameResult, DATABASE};
use failure::ResultExt;
use num_traits::identities::Zero;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// Result of the main game loop.
pub enum GameLoopOutcome {
    /// The player has changed depth.
    DepthChanged,
    /// Game window was closed by player.
    WindowClosed,
    /// Game has been quit.
    QuitGame,
    /// Player died and we need to return.
    PlayerDead,
    /// No actors remaining in queue.
    NoActors, // should never happen!
    /// Nothing special happened.
    None,
}

#[derive(Debug)]
pub struct GameData {
    pub console_settings: ConsoleSettings,
    /// A struct containing UI parameters.
    pub ui_settings: UiSettings,

    /// Message deque storing a fixed number of messages.
    message_list: VecDeque<String>,

    /// Reference to the player.
    player: Option<Actor>,
    /// Current global game turn.
    turn: GameRatio,

    /// Vector of tile info structs, indexed by id.
    tile_info_list: Vec<Arc<TileInfo>>,
    tile_start_id: Option<usize>,

    /// Vector of material structs, indexed by id.
    material_info_list: Vec<Arc<MaterialInfo>>,
    material_start_id: Option<usize>,
}

impl GameData {
    /// Creates a new `GameData` object.
    pub fn new() -> GameResult<GameData> {
        let settings = DATABASE.read().unwrap().get_obj("settings")?;
        let console_settings = ConsoleSettings::new(&settings)?;
        let ui_settings = UiSettings::new(&settings)?;

        let mut game_data = GameData {
            console_settings,
            ui_settings,

            message_list: VecDeque::with_capacity(constants::MESSAGE_DEQUE_SIZE),

            player: None,
            turn: GameRatio::zero(),

            tile_info_list: Vec::new(),
            tile_start_id: None,
            material_info_list: Vec::new(),
            material_start_id: None,
        };

        // As tiles contain materials, initialize materials first.
        game_data.init_materials()?;
        game_data.init_tiles()?;

        Ok(game_data)
    }

    /// Adds a string to the message deque.
    pub fn add_message(&self, message: &str) {} // TODO. Should pop_front when queue gets too big

    /// Gets a reference to the player.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player(&self) -> Actor {
        self.player.clone().unwrap()
    }

    pub fn set_player(&mut self, player: Actor) {
        self.player = Some(player)
    }

    /// Gets the current game turn.
    pub fn turn(&self) -> GameRatio {
        self.turn
    }

    /// Sets the game turn.
    pub fn set_turn(&mut self, value: GameRatio) {
        self.turn = value;
    }

    /// Returns a reference to the `TileInfo` object with `id`.
    pub fn tile_info(&self, id: usize) -> Arc<TileInfo> {
        Arc::clone(&self.tile_info_list[id - self.tile_start_id.unwrap()])
    }

    /// Returns a reference to the `MaterialInfo` object with `id`.
    pub fn material_info(&self, id: usize) -> Arc<MaterialInfo> {
        Arc::clone(&self.material_info_list[id - self.material_start_id.unwrap()])
    }

    fn init_tiles(&mut self) -> GameResult<()> {
        let tiles = DATABASE.read().unwrap().get_obj("tiles")?;
        let mut vec_temp: Vec<(Arc<TileInfo>, usize)> = Vec::new();
        let mut min = usize::max_value();

        for tile_val in tiles.values() {
            if let Value::Obj(ref tile_data) = *tile_val {
                let tile = Arc::new(
                    TileInfo::new(self, tile_data)
                        .context(format!("Could not load tile:\n{}", tile_data))?,
                );
                let id = tile_data.id();
                if id < min {
                    min = id;
                }
                vec_temp.push((tile, id));
            }
        }

        let mut vec_option: Vec<Option<Arc<TileInfo>>> = vec![None; vec_temp.len()];
        for (tile, id) in vec_temp {
            vec_option[id - min] = Some(tile);
        }
        let vec_final = vec_option.into_iter().map(|opt| opt.unwrap()).collect();
        self.tile_info_list = vec_final;
        self.tile_start_id = Some(min);

        Ok(())
    }

    fn init_materials(&mut self) -> GameResult<()> {
        let materials = DATABASE.read().unwrap().get_obj("materials")?;
        let mut vec_temp: Vec<(Arc<MaterialInfo>, usize)> = Vec::new();
        let mut min = usize::max_value();

        for material_val in materials.values() {
            if let Value::Obj(ref material_data) = *material_val {
                let material = Arc::new(
                    MaterialInfo::new(self, material_data)
                        .context(format!("Could not load material:\n{}", material_data))?,
                );
                let id = material_data.id();
                if id < min {
                    min = id;
                }
                vec_temp.push((material, id));
            }
        }

        // TODO: Use mem uninitialized to avoid this step.
        let mut vec_option: Vec<Option<Arc<MaterialInfo>>> = vec![None; vec_temp.len()];
        for (material, id) in vec_temp {
            vec_option[id - min] = Some(material);
        }
        let vec_final = vec_option.into_iter().map(|opt| opt.unwrap()).collect();
        self.material_info_list = vec_final;
        self.material_start_id = Some(min);

        Ok(())
    }
}
