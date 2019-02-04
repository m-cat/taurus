pub mod actor;

mod common;

use crate::actor::Actor;
use crate::constants;
use crate::coord::Coord;
use crate::defs::GameRatio;
use crate::dungeon::Dungeon;
use crate::object::Object;
use crate::tile::Tile;
use crate::util;
use crate::DATABASE;

// Test creating all actors, objects, and tiles contained in the database.
#[test]
fn create_everything() {
    let database = DATABASE.read().unwrap();
    let coord = Coord::new(0, 0);

    let data = database.get_obj("actors").unwrap();
    for actor in data.values() {
        for _ in 0..5 {
            let _ = Actor::new(coord, &actor.get_obj().unwrap());
        }
    }

    let data = database.get_obj("objects").unwrap();
    for object in data.values() {
        for active in &[true, false] {
            for _ in 0..5 {
                let _ = Object::new(coord, &object.get_obj().unwrap(), *active);
            }
        }
    }

    let data = database.get_obj("tiles").unwrap();
    for tile in data.values() {
        for _ in 0..5 {
            let _ = Tile::new(&tile.get_obj().unwrap());
        }
    }

    let data = database.get_obj("dungeon_profiles").unwrap();
    let dungeons = database
        .get_obj("dungeons")
        .unwrap()
        .get_arr("dungeons")
        .unwrap();
    for dungeon in data.values() {
        for danger in 0..dungeons.len() {
            let _ = Dungeon::new(danger as u32, &dungeon.get_obj().unwrap());
        }
    }
}

// Tests that the game queue system is working.
#[test]
fn game_queue() {
    let mut dungeon = common::setup_dungeon().unwrap();
    let database = DATABASE.read().unwrap();
    let actor_data = database.get_obj("actors").unwrap();
    let object_data = database.get_obj("objects").unwrap();

    let coord1 = Coord::new(0, 0);
    let coord2 = Coord::new(1, 1);

    let mut a_turn = None;
    let mut o_turn = None;

    Actor::insert_new(&mut dungeon, coord1, &actor_data.get_obj("test").unwrap()).unwrap();
    Actor::insert_new(
        &mut dungeon,
        coord2,
        &actor_data.get_obj("test_slow").unwrap(),
    )
    .unwrap();
    Object::insert_new(
        &mut dungeon,
        coord1,
        &object_data.get_obj("test_slow").unwrap(),
        util::rand::dice(1, 2),
    )
    .unwrap();

    assert_eq!(dungeon.peek_object().name(), "test_slow");
    assert_eq!(dungeon.peek_object().turn(), GameRatio::new(37, 10));

    assert_eq!(dungeon.peek_actor().name(), "test");
    assert_eq!(dungeon.peek_actor().turn(), GameRatio::new(1, 1));
    let _ = dungeon.step_turn(&mut a_turn, &mut o_turn);
    assert_eq!(dungeon.peek_actor().name(), "test");
    assert_eq!(dungeon.peek_actor().turn(), GameRatio::new(2, 1));
    let _ = dungeon.step_turn(&mut a_turn, &mut o_turn);
    assert_eq!(dungeon.peek_actor().name(), "test");
    assert_eq!(dungeon.peek_actor().turn(), GameRatio::new(3, 1));
    let _ = dungeon.step_turn(&mut a_turn, &mut o_turn);
    assert_eq!(dungeon.peek_actor().name(), "test_slow");
    assert_eq!(dungeon.peek_actor().turn(), GameRatio::new(7, 2));

    let _ = dungeon.step_turn(&mut a_turn, &mut o_turn);
    assert_eq!(dungeon.peek_object().name(), "test_slow");
    assert_eq!(dungeon.peek_object().turn(), GameRatio::new(37, 10));
    let _ = dungeon.step_turn(&mut a_turn, &mut o_turn);
    assert_eq!(dungeon.peek_actor().name(), "test");
    assert_eq!(dungeon.peek_actor().turn(), GameRatio::new(4, 1));
    assert_eq!(dungeon.peek_object().name(), "test_slow");
    assert_eq!(dungeon.peek_object().turn(), GameRatio::new(37 * 2, 10));
}
