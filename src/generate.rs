//! Module containing dungeon generation algorithms.

use util::*;
use constants;
use coord::Coord;
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

/// Creates the player and places him in a random location of the dungeon.
fn gen_player(game: &Game, dungeon: &mut Dungeon) {
    let player = player::player_create(game);
    add_actor_random_coord(dungeon, player);
}

/// Adds the given actor to a random available tile in the dungeon.
fn add_actor_random_coord(dungeon: &mut Dungeon, a: Actor) {
    let x = rand_range(0, dungeon.width() - 1);
    let y = rand_range(0, dungeon.height() - 1);
    dungeon.add_actor(x, y, a);
}

/// Generates a dungeon level using the "room method".
fn gen_dungeon_room_method(dungeon: &mut Dungeon, index: usize) {
    let mut room_list: Vec<Room> = Vec::new();
    let goal_num_rooms = gen_num_rooms(index);

    // Generate the initial room
    room_list.push(Room::new(0, 0, gen_room_width(index), gen_room_height(index)));

    // Generate rooms by looking for free space next to existing rooms
    for _ in 0..goal_num_rooms-1 {
        let mut found = false;

        while !found {
            let room = &room_list[rand_range(0, room_list.len()-1)];


        }
    }

    // Initialize the dungeon tile grid

    // Convert the list of rooms into a tile grid representation
}

/// Generates the number of `Room`s based on the dungeon level specified by `index`.
fn gen_num_rooms(index: usize) -> usize {
    10 + 10*index // TODO
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
