//! Actor tests.

use DATABASE;
use actor::Actor;
use coord::Coord;
use defs::GameRatio;
use std::rc::Rc;
use tests::common;

// Test `set_actor_coord`.
// TODO: Implement the "test" actor, un-ignore this test
#[test]
fn set_actor_coord() {
    let mut dungeon = common::setup_dungeon().unwrap();
    let data = DATABASE.read().unwrap().get_obj("actors").unwrap();
    let test = data.get_obj("test").unwrap();

    let coord1 = Coord::new(0, 0);
    let coord2 = Coord::new(1, 1);
    let coord3 = Coord::new(2, 2);
    let coord4 = Coord::new(3, 3);
    let coord5 = Coord::new(4, 4);

    Actor::insert_new(&mut dungeon, coord1, &test).unwrap();
    Actor::insert_new(&mut dungeon, coord5, &test).unwrap();
    assert_eq!(dungeon.num_actors(), 2);

    dungeon.move_actor(coord1, coord2);
    dungeon.move_actor(coord2, coord3);
    dungeon.move_actor(coord3, coord4);
    dungeon.move_actor(coord4, coord1);

    // Test that the actor is located at (0,0)
    let actor = match dungeon[coord1].actor {
        Some(ref actor) => actor,
        None => panic!(),
    };
    assert_eq!(actor.coord(), coord1);
    assert_eq!(dungeon.num_actors(), 2);
}

// Test that `set_actor_coord` panics when it should.
#[test]
#[should_panic]
fn set_actor_coord_panic() {
    let mut dungeon = common::setup_dungeon().unwrap();

    let data = DATABASE.read().unwrap().get_obj("actors").unwrap();
    let test = data.get_obj("test").unwrap();

    let coord1 = Coord::new(0, 0);
    let coord2 = Coord::new(1, 1);

    Actor::insert_new(&mut dungeon, coord1, &test).unwrap();
    Actor::insert_new(&mut dungeon, coord2, &test).unwrap();

    // Try setting to an occupied coordinate, inducing a panic.
    dungeon.move_actor(coord1, coord2);
}
