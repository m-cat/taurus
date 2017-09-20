use actor::Actor;
use coord::Coord;
use tests::common;

// Test `set_actor_coord`.
// TODO: Implement the "test" actor, un-ignore this test
#[test]
#[ignore]
fn test_set_actor_coord() {
    let (game, mut dungeon) = common::setup_game_test().unwrap();

    let xy1 = Coord::new(0, 0);
    let xy2 = Coord::new(1, 1);
    let xy3 = Coord::new(2, 2);
    let xy4 = Coord::new(3, 3);
    let xy5 = Coord::new(4, 4);

    Actor::insert_new(&game, &mut dungeon, xy1, "test");
    Actor::insert_new(&game, &mut dungeon, xy5, "test");
    assert_eq!(dungeon.num_actors(), 2);

    dungeon.set_actor_coord(xy1, xy2);
    dungeon.set_actor_coord(xy2, xy3);
    dungeon.set_actor_coord(xy3, xy4);
    dungeon.set_actor_coord(xy4, xy1);
    assert_eq!(dungeon.num_actors(), 2);

    let actor = &dungeon.get_actor(xy4).unwrap();

    // Test that the actor is located at (3,3)
    assert_eq!(actor.coord(), xy4);
}

// Test that `set_actor_coord` panics when it should.
#[test]
#[should_panic]
fn test_set_actor_coord_panic() {
    let (game, mut dungeon) = common::setup_game_test().unwrap();

    let xy1 = Coord::new(0, 0);
    let xy2 = Coord::new(1, 1);

    Actor::insert_new(&game, &mut dungeon, xy1, "test");
    Actor::insert_new(&game, &mut dungeon, xy2, "test");

    // Try setting to an occupied coordinate, inducing a panic.
    dungeon.set_actor_coord(xy1, xy2);
}
