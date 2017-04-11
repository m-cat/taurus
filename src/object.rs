//! Taurus - object.rs
//! Copyright (C) 2017  Marcin Swieczkowski <scatman@bu.edu>

use taurus::coord::Coord;

/// A data structure for things like doors and traps which
/// can be interacted with. For more about the differences
/// between objects and actors, see actor.rs.
pub struct Object {
    /// The class of object, containing the object kind
    // class: ObjectClass,
    /// Coordinate location in level
    pub xy: Coord,
}

pub enum ObjectClass {
    Door(DoorKind, DoorState),
    Trap(TrapKind, TrapState),
}

pub enum DoorKind {}

pub enum DoorState {
    Open,
    Closed,
}

pub enum TrapKind {}

pub enum TrapState {
    Active,
    Inactive,
}
