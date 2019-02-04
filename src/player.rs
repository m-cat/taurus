//! Module for player-specific logic.

use crate::actor::{Actor, Behavior};
use crate::console::KeyCode::*;
use crate::console::*;
use crate::constants;
use crate::coord::Coord;
use crate::dungeon::{ActResult, Dungeon};
use crate::game_data::GameData;
use crate::ui;
use crate::util;
use crate::util::direction::CompassDirection;
use crate::{GameResult, CONSOLE};
#[cfg(feature = "dev")]
use flame;
use std::fs::File;
use std::rc::Rc;
use std::{thread, time};

/// Acts out the player's turn.
pub fn player_act(player: &mut Actor, dungeon: &mut Dungeon) -> ActResult {
    let mut end_turn = false;

    // Initialize input flags to check for.
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

            if let Some((flags, event)) = console.check_for_event(input_flags) {
                break (flags, event);
            }

            // Sleep a bit so we don't tax the CPU.
            thread::sleep(time::Duration::from_millis(1));
        };

        // Respond to user input.

        let (result, end) = player_process_event(player, dungeon, flags, event);
        if result != ActResult::None {
            return result;
        }

        end_turn = end;

        // Calculate FOV.

        calc_fov(player, dungeon);
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
                #[allow(match_same_arms)]
                match key.code {
                    Left => return player.try_move_dir(dungeon, CompassDirection::W),
                    Up => return player.try_move_dir(dungeon, CompassDirection::N),
                    Right => return player.try_move_dir(dungeon, CompassDirection::E),
                    Down => return player.try_move_dir(dungeon, CompassDirection::S),

                    Escape => return (ActResult::QuitGame, false),

                    F1 => {
                        // Dump the profiler report to disk.
                        #[cfg(feature = "dev")]
                        {
                            println!("Writing flame report to {}...", constants::FLAME_PATH);
                            flame::dump_html(&mut File::create(constants::FLAME_PATH).unwrap())
                                .unwrap();
                            println!("Done.");
                        }

                        ()
                    }

                    _ => (),
                }
            } else {
                match key.printable {
                    _ => (),
                }
            }
        }
        Event::Mouse(mouse) => {
            // Print debug info about all structures at mouse.
            #[cfg(feature = "dev")]
            {
                let view = ui::calc_game_view();

                let (mouse_x, mouse_y) = (mouse.cx, mouse.cy);
                let (game_x, game_y) = (mouse_x as i32 + view.left, mouse_y as i32 + view.top);
                let coord = Coord::new(game_x, game_y);

                if dungeon.in_bounds(coord) {
                    let tile = &dungeon[coord];

                    println!("\nTile at ({}, {}) contains:", game_x, game_y);
                    if let Some(ref actor) = tile.actor {
                        println!(" - Actor: {:#?}", actor);
                    }
                    if let Some(ref object) = tile.object {
                        println!(" - Object: {:#?}", object);
                    }
                    if let Some(ref item_stash) = tile.item_stash {
                        println!(" - Item stash: {:#?}", item_stash);
                    }
                    println!(" - Tile info: {:#?}", tile.info);
                }
            }
        }
    }

    (ActResult::None, false)
}

/// Calculates FOV around the player.
/// This should be called whenever the player moves.
/// Messages, for example, are only added if the player sees the event.
pub fn calc_fov(player: &Actor, dungeon: &mut Dungeon) {
    // TODO: Replace libtcod FOV algorithm.
    // We only need to get FOV for a small section of the dungeon.

    let inner = player.inner.lock().unwrap();
    let origin = inner.coord;
    let fov_radius = inner.fov_radius as i32;

    dungeon.calc_fov(origin, fov_radius);
}
