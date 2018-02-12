//! Module containing dungeon generation algorithms.

mod util;

use {DATABASE, GAMEDATA, GameResult};
use actor::Actor;
use coord::Coord;
use database::{Arr, Database};
use defs::*;
use dungeon::{Dungeon, DungeonList, DungeonType};
use error::{GameError, err_unexpected};
use failure::{Fail, ResultExt};
use game_data::GameData;
use generate::util::*;
use object::Object;
use player;
use std::{fmt, thread, time};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use tile::{Tile, TileInfo};
use ui::draw_game;
use util::direction::CardinalDirection;
use util::direction::CardinalDirection::*;
use util::math::{min_max, overlaps};
use util::rand::{Choose, dice, rand_int, rand_ratio};
use util::rectangle::Rectangle;

/// Generates a connected series of dungeons.
pub fn gen_dungeon_list(
    mut dungeon_list: DungeonList,
    dungeons_arr: &Arr,
    num_dungeons: usize,
) -> GameResult<DungeonList> {
    // Generate each depth.

    for n in 0..num_dungeons {
        create_dungeon(&mut dungeon_list, dungeons_arr, n)?;
    }

    // let mut thread_list = Vec::with_capacity(num_dungeons);
    // let dungeon_list = Arc::new(Mutex::new(dungeon_list));
    // let (tx, rx) = mpsc::channel();

    // for n in 0..num_dungeons {
    //     let dungeon_list = Arc::clone(&dungeon_list);
    //     let dungeons_arr = dungeons_arr.clone();
    //     let tx = tx.clone();

    //     // TODO: Limit threads to num of cores. Use a threadpool here?
    //     thread_list.push(thread::spawn(move || {
    //         let result = create_dungeon(&dungeon_list, &dungeons_arr, n);
    //         tx.send(result).unwrap();
    //     }));
    // }
    // for n in 0..num_dungeons {
    //     rx.recv()??;
    // }

    // let mut dungeon_list = Arc::try_unwrap(dungeon_list).unwrap().into_inner()?;

    // Generate stairs and pits, skipping the last depth.

    /*
    for n in 0..dungeon_list.len() - 1 {
    gen_pits(dungeon_list, n);
}
     */

    // Add player.

    let player = gen_player(&mut dungeon_list, 0)?;
    {
        let dungeon = &mut dungeon_list[0];
        dungeon.add_actor(player);
    }

    Ok(dungeon_list)
}

fn create_dungeon(
    dungeon_list: &mut DungeonList,
    dungeons_arr: &Arr,
    index: usize,
) -> GameResult<()> {
    let profile = get_dungeon_profile(dungeons_arr, index)?;
    let dungeon = Dungeon::new(index as u32, &profile).context(format!(
        "Failed to create dungeon at depth {}",
        index
    ))?;
    dungeon_list.push(dungeon);

    Ok(())
}

/// Generates a single depth of the dungeon.
pub fn gen_dungeon(mut dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    match dungeon.dungeon_type {
        DungeonType::Room => gen_dungeon_room(&mut dungeon, profile)?,
        DungeonType::Empty => gen_dungeon_empty(&mut dungeon, profile)?,
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

    let a = Actor::new(coord, actor_data).context(format!(
        "Could not load actor:\n{}",
        actor_data
    ))?;

    Ok(a)
}

/// Creates the player and places him in a random location of the dungeon.
fn gen_player(dungeon_list: &mut DungeonList, depth: usize) -> GameResult<Actor> {
    dungeon_list.current_depth = depth;

    let mut dungeon = &mut dungeon_list[depth];
    let player_data = DATABASE.read().unwrap().get_obj("player")?;

    let player = gen_actor_random_coord(dungeon, &player_data)?;

    GAMEDATA.write().unwrap().set_player(player.clone());

    player::calc_fov(&player, &mut dungeon);

    Ok(player)
}

fn get_dungeon_profile(dungeons_arr: &Arr, index: usize) -> GameResult<Database> {
    let dungeons_file = "dungeons.over";

    let arr = dungeons_arr.get(index)?.get_arr()?;

    Ok(pick_obj_from_tup_arr(&arr).context(format!(
        "Parsing \"dungeons\" Arr in \"{}\"",
        dungeons_file
    ))?)
}

#[inline]
pub fn gen_dungeon_empty(dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    dungeon.init_grid(20, 20, &profile.get_obj("wall_tile")?)?;

    Ok(())
}

#[derive(Debug)]
struct DungeonRoomParams {
    min_width: usize,
    max_width: usize,
    min_height: usize,
    max_height: usize,

    min_num_rooms: usize,
    max_num_rooms: usize,
}

/// Generates a dungeon level using the "room method".
#[inline]
pub fn gen_dungeon_room(dungeon: &mut Dungeon, profile: &Database) -> GameResult<()> {
    let mut room_list: Vec<Rectangle> = Vec::new();
    let mut object_list: Vec<Object> = Vec::new();
    let direction_list = vec![N, E, S, W];

    let params = DungeonRoomParams {
        // TODO: Load this info only once, store it in a DungeonTypeInfo struct in GAMEDATA?
        min_width: big_to_usize(profile.get_int("min_width")?)?,
        max_width: big_to_usize(profile.get_int("max_width")?)?,
        min_height: big_to_usize(profile.get_int("min_height")?)?,
        max_height: big_to_usize(profile.get_int("max_height")?)?,

        min_num_rooms: big_to_usize(profile.get_int("min_num_rooms")?)?,
        max_num_rooms: big_to_usize(profile.get_int("max_num_rooms")?)?,
    };
    let goal_num_rooms = gen_num_rooms(&params);

    // Generate the initial room.
    room_list.push(Rectangle::from_dimensions(
        0,
        0,
        gen_room_width(&params),
        gen_room_height(&params),
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
                    &params,
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
            let mut object = object.inner.lock().unwrap();
            let new_coord = object.coord() + Coord::new(dx, dy);
            object.set_coord(new_coord);
            new_coord
        };
        dungeon[coord].set_tile_info(
            &profile.get_obj("floor_tile")?, // TODO: only get this once
        )?;
        dungeon.add_object(object);
    }

    Ok(())
}

// Generates a room adjacent to `room`, or returns `None`.
#[inline]
fn gen_room_adjacent(
    dungeon: &mut Dungeon,
    room: &Rectangle,
    direction: &CardinalDirection,
    room_list: &[Rectangle],
    object_list: &mut Vec<Object>,
    profile: &Database,
    params: &DungeonRoomParams,
) -> GameResult<Option<Rectangle>> {
    let top: i32;
    let left: i32;

    let width = gen_room_width(params) as i32;
    let height = gen_room_height(params) as i32;

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
    let new_room = Rectangle::from_dimensions(left, top, width as usize, height as usize);

    if check_room_free(&new_room, room_list) {
        let door = gen_room_adjacent_door(dungeon, room, &new_room, direction, profile)?;
        object_list.push(door);
        Ok(Some(new_room))
    } else {
        Ok(None)
    }
}

// Generates a door between two adjacent rooms in given `Direction`.
#[inline]
fn gen_room_adjacent_door(
    dungeon: &mut Dungeon,
    room: &Rectangle,
    new_room: &Rectangle,
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

    // TODO: Clone a model object here.
    Ok(Object::new(coord, &door, dice(8, 10)).context(format!(
        "Could not load object:\n{}",
        door
    ))?)
}

// Checks if `room` does not collide with any rooms in `room_list`.
#[inline]
fn check_room_free(room: &Rectangle, room_list: &[Rectangle]) -> bool {
    !room_list.iter().any(|other| room.overlaps(other))
}

// Initializes `dungeon`'s dungeon grid based on the rooms in `room_list`.
#[inline]
fn init_dungeon_from_rooms(
    dungeon: &mut Dungeon,
    room_list: &[Rectangle],
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

    let width = (max_right + min_left.abs() + 1) as usize + 2;
    let height = (max_bottom + min_top.abs() + 1) as usize + 2;

    dungeon.init_grid(
        width,
        height,
        &profile.get_obj("wall_tile")?,
    )?;

    let dx = min_left.abs() + 1;
    let dy = min_top.abs() + 1;

    let floor = GAMEDATA.read().unwrap().tile_info(
        profile.get_obj("floor_tile")?.id(),
    );
    gen_init_dungeon_rooms(dungeon, &floor, room_list, dx, dy);

    Ok((dx, dy))
}

#[inline]
fn gen_init_dungeon_rooms(
    dungeon: &mut Dungeon,
    floor: &Arc<TileInfo>,
    room_list: &[Rectangle],
    dx: i32,
    dy: i32,
) {
    for room in room_list {
        for x in room.left..room.right + 1 {
            for y in room.top..room.bottom + 1 {
                dungeon[Coord::new(x + dx, y + dy)].info = Arc::clone(floor);
            }
        }
    }
}

/// Generates the number of rooms for the dungeon level specified by `index`.
#[inline]
fn gen_num_rooms(params: &DungeonRoomParams) -> usize {
    rand_int(params.min_num_rooms, params.max_num_rooms)
}

/// Generates a random width for a room based on the dungeon level specified by `index`.
#[inline]
fn gen_room_width(params: &DungeonRoomParams) -> usize {
    rand_int(params.min_width, params.max_width)
}

/// Generates a random height for a room based on the dungeon level specified by `index`.
#[inline]
fn gen_room_height(params: &DungeonRoomParams) -> usize {
    rand_int(params.min_height, params.max_height)
}
