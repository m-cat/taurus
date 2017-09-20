use actor::Actor;
use coord::Coord;
use tests::common;

// Test `set_actor_coord`.
// TODO: Implement the "test" actor, un-ignore this test
#[test]
#[ignore]
fn test_set_actor_coord() {
    let (game, mut dungeon) = common::setup_game_test().unwrap();

    let coord1 = Coord::new(0, 0);
    let coord2 = Coord::new(1, 1);
    let coord3 = Coord::new(2, 2);
    let coord4 = Coord::new(3, 3);
    let coord5 = Coord::new(4, 4);

    Actor::insert_new(&game, &mut dungeon, coord1, "test");
    Actor::insert_new(&game, &mut dungeon, coord5, "test");
    assert_eq!(dungeon.num_actors(), 2);

    dungeon.set_actor_coord(coord1, coord2);
    dungeon.set_actor_coord(coord2, coord3);
    dungeon.set_actor_coord(coord3, coord4);
    dungeon.set_actor_coord(coord4, coord1);
    assert_eq!(dungeon.num_actors(), 2);

    let actor = &dungeon.get_actor(coord4).unwrap();

    // Test that the actor is located at (3,3)
    assert_eq!(actor.coord(), coord4);
}

// Test that `set_actor_coord` panics when it should.
#[test]
#[should_panic]
fn test_set_actor_coord_panic() {
    let (game, mut dungeon) = common::setup_game_test().unwrap();

    let coord1 = Coord::new(0, 0);
    let coord2 = Coord::new(1, 1);

    Actor::insert_new(&game, &mut dungeon, coord1, "test");
    Actor::insert_new(&game, &mut dungeon, coord2, "test");

    // Try setting to an occupied coordinate, inducing a panic.
    dungeon.set_actor_coord(coord1, coord2);
}
