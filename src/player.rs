//! Module for player-specific logic.

use GameResult;
use actor::{ActResult, Actor, Behavior};
use console::{self, CONSOLE, DrawConsole, Key};
use console::KeyCode::*;
use coord::Coord;
use dungeon::Dungeon;
use game_data::GameData;
use std::rc::Rc;
use ui;
use util::direction::CompassDirection;

/// Acts out the player's turn.
pub fn player_act(player: &mut Actor, dungeon: &mut Dungeon) -> ActResult {
    let mut end_turn = false;

    // While user input is not a game action...
    while !end_turn {
        // Check if the window was closed by the user.
        if CONSOLE.lock().unwrap().window_closed() {
            return ActResult::WindowClosed;
        } // TODO: how does window_closed work?

        // Draw the game and UI to the screen.
        ui::draw_all(dungeon);

        // Wait for user input.
        let key = CONSOLE.lock().unwrap().wait_for_keypress(true);

        // Respond to user input.
        let (result, end) = player_process_key(player, dungeon, key);
        if result != ActResult::None {
            return result;
        }

        end_turn = end;
    }

    ActResult::None
}

// Processes input key. Returns true if the player uses up a turn.
pub fn player_process_key(
    player: &mut Actor,
    dungeon: &mut Dungeon,
    key: Key,
) -> (ActResult, bool) {
    if key.code != NoKey {
        match key.code {
            Left => return player.try_move_dir(dungeon, CompassDirection::W),
            Up => return player.try_move_dir(dungeon, CompassDirection::N),
            Right => return player.try_move_dir(dungeon, CompassDirection::E),
            Down => return player.try_move_dir(dungeon, CompassDirection::S),

            Escape => return (ActResult::QuitGame, false),

            _ => (),
        }
    } else {
        match key.printable {
            _ => (),
        }
    }

    (ActResult::None, false)
}
