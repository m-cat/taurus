//! Module containing dungeon generation algorithms.

use util::*;
use util::Direction::*;
use constants;
use coord::Coord;
use object::Object;
use actor::Actor;
use dungeon::Dungeon;
use game::Game;

/// Generates the entire dungeon.
pub fn gen_game(game: &mut Game, dungeon_list: &mut Vec<Dungeon>) {
    // Create the list of dungeons
    for n in 0..constants::NUM_DUNGEONS {
        dungeon_list.push(Dungeon::new(n + 1));
    }

    // Generate each depth
    for n in 0..constants::NUM_DUNGEONS {
        gen_depth(game, dungeon_list, n);
    }

    // Generate pits
    for n in 0..constants::NUM_DUNGEONS - 1 {
        gen_pits(game, dungeon_list, n);
    }

    gen_player(game, &mut dungeon_list[0]);
}

/// Generates a single depth of the dungeon.
fn gen_depth(game: &Game, dungeon_list: &mut Vec<Dungeon>, index: usize) {
    let mut dungeon = dungeon_list
        .get_mut(index)
        .expect("Generate::gen_depth failed, invalid index");

    gen_dungeon_room_method(&mut dungeon, index);
    // let a = Actor::new(game);
    // add_actor_random_coord(dungeon, a);
}

/// Generates pits for a depth of the dungeon.
///
/// # Panics
/// If `index` corresponds to the last depth in `dungeon_list`.
fn gen_pits(game: &Game, dungeon_list: &mut Vec<Dungeon>, index: usize) {
    assert!(index < dungeon_list.len() - 1);
}

/// Creates an actor of type `name` and places it in a random open location in `dungeon`.
fn gen_actor_random_coord(game: &Game, dungeon: &mut Dungeon, name: &str) {
    let xy = dungeon.random_avail_coord_actor();
    let a = Actor::new(game, name);
    dungeon.add_actor(xy, a);
}

/// Creates the player and places him in a random location of the dungeon.
fn gen_player(game: &Game, dungeon: &mut Dungeon) {
    gen_actor_random_coord(game, dungeon, "Player");
}

/// Generates a dungeon level using the "room method".
fn gen_dungeon_room_method(dungeon: &mut Dungeon, index: usize) {
    let mut room_list: Vec<Room> = Vec::new();
    let mut object_list: Vec<(Coord, Object)> = Vec::new();
    let direction_list = vec![N, E, S, W];
    let goal_num_rooms = gen_num_rooms(index);

    // Generate the initial room
    room_list.push(Room::from_dimensions(0, 0, gen_room_width(index), gen_room_height(index)));

    // Generate rooms by looking for free space next to existing rooms
    for _ in 0..goal_num_rooms - 1 {
        loop {
            let room = room_list.choose().unwrap().clone();
            let direction = direction_list.choose().unwrap();

            // Try a few times to generate a room here
            if let Some(new_room) = try_some!(gen_room_adjacent(&room,
                                                                *direction,
                                                                &room_list,
                                                                &mut object_list,
                                                                index),
                                              3) {
                room_list.push(new_room);
                break;
            };
        }
    }

    // Initialize the dungeon tile grid
    init_dungeon_from_rooms(dungeon, &room_list);
    // Add doors
    for (coord, object) in object_list {
        dungeon.add_object(coord, object);
    }

    // Convert the list of rooms into a tile grid representation
}

/// Generates a room adjacent to `room`, or returns `None`.
///
/// # Panics
/// Panics if `direction` is not orthogonal.
fn gen_room_adjacent(room: &Room,
                     direction: Direction,
                     room_list: &[Room],
                     object_list: &mut Vec<(Coord, Object)>,
                     index: usize)
                     -> Option<Room> {
    let top: int;
    let left: int;
    let new_room: Room;
    let width = gen_room_width(index) as int;
    let height = gen_room_height(index) as int;

    match direction {
        N => {
            top = room.top - height;
            left = rand_range(room.left - width + 3, room.right - 2);
        }
        E => {
            top = rand_range(room.top - height + 3, room.bottom - 2);
            left = room.right + 1;
        }
        S => {
            top = room.bottom + 1;
            left = rand_range(room.left - width + 3, room.right - 2);
        }
        W => {
            top = rand_range(room.top - height + 3, room.bottom - 2);
            left = room.left - width;
        }
        _ => panic!("Generate::gen_room_adjacent failed: non-orthogonal direction."),
    };
    new_room = Room::from_dimensions(top, left, width as usize, height as usize);

    if check_room_free(&new_room, room_list) {
        let coord_door = gen_room_adjacent_door(room, &new_room, direction);
        object_list.push(coord_door);
        Some(new_room)
    } else {
        None
    }
}

/// Generates a door between two adjacent `Room`s in given `Direction`.
///
/// # Panics
/// Panics if `direction` is not orthogonal.
fn gen_room_adjacent_door(room: &Room, other: &Room, direction: Direction) -> (Coord, Object) {
    unimplemented!() // todo
}

/// Check if `room` does not collide with any `Room`s in `room_list`.
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
    rand_range(2, 5) // TODO
}

/// Generates a random height for a `Room` based on the dungeon level specified by `index`.
fn gen_room_height(index: usize) -> usize {
    rand_range(2, 5) // TODO
}

/// A struct for storing data for a single `Room`, used in dungeon generation.
/// Note that the four bounding boxes correspond to the `Room`'s walls.
#[derive(Clone)]
struct Room {
    left: int,
    top: int,
    right: int,
    bottom: int,
}

impl Room {
    #[allow(dead_code)]
    pub fn new(left: int, top: int, right: int, bottom: int) -> Room {
        Room {
            left: left,
            top: top,
            right: right,
            bottom: bottom,
        }
    }

    /// Returns a new `Room` created from given `width` and `height`.
    pub fn from_dimensions(left: int, top: int, width: usize, height: usize) -> Room {
        Room {
            left: left,
            top: top,
            right: left + width as int - 1,
            bottom: top + height as int - 1,
        }
    }

    /// Returns true if `self` and `other` overlap.
    pub fn overlaps(&self, other: &Self) -> bool {
        overlaps(self.left, self.right, other.left, other.right) &&
        overlaps(self.top, self.bottom, other.top, other.bottom)
    }
}

#[cfg(test)]
mod tests {
    use generate::Room;

    #[test]
    fn test_room_overlaps() {
        let rooms = vec![Room::new(0, 0, 1, 1),
                         Room::new(1, 0, 3, 3),
                         Room::new(-1, -1, 4, 4),
                         Room::new(-2, -2, -1, -1)];

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
