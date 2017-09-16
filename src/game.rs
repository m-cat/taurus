use constants;
use coord::Coord;
use data;
use database::Database;
use fraction::Fraction;
use num_traits::identities::Zero;
use std::cell::Cell;
use std::collections::VecDeque;
use std::io;
use util::uint;

pub enum GameLoopResult {
    /// The player has changed depth
    DepthChanged(usize),
    /// Game window was closed by player
    WindowClosed,
    /// Player died and we need to return
    PlayerDead,
    /// No actors remaining in queue
    NoActors, // should never happen!
    /// Nothing special happened
    None,
}

/// Struct containing game-wide data such as the database and the message list.
pub struct Game {
    /// A reference to the main game database containing monster info, tile info, etc.
    pub database: Database,

    /// Message deque storing a fixed number of messages.
    message_deque: VecDeque<String>,

    /// Current depth that the player is on, indexed starting at 1.
    player_depth: Option<usize>,
    /// Current coordinates of the player
    player_xy: Option<Coord>,

    /// Current global game turn.
    turn: Cell<Fraction>, // Cell type used for interior mutability
    /// Number of actors created, used for assigning unique id's.
    num_actors: Cell<uint>, // Cell type for interior mutability
}

impl Game {
    pub fn new() -> io::Result<Game> {
        let mut database = Database::new(); // initialize the database
        data::init_game(&mut database)?;

        Ok(Game {
            database: database,
            message_deque: VecDeque::with_capacity(constants::MESSAGE_DEQUE_SIZE),
            player_depth: None,
            player_xy: None,
            turn: Cell::new(Fraction::zero()),
            num_actors: Cell::new(0),
        })
    }

    /// Gets the current depth that the player is on.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player_depth(&self) -> usize {
        self.player_depth.expect(
            "Game::player_depth failed: player does not exist.",
        )
    }
    pub fn set_player_depth(&mut self, value: usize) {
        self.player_depth = Some(value);
    }

    /// Gets the current coordinates of the player.
    ///
    /// # Panics
    /// If the player doesn't exist.
    pub fn player_xy(&self) -> Coord {
        self.player_xy.expect(
            "Game::player_xy failed: player does not exist.",
        )
    }
    pub fn set_player_xy(&mut self, value: Coord) {
        self.player_xy = Some(value);
    }

    pub fn turn(&self) -> Fraction {
        self.turn.get()
    }
    pub fn set_turn(&self, value: Fraction) {
        self.turn.set(value);
    }

    /// Generates a new unique actor id.
    pub fn actor_id(&self) -> uint {
        let n = self.num_actors.get();
        self.num_actors.set(n + 1);
        n
    }

    /// Adds a string to the message deque.
    pub fn add_message(&self, message: &str) {} // TODO. Should pop_front when queue gets too big
}
