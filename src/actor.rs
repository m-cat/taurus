use coord::Coord;
use dungeon::Dungeon;
use fraction::Fraction;
use game::Game;
use std::cmp::Ordering;
use util::{int, uint};

/// An actor is any entity which could conceivably `act`, that is, have a turn.
/// Only one actor can occupy a tile at a time.
/// For things like doors and traps, we have a separate struct named Object.
/// An object can share a tile with an actor.
pub struct Actor {
    /// Unique id for this instance.
    id: uint,
    // kind: ActorEnum,
    /// Character to draw to the console with.
    c: char,
    /// Coordinate location in level.
    xy: Coord,
    /// Current turn.
    turn: Fraction,

    // STATS
    hp_cur: int, // int, because this value can be negative!
    hp_max: uint,
    speed: Fraction,

    // COMBAT STATE

    // AI ATTRIBUTES
    behavior: Behavior,
}

impl Actor {
    pub fn insert_new(game: &Game, dungeon: &mut Dungeon, xy: Coord, name: &str) {
        let actor_database = game.database.get("actor").get(name);

        let hp = actor_database.get("hp").get_uint();

        let mut a = Actor {
            id: game.actor_id(),
            c: actor_database.get("c").get_char(),
            xy: xy,
            turn: game.turn(), // we update this later in this function

            hp_cur: hp as int,
            hp_max: hp,
            speed: actor_database.get("speed").get_fraction(),

            behavior: Behavior::string_to_behavior(actor_database.get("behavior").get_str()),
        };
        a.turn += a.speed(); // Set the actor's turn.

        dungeon.add_actor(xy, a);
    }

    /// Returns the actor id for this actor.
    pub fn id(&self) -> uint {
        self.id
    }

    /// Returns the name associated with this actor.
    pub fn name(&self) -> &str {
        // TODO
        "test"
    }

    /// Generates and returns the description of this actor.
    pub fn description(&self) -> String {
        // TODO
        "test".to_string()
    }

    /// Returns this actor's base speed.
    pub fn speed(&self) -> Fraction {
        self.speed
    }

    /// Returns this actor's coordinates.
    pub fn coord(&self) -> Coord {
        self.xy
    }

    /// Sets this actor's coordinates.
    pub fn set_coord(&mut self, xy: Coord) {
        self.xy = xy;
    }

    /// Returns this actor's next turn value.
    pub fn turn(&self) -> Fraction {
        self.turn
    }

    /// Sets this actor's turn to a new value.
    pub fn set_turn(&mut self, turn: Fraction) {
        self.turn = turn;
    }

    /// Updates this actor's turn.
    pub fn update_turn(&mut self) {
        self.turn += self.speed();
    }

    /// Returns this actor's behavior value.
    pub fn behavior(&self) -> Behavior {
        self.behavior
    }

    /// Acts out the actor's turn.
    /// Could change itself or the dungeon as a side effect.
    /// Actor should update its own `turn` value.
    pub fn act(&mut self, game: &Game, dungeon: &mut Dungeon) -> ActResult {
        unimplemented!();

        // let result = match self.behavior {
        //     Behavior::Player => player::player_act(self, game, dungeon),
        //     _ => ActResult::None,
        // };

        // match result {
        //     ActResult::None => {}
        //     _ => return result,
        // }

        // ActResult::None
    }
}

/// Enum listing possible AI states of an actor.
#[derive(Clone, Copy)]
pub enum Behavior {
    /// Behavior corresponding to the player itself.
    Player,
    Friendly,
    Wary,
    Defensive,
    Hostile,
    Hunting,
}

impl Behavior {
    /// Converts `string` to a `Behavior` enum.
    ///
    /// # Panics
    /// If `string` does not correspond to a Behavior value.
    pub fn string_to_behavior(string: &str) -> Behavior {
        match string {
            "Player" => Behavior::Player,
            "Friendly" => Behavior::Friendly,
            "Wary" => Behavior::Wary,
            "Defensive" => Behavior::Defensive,
            "Hostile" => Behavior::Hostile,
            "Hunting" => Behavior::Hunting,
            _ => {
                panic!(
                    "Behavior::string_to_behavior failed: invalid input \"{}\"",
                    string
                )
            }
        }
    }
}

/// Enum of possible results of a player acton.
pub enum ActResult {
    WindowClosed,
    None,
}

/// A struct that combines Coord + turn.
/// Why? So we can use coords instead of actors in the priority queue and sort the queue by turn.
/// This allows the actor map to fully own the actors. We always get actors from one place,
/// the actor map, keyed by Coord.
pub struct CoordTurn {
    pub xy: Coord,
    pub turn: Fraction,
    /// The id of the actor.
    pub id: uint,
}

// Traits for priority queue

impl Eq for CoordTurn {}
impl PartialEq for CoordTurn {
    /// Returns a1 == a2 iff their `turn` values are equal.
    fn eq(&self, other: &CoordTurn) -> bool {
        self.turn == other.turn
    }
}

impl Ord for CoordTurn {
    /// Compares CoordTurns (and actors by proxy) by turn.
    /// Note that the ordering is flipped so the priority queue becomes a min-heap.
    fn cmp(&self, other: &CoordTurn) -> Ordering {
        // Since we're comparing floating values here, we have to use partial_cmp.
        // We should never do an invalid comparison here, so this is okay
        other.turn.partial_cmp(&self.turn).unwrap()
    }
}
impl PartialOrd for CoordTurn {
    fn partial_cmp(&self, other: &CoordTurn) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
