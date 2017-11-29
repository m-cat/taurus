//! Module for game-wide data.

use GameResult;
use actor::Actor;
use constants;
use coord::Coord;
use database::{self, Database};
use defs::GameRatio;
use num_traits::identities::Zero;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use tile::TileInfo;

/// Result of the main game loop.
pub enum GameLoopOutcome {
    /// The player has changed depth
    DepthChanged,
    /// Game window was closed by player
    WindowClosed,
    /// Game has been quit.
    QuitGame,
    /// Player died and we need to return
    PlayerDead,
    /// No actors remaining in queue
    NoActors, // should never happen!
    /// Nothing special happened
    None,
}

#[derive(Debug)]
struct GameDataInner {
    /// A reference to the main game database containing monster info, tile info, etc.
    database: Database,

    /// Message deque storing a fixed number of messages.
    message_deque: VecDeque<String>,

    /// Reference to the player.
    player: Option<Actor>,
    /// Current depth of the player, indexed starting at 1.
    player_depth: Option<usize>,

    /// Current global game turn.
    turn: GameRatio,
    /// Number of actors created, used for assigning unique id's.
    num_actors: u32,

    /// Map of tile names to tile info structs.
    tile_info_map: HashMap<String, Rc<TileInfo>>,
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

        Ok(GameData {
            inner: Rc::new(RefCell::new(GameDataInner {
                database: database,

                message_deque: VecDeque::with_capacity(constants::MESSAGE_DEQUE_SIZE),

                player: None,
                player_depth: None,

                turn: GameRatio::zero(),
                num_actors: 0,

                tile_info_map: HashMap::new(),
            })),
        })
    }

    /// Returns a clone of the database.
    pub fn database(&self) -> Database {
        self.inner.borrow().database.clone()
    }

    /// Adds a string to the message deque.
    pub fn add_message(&self, message: &str) {} // TODO. Should pop_front when queue gets too big

    /// Gets the current depth that the player is on.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player_depth(&self) -> usize {
        self.inner.borrow().player_depth.unwrap()
    }

    /// Sets the player depth.
    pub fn set_player_depth(&mut self, value: usize) {
        self.inner.borrow_mut().player_depth = Some(value);
    }

    /// Gets a reference to the player.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player(&self) -> Actor {
        let inner = self.inner.borrow();
        inner.player.clone().unwrap()
    }

    pub fn set_player(&mut self, player: Actor) {
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
    pub fn actor_id(&mut self) -> u32 {
        let mut inner = self.inner.borrow_mut();
        let id = inner.num_actors;
        inner.num_actors += 1;
        id
    }

    /// Returns a reference to the `TileInfo` object with `name`.
    pub fn tile_info(&self, name: &str) -> Option<Rc<TileInfo>> {
        self.inner.borrow().tile_info_map.get(name).cloned()
    }

    /// Adds `tile_info` to the list and returns a reference to it.
    pub fn add_tile_info(&mut self, tile_info: TileInfo, name: String) -> Rc<TileInfo> {
        let info_ref = Rc::new(tile_info);
        match self.inner.borrow_mut().tile_info_map.insert(
            name,
            Rc::clone(&info_ref),
        ) {
            Some(_) => panic!("logic error"),
            None => info_ref,
        }
    }
}
