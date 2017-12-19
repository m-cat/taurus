//! Module containing dungeon generation algorithms.

mod room;
mod util;

use GameResult;
use actor::Actor;
use console::CONSOLE;
use coord::Coord;
use database::Database;
use defs::{GameRatio, to_gameratio};
use dungeon::{Dungeon, DungeonList, DungeonType};
use error::{GameError, err_unexpected};
use failure::{Fail, ResultExt};
use game_data::GameData;
use generate::room::Room;
use generate::util::*;
use object::Object;
use std::{fmt, thread, time};
use std::cell::RefCell;
use std::rc::Rc;
use tile::Tile;
use ui::draw_game;
use util::direction::CardinalDirection;
use util::direction::CardinalDirection::*;
use util::math::{min_max, overlaps};
use util::rand::{Choose, dice, rand_int, rand_ratio};

/// Generates a connected series of dungeons.
pub fn gen_dungeon_list(
    dungeon_list: &mut DungeonList,
    mut game_data: GameData,
    num_dungeons: usize,
) -> GameResult<()> {
    // Generate each depth.
    for n in 0..num_dungeons {
        let profile = get_dungeon_profile(n, &game_data)?;
        dungeon_list.push(Dungeon::new(&mut game_data, n as u32, &profile)?);
    }

    /*
    // Generate pits, skipping the last depth.
    for n in 0..dungeon_list.len() - 1 {
        gen_pits(dungeon_list, n);
    }
     */

    // Add player.
    let player = gen_player(dungeon_list, 0)?;
    let mut dungeon = &mut dungeon_list[0];
    dungeon.add_actor(player);

    Ok(())
}

/// Generates a single depth of the dungeon.
pub fn gen_dungeon(mut dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    match dungeon.dungeon_type {
        DungeonType::Room => gen_dungeon_room(&mut dungeon, &profile)?,
        DungeonType::Empty => gen_dungeon_empty(&mut dungeon, &profile)?,
    }

    Ok(())
}

/// Generates pits for a depth of the dungeon.
fn gen_pits(dungeon_list: &mut DungeonList, index: usize) {
    debug_assert!(index < dungeon_list.len() - 1);

    unimplemented!();
}

/// Creates an actor of type `name` and places it in a random open location in `dungeon`.
fn gen_actor_random_coord(dungeon: &Dungeon, actor_data: &Database) -> GameResult<Actor> {
    let coord = dungeon.random_open_coord_actor();

    debug_assert!(coord.is_some());

    // If we're out of squares, don't add the actor.
    let coord = match coord {
        Some(coord) => coord,
        None => return err_unexpected("Ran out of tiles for new actors"),
    };

    let a = Actor::new(dungeon.game_data(), coord, actor_data).context(
        format!(
            "Could not load actor:\n{}",
            actor_data
        ),
    )?;

    Ok(a)
}

/// Creates the player and places him in a random location of the dungeon.
fn gen_player(dungeon_list: &mut DungeonList, depth: usize) -> GameResult<Actor> {
    dungeon_list.current_depth = depth;

    let dungeon = &dungeon_list[depth];
    let player_data = dungeon_list.game_data().database().get_obj("player")?;

    let player = gen_actor_random_coord(&dungeon, &player_data)?;

    let game_data = dungeon_list.game_data();
    game_data.set_player(player.clone());

    Ok(player)
}

fn get_dungeon_profile(index: usize, game_data: &GameData) -> GameResult<Database> {
    let dungeons_file = "dungeons.over";

    let arr = game_data
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

pub fn gen_dungeon_empty(dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    dungeon.init_grid(20, 20, &profile.get_obj("wall_tile")?)?;

    Ok(())
}

/// Generates a dungeon level using the "room method".
pub fn gen_dungeon_room(dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    let game_data = dungeon.game_data().clone();
    let mut room_list: Vec<Room> = Vec::new();
    let mut object_list: Vec<Object> = Vec::new();
    let direction_list = vec![N, E, S, W];
    let goal_num_rooms = gen_num_rooms();

    // Generate the initial room.
    room_list.push(Room::from_dimensions(
        0,
        0,
        gen_room_width(),
        gen_room_height(),
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
        let coord = {
            let mut object = object.inner.borrow_mut();
            let new_coord = object.coord() + Coord::new(dx, dy);
            object.set_coord(new_coord);
            new_coord
        };
        dungeon[coord].set_tile_info(
            &game_data,
            &profile.get_obj("floor_tile")?,
        )?;
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
    object_list: &mut Vec<Object>,
    profile: &Database,
) -> GameResult<Option<Room>> {
    let top: i32;
    let left: i32;

    let width = gen_room_width() as i32;
    let height = gen_room_height() as i32;

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
) -> GameResult<Object> {
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
                    Tile::new(dungeon.game_data(), &floor).context(format!(
                        "Could not load tile:\n{}",
                        floor
                    ))?;
            }
        }
    }

    Ok((dx, dy))
}

/// Generates the number of `Room`s for the dungeon level specified by `index`.
fn gen_num_rooms() -> usize {
    20 // TODO
}

/// Generates a random width for a `Room` based on the dungeon level specified by `index`.
fn gen_room_width() -> usize {
    rand_int(2, 5) // TODO
}

/// Generates a random height for a `Room` based on the dungeon level specified by `index`.
fn gen_room_height() -> usize {
    rand_int(2, 5) // TODO
}
