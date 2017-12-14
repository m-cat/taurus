//! Module containing dungeon generation algorithms.

mod util;

use GameResult;
use actor::Actor;
use console::CONSOLE;
use coord::Coord;
use database::Database;
use defs::{GameRatio, to_gameratio};
use dungeon::{Dungeon, DungeonList};
use error::{GameError, err_unexpected};
use failure::{Fail, ResultExt};
use generate::util::*;
use object::Object;
use std::{fmt, thread, time};
use std::rc::Rc;
use tile::Tile;
use ui::draw_game;
use util::direction::CardinalDirection;
use util::direction::CardinalDirection::*;
use util::math::{min_max, overlaps};
use util::rand::{Choose, dice, rand_int, rand_ratio};

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

    let mut dungeon = &mut dungeon_list[0];
    let player = gen_player(&mut dungeon, 0)?;
    dungeon.add_actor(player);

    Ok(())
}

/// Generates a single depth of the dungeon.
fn gen_depth(dungeon_list: &mut DungeonList, index: usize) -> GameResult<()> {
    let mut dungeon = &mut dungeon_list[index];
    let profile = get_dungeon_profile(dungeon, index)?;

    gen_dungeon_room_method(&mut dungeon, index, &profile)?;
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
fn gen_actor_random_coord(dungeon: &mut Dungeon, actor_data: &Database) -> GameResult<Actor> {
    let coord = dungeon.random_open_coord_actor();

    debug_assert!(coord.is_some());

    // If we're out of squares, don't add the actor.
    let coord = match coord {
        Some(coord) => coord,
        None => return err_unexpected("Ran out of tiles for new actors"),
    };

    let a = Actor::new(dungeon.mut_game_data(), coord, actor_data)
        .context(format!("Could not load actor:\n{}", actor_data))?;

    Ok(a)
}

/// Creates the player and places him in a random location of the dungeon.
fn gen_player(dungeon: &mut Dungeon, depth: usize) -> GameResult<Actor> {
    let player_data = dungeon.game_data().database().get_obj("player")?;

    let player = gen_actor_random_coord(dungeon, &player_data)?;

    let game_data = dungeon.mut_game_data();
    game_data.set_player(player.clone());
    game_data.set_player_depth(depth);

    Ok(player)
}

fn get_dungeon_profile(dungeon: &Dungeon, index: usize) -> GameResult<Database> {
    let dungeons_file = "dungeons.over";

    let arr = dungeon
        .game_data()
        .database()
        .get_obj("dungeons")
        .context("Parsing main.dungeons")?
        .get_arr("dungeons")
        .context("Parsing main.dungeons.dungeons")?
        .get(index)?
        .get_arr()?;

    Ok(pick_obj_from_tup_arr(&arr).context(format!(
        "Parsing \"dungeons\" Arr in \"{}\"",
        dungeons_file
    ))?)
}

/// Generates a dungeon level using the "room method".
pub fn gen_dungeon_room_method(
    dungeon: &mut Dungeon,
    index: usize,
    profile: &Database,
) -> GameResult<()> {
    let mut room_list: Vec<Room> = Vec::new();
    let mut object_list: Vec<Box<Object>> = Vec::new();
    let direction_list = vec![N, E, S, W];
    let goal_num_rooms = gen_num_rooms(index);

    // Generate the initial room.
    room_list.push(Room::from_dimensions(
        0,
        0,
        gen_room_width(index),
        gen_room_height(index),
    ));

    // Generate rooms by looking for free space next to existing rooms.
    for _ in 0..goal_num_rooms - 1 {
        loop {
            let room = room_list.choose().unwrap().clone();
            let direction = direction_list.choose().unwrap();

            // Try a few times to generate a room here
            if let Some(new_room) = try_some!(
                gen_room_adjacent(
                    dungeon,
                    &room,
                    direction,
                    &room_list,
                    &mut object_list,
                    index,
                    profile,
                )?,
                3
            )
            {
                room_list.push(new_room);
                break;
            };
        }
    }

    // Initialize the dungeon tile grid and convert the list of rooms into a tile grid
    // representation.
    let (dx, dy) = init_dungeon_from_rooms(dungeon, &room_list, profile)?;

    // Update coordinates for actors, objects, and items

    // Add doors
    for mut object in object_list {
        let new_coord = object.coord() + Coord::new(dx, dy);
        object.set_coord(new_coord);
        dungeon.add_object(object);
    }

    Ok(())
}

/// Generates a room adjacent to `room`, or returns `None`.
fn gen_room_adjacent(
    dungeon: &mut Dungeon,
    room: &Room,
    direction: &CardinalDirection,
    room_list: &[Room],
    object_list: &mut Vec<Box<Object>>,
    index: usize,
    profile: &Database,
) -> GameResult<Option<Room>> {
    let top: i32;
    let left: i32;

    let width = gen_room_width(index) as i32;
    let height = gen_room_height(index) as i32;

    match *direction {
        W => {
            left = room.left - width - 1;
            top = rand_int(room.top - height + 1, room.bottom);
        }
        N => {
            left = rand_int(room.left - width + 1, room.right);
            top = room.top - height - 1;
        }
        E => {
            left = room.right + 2;
            top = rand_int(room.top - height + 1, room.bottom);
        }
        S => {
            left = rand_int(room.left - width + 1, room.right);
            top = room.bottom + 2;
        }
    };
    let new_room = Room::from_dimensions(left, top, width as usize, height as usize);

    if check_room_free(&new_room, room_list) {
        let door = gen_room_adjacent_door(dungeon, room, &new_room, direction, profile)?;
        object_list.push(door);
        Ok(Some(new_room))
    } else {
        Ok(None)
    }
}

/// Generates a door between two adjacent `Room`s in given `Direction`.
fn gen_room_adjacent_door(
    dungeon: &mut Dungeon,
    room: &Room,
    new_room: &Room,
    direction: &CardinalDirection,
    profile: &Database,
) -> GameResult<Box<Object>> {
    let x;
    let y;

    match *direction {
        W => {
            x = room.left - 1;
            y = rand_int(
                max!(room.top, new_room.top),
                min!(room.bottom, new_room.bottom),
            );
        }
        N => {
            x = rand_int(
                max!(room.left, new_room.left),
                min!(room.right, new_room.right),
            );
            y = room.top - 1;
        }
        E => {
            x = room.right + 1;
            y = rand_int(
                max!(room.top, new_room.top),
                min!(room.bottom, new_room.bottom),
            );
        }
        S => {
            x = rand_int(
                max!(room.left, new_room.left),
                min!(room.right, new_room.right),
            );
            y = room.bottom + 1;
        }
    }

    let coord = Coord::new(x, y);
    let door = pick_obj_from_tup_arr(&profile.get_arr("doors")?).context(
        "Parsing \"doors\" Arr in \"dungeon_profiles.over\"",
    )?;

    Ok(Object::new(dungeon.game_data(), coord, &door, dice(8, 10))
        .context(format!("Could not load object:\n{}", door))?)
}

/// Checks if `room` does not collide with any `Room`s in `room_list`.
fn check_room_free(room: &Room, room_list: &[Room]) -> bool {
    !room_list.iter().any(|other| room.overlaps(other))
}

/// Initializes `dungeon`'s dungeon grid based on the `Room`s in `room_list`.
fn init_dungeon_from_rooms(
    dungeon: &mut Dungeon,
    room_list: &[Room],
    profile: &Database,
) -> GameResult<(i32, i32)> {
    let (mut min_left, mut min_top, mut max_right, mut max_bottom) = (0, 0, 0, 0);

    for room in room_list {
        if room.left < min_left {
            min_left = room.left;
        }
        if room.top < min_top {
            min_top = room.top;
        }
        if room.right > max_right {
            max_right = room.right;
        }
        if room.bottom > max_bottom {
            max_bottom = room.bottom;
        }
    }

    debug_assert!(min_left <= 0 && min_top <= 0);

    let width = (max_right + min_left.abs() + 1) as usize;
    let height = (max_bottom + min_top.abs() + 1) as usize;

    dungeon.init_grid(
        width,
        height,
        &profile.get_obj("wall_tile")?,
    )?;

    let dx = min_left.abs();
    let dy = min_top.abs();

    let floor = profile.get_obj("floor_tile")?;
    for room in room_list {
        for x in room.left..room.right + 1 {
            for y in room.top..room.bottom + 1 {
                dungeon[(x + dx) as usize][(y + dy) as usize] =
                    Tile::new(dungeon.mut_game_data(), &floor).context(format!(
                        "Could not load tile:\n{}",
                        floor
                    ))?;
            }
        }
    }

    Ok((dx, dy))
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
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Room {
    /// Returns a new `Room` with given bounding boxes.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Room {
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
    pub fn from_dimensions(left: i32, top: i32, width: usize, height: usize) -> Room {
        debug_assert!(width > 0 && height > 0);

        Room {
            left,
            top,
            right: left + width as i32 - 1,
            bottom: top + height as i32 - 1,
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

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {})",
            self.left,
            self.top,
            self.right,
            self.bottom
        )
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
