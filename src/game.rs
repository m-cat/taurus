use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use num_traits::identities::Zero;
use fraction::Fraction;

use util::uint;
use console::GameConsole;
use database::Database;
use dungeon::Dungeon;
use generate;
use constants;

/// Struct containing game-wide data such as the draw console,
/// the database, and the dungeon levels
pub struct Game {
    /// A reference to the console, wrapped around Rc<Refcell>.
    pub console: Rc<RefCell<GameConsole>>,
    /// A reference to the main game database containing monster info, tile info, etc
    pub database: Database,

    /// Message deque storing a fixed number of messages
    message_deque: VecDeque<String>,

    /// Current depth that the player is on, indexed starting at 1
    depth: usize,
    /// Current global game turn
    turn: Cell<Fraction>, // Cell type used for interior mutability

    /// Number of actors created, used for assigning unique id's
    num_actors: Cell<uint>, // Cell type for interior mutability
}

impl Game {
    pub fn new() -> Game {
        let console = GameConsole::init(); // initialize the console
        let database = Database::init(); // initialize the database

        Game {
            console: Rc::new(RefCell::new(console)),
            database: database,
            message_deque: VecDeque::with_capacity(constants::MESSAGE_DEQUE_SIZE),
            depth: 1,
            turn: Cell::new(Fraction::zero()),
            num_actors: Cell::new(0),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn set_depth(&mut self, value: usize) {
        self.depth = value;
    }

    pub fn turn(&self) -> Fraction {
        self.turn.get()
    }
    pub fn set_turn(&self, value: Fraction) {
        self.turn.set(value);
    }

    pub fn actor_id(&self) -> uint {
        let n = self.num_actors.get();
        self.num_actors.set(n + 1);
        n
    }

    /// Adds a string to the message deque
    pub fn add_message(&self, message: &str) {} // TODO. Should pop_front when queue gets too big
}
