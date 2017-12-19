pub mod actor;

mod common;

use actor::Actor;
use constants;
use coord::Coord;
use defs::GameRatio;
use dungeon::Dungeon;
use object::Object;
use tile::Tile;
use util;

// Test creating all actors, objects, and tiles contained in the database.
#[test]
fn create_everything() {
    let mut dungeon = common::setup_dungeon().unwrap();
    let mut game_data = dungeon.game_data();
    let database = game_data.database();
    let coord = Coord::new(0, 0);

    let data = database.get_obj("actors").unwrap();
    for actor in data.values() {
        for _ in 0..5 {
            let _ = Actor::new(&mut game_data, coord, &actor.get_obj().unwrap());
        }
    }

    let data = database.get_obj("objects").unwrap();
    for object in data.values() {
        for active in &[true, false] {
            for _ in 0..5 {
                let _ = Object::new(&game_data, coord, &object.get_obj().unwrap(), *active);
            }
        }
    }

    let data = database.get_obj("tiles").unwrap();
    for tile in data.values() {
        for _ in 0..5 {
            let _ = Tile::new(&mut game_data, &tile.get_obj().unwrap());
        }
    }

    let data = database.get_obj("dungeon_profiles").unwrap();
    for dungeon in data.values() {
        for danger in 0..constants::NUM_DUNGEONS {
            let _ = Dungeon::new(&mut game_data, danger as u32, &dungeon.get_obj().unwrap());
        }
    }
}

// Tests that the game queue system is working.
#[test]
fn game_queue() {
    let mut dungeon = common::setup_dungeon().unwrap();
    let database = dungeon.game_data().database();
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
    ).unwrap();
    Object::insert_new(
        &mut dungeon,
        coord1,
        &object_data.get_obj("test_slow").unwrap(),
        util::rand::dice(1, 2),
    ).unwrap();

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
