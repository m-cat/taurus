use utility::{uint, int};
use coord::Coord;

/// Data structure for the player, enemies, NPCs, etc.
pub struct Actor {
    kind: ActorEnum,
    /// Character to draw with
    c: char,

    /// Coordinate location in level
    xy: Coord,

    // STATS
    hp_cur: int,
    hp_max: uint,

    // COMBAT STATE
    poison_amt: uint,

    // AI ATTRIBUTES
    aggression: AggrEnum,
}

enum ActorEnum {
    Player,
    GiantRat,
}

enum AggrEnum {
}
