extern crate taurus;

use taurus::coord::Coord;
use taurus::actor::Actor;

mod common;

#[test]
fn test_set_actor_coord() {
    let (game, mut dungeon) = common::setup_game_test();
    let actor = Actor::new(&game, "test");

    dungeon.add_actor(0, 0, actor);
    dungeon.set_actor_coord(0, 0, 1, 1);
    dungeon.set_actor_coord(1, 1, 2, 2);
    dungeon.set_actor_coord(2, 2, 3, 3);

    // Test that there is still only one actor.
    assert_eq!(dungeon.num_actors(), 1);

    let actor = &dungeon.get_actor(3, 3).unwrap();

    // Test that the actor is located at (3,3)
    assert_eq!(actor.xy, Coord::new(3, 3));
}

#[test]
#[should_panic]
fn test_set_actor_coord_panic() {
    let (game, mut dungeon) = common::setup_game_test();
    let actor1 = Actor::new(&game, "test");
    let actor2 = Actor::new(&game, "test");

    dungeon.add_actor(0, 0, actor1);
    dungeon.add_actor(1, 1, actor2);

    // Try setting to an occupied coordinate, inducing a panic.
    dungeon.set_actor_coord(0, 0, 1, 1);
}
