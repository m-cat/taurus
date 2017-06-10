extern crate taurus;

use taurus::coord::Coord;
use taurus::actor::Actor;

mod common;

#[test]
fn test_set_actor_coord() {
    let (game, mut dungeon) = common::setup_game_test();
    let actor = Actor::new(&game, "test");

    let xy1 = Coord { x: 0, y: 0 };
    let xy2 = Coord { x: 1, y: 1 };
    let xy3 = Coord { x: 2, y: 2 };
    let xy4 = Coord { x: 3, y: 3 };

    dungeon.add_actor(xy1, actor);
    dungeon.set_actor_coord(xy1, xy2);
    dungeon.set_actor_coord(xy2, xy3);
    dungeon.set_actor_coord(xy3, xy4);

    // Test that there is still only one actor.
    assert_eq!(dungeon.num_actors(), 1);

    let actor = &dungeon.get_actor(xy4).unwrap();

    // Test that the actor is located at (3,3)
    assert_eq!(actor.coord(), xy4);
}

#[test]
#[should_panic]
fn test_set_actor_coord_panic() {
    let (game, mut dungeon) = common::setup_game_test();
    let actor1 = Actor::new(&game, "test");
    let actor2 = Actor::new(&game, "test");

    let xy1 = Coord { x: 0, y: 0 };
    let xy2 = Coord { x: 1, y: 1 };

    dungeon.add_actor(xy1, actor1);
    dungeon.add_actor(xy2, actor2);

    // Try setting to an occupied coordinate, inducing a panic.
    dungeon.set_actor_coord(xy1, xy2);
}
