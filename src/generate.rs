//! Module containing dungeon generation algorithms.

use GameResult;
use actor::Actor;
use coord::Coord;
use defs::int;
use dungeon::{Dungeon, DungeonList};
use object::Object;
use util::direction::CardinalDirection;
use util::direction::CardinalDirection::*;
use util::math::{min_max, overlaps};
use util::rand::{Choose, dice, rand_int};

/// Generates the entire dungeon.
pub fn gen_game(dungeon_list: &mut DungeonList) -> GameResult<()> {
    // Generate each depth.
    for n in 0..dungeon_list.len() {
        gen_depth(dungeon_list, n)?;
    }

    // Generate pits, skipping the last depth.
    for n in 0..dungeon_list.len() - 1 {
        gen_pits(dungeon_list, n);
    }

    gen_player(&mut dungeon_list[0])
}

/// Generates a single depth of the dungeon.
fn gen_depth(dungeon_list: &mut DungeonList, index: usize) -> GameResult<()> {
    let mut dungeon = &mut dungeon_list[index];

    gen_dungeon_room_method(&mut dungeon, index)?;
    // let a = Actor::new(game);
    // add_actor_random_coord(dungeon, a);

    Ok(())
}

/// Generates pits for a depth of the dungeon.
fn gen_pits(dungeon_list: &mut DungeonList, index: usize) {
    debug_assert!(index < dungeon_list.len() - 1);

    unimplemented!();
}

/// Creates an actor of type `name` and places it in a random open location in `dungeon`.
fn gen_actor_random_coord(dungeon: &mut Dungeon, name: &str) -> GameResult<()> {
    let coord = dungeon.random_open_coord_actor();

    debug_assert!(coord.is_some());

    // If we're out of squares, don't add the actor.
    let coord = match coord {
        Some(coord) => coord,
        None => return Ok(()),
    };

    let a = Actor::new(dungeon.mut_game_data(), coord, name)?;
    dungeon.add_actor(a);

    Ok(())
}

/// Creates the player and places him in a random location of the dungeon.
// TODO: Check if the actor was added successfully. This should never fail.
fn gen_player(dungeon: &mut Dungeon) -> GameResult<()> {
    gen_actor_random_coord(dungeon, "player")
}

/// Generates a dungeon level using the "room method".
fn gen_dungeon_room_method(dungeon: &mut Dungeon, index: usize) -> GameResult<()> {
    let mut room_list: Vec<Room> = Vec::new();
    let mut object_list: Vec<Object> = Vec::new();
    let direction_list = vec![N, E, S, W];
    let goal_num_rooms = gen_num_rooms(index);

    // Generate the initial room
    room_list.push(Room::from_dimensions(
        0,
        0,
        gen_room_width(index),
        gen_room_height(index),
    ));

    // Generate rooms by looking for free space next to existing rooms
    for _ in 0..goal_num_rooms - 1 {
        loop {
            let room = room_list.choose().unwrap().clone();
            let direction = direction_list.choose().unwrap();

            // Try a few times to generate a room here
            if let Some(new_room) = try_some!(
                gen_room_adjacent(
                    dungeon,
                    &room,
                    *direction,
                    &room_list,
                    &mut object_list,
                    index,
                )?,
                3
            )
            {
                room_list.push(new_room);
                break;
            };
        }
    }

    // Initialize the dungeon tile grid
    init_dungeon_from_rooms(dungeon, &room_list);

    // Convert the list of rooms into a tile grid representation

    // Add doors
    for object in object_list {
        dungeon.add_object(object);
    }

    Ok(())
}

/// Generates a room adjacent to `room`, or returns `None`.
fn gen_room_adjacent(
    dungeon: &mut Dungeon,
    room: &Room,
    direction: CardinalDirection,
    room_list: &[Room],
    object_list: &mut Vec<Object>,
    index: usize,
) -> GameResult<Option<Room>> {
    let top: int;
    let left: int;
    let width = gen_room_width(index) as int;
    let height = gen_room_height(index) as int;

    match direction {
        N => {
            top = room.top - height - 1;
            left = rand_int(room.left - width + 1, room.right);
        }
        E => {
            top = rand_int(room.top - height + 1, room.bottom);
            left = room.right + 2;
        }
        S => {
            top = room.bottom + 2;
            left = rand_int(room.left - width + 1, room.right);
        }
        W => {
            top = rand_int(room.top - height + 1, room.bottom);
            left = room.left - width - 1;
        }
    };
    let new_room = Room::from_dimensions(top, left, width as usize, height as usize);

    Ok(if check_room_free(&new_room, room_list) {
        let door = gen_room_adjacent_door(dungeon, room, &new_room, direction)?;
        object_list.push(door);
        Some(new_room)
    } else {
        None
    })
}

/// Generates a door between two adjacent `Room`s in given `Direction`.
fn gen_room_adjacent_door(
    dungeon: &mut Dungeon,
    room: &Room,
    other: &Room,
    direction: CardinalDirection,
) -> GameResult<Object> {
    let x: int;
    let y: int;

    match direction {
        N => {
            x = rand_int(max!(room.left, other.left), min!(room.right, other.right));
            y = room.top - 1;
        }
        E => {
            x = room.right + 1;
            y = rand_int(max!(room.top, other.top), min!(room.bottom, other.bottom));
        }
        S => {
            x = rand_int(max!(room.left, other.left), min!(room.right, other.right));
            y = room.bottom + 1;
        }
        W => {
            x = room.left - 1;
            y = rand_int(max!(room.top, other.top), min!(room.bottom, other.bottom));
        }
    }

    let coord = Coord::new(x, y);
    Object::new(dungeon.game_data(), coord, "wooden door", dice(8, 10))
}

/// Checks if `room` does not collide with any `Room`s in `room_list`.
fn check_room_free(room: &Room, room_list: &[Room]) -> bool {
    !room_list.iter().any(|other| room.overlaps(other))
}

/// Initializes `dungeon`'s dungeon grid based on the `Room`s in `room_list`.
fn init_dungeon_from_rooms(dungeon: &mut Dungeon, room_list: &[Room]) {
    // todo
}

/// Generates the number of `Room`s for the dungeon level specified by `index`.
fn gen_num_rooms(index: usize) -> usize {
    10 + 10 * index // TODO
}

/// Generates a random width for a `Room` based on the dungeon level specified by `index`.
fn gen_room_width(index: usize) -> usize {
    rand_int(2, 5) // TODO
}

/// Generates a random height for a `Room` based on the dungeon level specified by `index`.
fn gen_room_height(index: usize) -> usize {
    rand_int(2, 5) // TODO
}

/// A struct for storing data for a single `Room`, used in dungeon generation.
/// Note that the four bounding boxes correspond to the `Room`'s interior
/// and do not include its walls.
#[derive(Clone, Eq, PartialEq, Debug)]
struct Room {
    left: int,
    top: int,
    right: int,
    bottom: int,
}

impl Room {
    /// Returns a new `Room` with given bounding boxes.
    #[allow(dead_code)]
    pub fn new(left: int, top: int, right: int, bottom: int) -> Room {
        let (left, right) = min_max(left, right);
        let (top, bottom) = min_max(top, bottom);

        Room {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    /// Returns a new `Room` created from given `width` and `height`.
    pub fn from_dimensions(left: int, top: int, width: usize, height: usize) -> Room {
        debug_assert!(width > 0 && height > 0);

        Room {
            left: left,
            top: top,
            right: left + width as int - 1,
            bottom: top + height as int - 1,
        }
    }

    /// Returns true if `self` and `other` overlap.
    /// Note that we allow walls to overlap, but not so the interiors of the `Room`s
    /// are connected.
    pub fn overlaps(&self, other: &Self) -> bool {
        overlaps(self.left - 1, self.right, other.left - 1, other.right) &&
            overlaps(self.top - 1, self.bottom, other.top - 1, other.bottom)
    }
}

#[cfg(test)]
mod tests {
    use generate::Room;

    #[test]
    fn room_new() {
        assert_eq!(Room::new(1, 1, 2, 2), Room::from_dimensions(1, 1, 2, 2));
    }

    #[test]
    fn room_overlaps() {
        let rooms = vec![
            Room::new(0, 0, 1, 1),
            Room::new(1, 0, 3, 3),
            Room::new(-1, -1, 4, 4),
            Room::new(-3, -3, -2, -2),
        ];

        assert!(rooms[0].overlaps(&rooms[1]));
        assert!(rooms[0].overlaps(&rooms[2]));
        assert!(rooms[2].overlaps(&rooms[3]));
        assert!(!rooms[0].overlaps(&rooms[3]));
        assert!(!rooms[1].overlaps(&rooms[3]));

        for (i, room1) in rooms.iter().enumerate() {
            for (j, room2) in rooms.iter().enumerate() {
                if i == j {
                    assert!(room1.overlaps(room2));
                }
                assert_eq!(room1.overlaps(room2), room2.overlaps(room1));
            }
        }
    }
}
