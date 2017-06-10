//! Module containing dungeon generation algorithms.

use util::*;
use util::Direction::*;
use constants;
use coord::Coord;
use object::Object;
use actor::Actor;
use dungeon::Dungeon;
use game::Game;
use player;

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
    let mut object_list: Vec<Object> = Vec::new();
    let direction_list = vec![N, E, S, W];
    let goal_num_rooms = gen_num_rooms(index);

    // Generate the initial room
    room_list.push(Room::new(0, 0, gen_room_width(index), gen_room_height(index)));

    // Generate rooms by looking for free space next to existing rooms
    for _ in 0..goal_num_rooms - 1 {
        let mut found = false;

        while !found {
            let mut room = room_list.choose().unwrap().clone();
            let direction = direction_list.choose().unwrap();

            match gen_room_adjacent(&room, *direction, &room_list, &mut object_list) {
                Some(new_room) => {
                    room_list.push(new_room);
                    found = true;
                }
                None => {} // keep looking
            };
        }
    }

    // Initialize the dungeon tile grid
    init_dungeon_from_rooms(dungeon, &room_list);

    // Convert the list of rooms into a tile grid representation
}

/// Generates a room adjacent to `room`, or returns `None`.
fn gen_room_adjacent(room: &Room,
                     direction: Direction,
                     room_list: &Vec<Room>,
                     object_list: &mut Vec<Object>)
                     -> Option<Room> {
    None // todo
}

/// Initializes `dungeon`'s dungeon grid based on the `Room`s in `room_list`.
fn init_dungeon_from_rooms(dungeon: &mut Dungeon, room_list: &Vec<Room>) {
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
#[derive(Clone)]
struct Room {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,

    door_list: Vec<Coord>,
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Room {
        Room {
            left: x,
            top: y,
            right: x + width - 1,
            bottom: y + height - 1,

            door_list: Vec::new(),
        }
    }
}
