//! Module for game-wide data.

use GameResult;
use actor::Actor;
use constants;
use coord::Coord;
use database::{self, Database, Value};
use defs::GameRatio;
use failure::ResultExt;
use material::MaterialInfo;
use num_traits::identities::Zero;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use tile::TileInfo;
use ui::UiSettings;

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
struct GameDataInner {
    /// A reference to the main game database containing monster info, tile info, etc.
    database: Database,

    /// A struct containing UI parameters.
    ui_settings: UiSettings,

    /// Message deque storing a fixed number of messages.
    message_list: VecDeque<String>,

    /// Reference to the player.
    player: Option<Actor>,

    /// Current global game turn.
    turn: GameRatio,
    /// Number of actors created, used for assigning unique id's.
    num_actors: usize,

    /// Vector of tile info structs, indexed by id.
    tile_info_list: Vec<Rc<TileInfo>>,
    /// Vector of material structs, indexed by id.
    material_info_list: Vec<Rc<MaterialInfo>>,
}

/// Struct containing game-wide data such as the database and the message list.
#[derive(Clone, Debug)]
pub struct GameData {
    inner: Rc<RefCell<GameDataInner>>,
}

impl GameData {
    /// Creates a new `GameData` object.
    pub fn new() -> GameResult<GameData> {
        let database = database::load_data()?;

        let ui_settings = UiSettings::new(&database.get_obj("settings")?)?;

        let mut game_data = GameData {
            inner: Rc::new(RefCell::new(GameDataInner {
                database: database.clone(),

                ui_settings,

                message_list: VecDeque::with_capacity(constants::MESSAGE_DEQUE_SIZE),

                player: None,

                turn: GameRatio::zero(),
                num_actors: 0,

                tile_info_list: Vec::new(),
                material_info_list: Vec::new(),
            })),
        };

        // As tiles contain materials, initialize materials first.
        let material_info_list = game_data.init_materials(&database)?;
        let tile_info_list = game_data.init_tiles(&database)?;

        Ok(game_data)
    }

    /// Returns a clone of the database.
    pub fn database(&self) -> Database {
        self.inner.borrow().database.clone()
    }

    /// Returns a clone of the database.
    pub fn ui_settings(&self) -> UiSettings {
        self.inner.borrow().ui_settings
    }

    /// Adds a string to the message deque.
    pub fn add_message(&self, message: &str) {} // TODO. Should pop_front when queue gets too big

    /// Gets a reference to the player.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player(&self) -> Actor {
        let inner = self.inner.borrow();
        inner.player.clone().unwrap()
    }

    pub fn set_player(&self, player: Actor) {
        self.inner.borrow_mut().player = Some(player)
    }

    /// Gets the current game turn.
    pub fn turn(&self) -> GameRatio {
        self.inner.borrow().turn
    }

    /// Sets the game turn.
    pub fn set_turn(&self, value: GameRatio) {
        self.inner.borrow_mut().turn = value;
    }

    /// Generates a new unique actor id.
    pub fn actor_id(&self) -> usize {
        let mut inner = self.inner.borrow_mut();
        let id = inner.num_actors;
        inner.num_actors += 1;
        id
    }

    /// Returns a reference to the `TileInfo` object with `id`.
    pub fn tile_info(&self, id: usize) -> Rc<TileInfo> {
        self.inner.borrow().tile_info_list[id].clone()
    }

    /// Returns a reference to the `MaterialInfo` object with `id`.
    pub fn material_info(&self, id: usize) -> Rc<MaterialInfo> {
        self.inner.borrow().material_info_list[id].clone()
    }

    fn init_tiles(&mut self, database: &Database) -> GameResult<()> {
        let tiles = database.get_obj("tiles")?;
        let len = tiles.len();
        let mut vec_temp: Vec<Option<Rc<TileInfo>>> = vec![None; len];

        for tile_val in tiles.values() {
            if let Value::Obj(ref tile_data) = *tile_val {
                let tile = Rc::new(TileInfo::new(self, tile_data).context(format!(
                    "Could not load tile:\n{}",
                    tile_data
                ))?);
                vec_temp[tile_data.id()] = Some(tile);
            }
        }
        let vec_final = vec_temp.into_iter().map(|opt| opt.unwrap()).collect();
        self.inner.borrow_mut().tile_info_list = vec_final;

        Ok(())
    }

    fn init_materials(&mut self, database: &Database) -> GameResult<()> {
        let materials = database.get_obj("materials")?;
        let len = materials.len();
        let mut vec_temp: Vec<Option<Rc<MaterialInfo>>> = vec![None; len];

        for material_val in materials.values() {
            if let Value::Obj(ref material_data) = *material_val {
                let material = Rc::new(MaterialInfo::new(material_data).context(format!(
                    "Could not load material:\n{}",
                    material_data
                ))?);
                vec_temp[material_data.id()] = Some(material);
            }
        }
        let vec_final = vec_temp.into_iter().map(|opt| opt.unwrap()).collect();
        self.inner.borrow_mut().material_info_list = vec_final;

        Ok(())
    }
}
