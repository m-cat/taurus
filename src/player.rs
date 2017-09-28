//! Module for player-specific logic.

use GameResult;
use actor::{ActResult, Actor};
use console::{Console, Key};
use console::KeyCode::*;
use coord::Coord;
use dungeon::Dungeon;
use game_data::GameData;
use ui;
use util::direction::CompassDirection;

/// Creates and returns the player actor.
pub fn player_create(game_data: &mut GameData, coord: Coord, depth: usize) -> GameResult<Actor> {
    let player = Actor::new(game_data, coord, "player")?;

    game_data.set_player_depth(depth);
    game_data.set_player_coord(player.coord());

    Ok(player)
}

/// Acts out the player's turn.
pub fn player_act(player: &mut Actor, dungeon: &mut Dungeon, console: &mut Console) -> ActResult {
    let mut end_turn = false;

    // While user input is not a game action...
    while !end_turn {
        // Check if the window was closed by the user.
        if console.window_closed() {
            return ActResult::WindowClosed;
        } // TODO: how does window_closed work?

        // Draw the game to the screen.
        ui::game_draw(dungeon, console);

        // Wait for user input.
        let key = console.wait_for_keypress(true);

        // Respond to user input.
        let (_result, end) = player_process_key(player, dungeon, key);
        end_turn = end;
    }

    ActResult::None
}

// Processes input key. Returns true if the actor has used up turn.
pub fn player_process_key(
    player: &mut Actor,
    dungeon: &mut Dungeon,
    key: Key,
) -> (ActResult, bool) {
    if key.code != NoKey {
        match key.code {
            Up => return player.try_move_dir(dungeon, CompassDirection::N),
            _ => (),
        }
    } else {
        match key.printable {
            'q' => return (ActResult::QuitGame, false),
            _ => (),
        }
    }

    (ActResult::None, false)
}
