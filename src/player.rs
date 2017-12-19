//! Module for player-specific logic.

use GameResult;
use actor::{Actor, Behavior};
use console::*;
use console::KeyCode::*;
use coord::Coord;
use dungeon::{ActResult, Dungeon};
use game_data::GameData;
use std::rc::Rc;
use ui;
use util::direction::CompassDirection;

/// Acts out the player's turn.
pub fn player_act(player: &mut Actor, dungeon: &mut Dungeon) -> ActResult {
    let mut end_turn = false;
    let mut input_flags = EventFlags::empty();
    input_flags.insert(input::KEY);
    input_flags.insert(input::MOUSE_PRESS);

    // While user input is not a game action...
    while !end_turn {
        // Draw the game and UI to the screen.
        ui::draw_all(dungeon);

        // Wait for user input.
        let console = CONSOLE.lock().unwrap();
        let (flags, event) = loop {
            // Check if the window was closed by the user.
            if console.window_closed() {
                return ActResult::WindowClosed;
            }

            match console.check_for_event(input_flags) {
                Some((flags, event)) => break (flags, event),
                None => (),
            }
        };

        // Respond to user input.
        let (result, end) = player_process_event(player, dungeon, flags, event);
        if result != ActResult::None {
            return result;
        }

        end_turn = end;
    }

    ActResult::None
}

// Processes input event. Returns true if the player uses up a turn.
pub fn player_process_event(
    player: &mut Actor,
    dungeon: &mut Dungeon,
    flags: EventFlags,
    event: Event,
) -> (ActResult, bool) {
    match event {
        Event::Key(key) => {
            if flags.contains(input::KEY_PRESS) {
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
        }
        Event::Mouse(_) => (),
    }

    (ActResult::None, false)
}
