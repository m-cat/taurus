//! Module for player-specific logic.

use actor::ActResult;
use actor::Actor;
use console::GameConsole;
use coord::Coord;
use dungeon::Dungeon;
use game::Game;
use ui;

/// Creates and returns the player actor.
pub fn player_create(game: &Game, dungeon: &mut Dungeon, coord: Coord) {
    Actor::insert_new(game, dungeon, coord, "player");
}

/// Acts out the player's turn.
pub fn player_act(
    player: &mut Actor,
    game: &Game,
    dungeon: &mut Dungeon,
    console: &mut GameConsole,
) -> ActResult {
    // While user input is not a game action

    // Check if the window was closed by the user.
    if console.window_closed() {
        return ActResult::WindowClosed;
    } // TODO: how does window_closed work?

    // Draw the game to the screen
    ui::game_draw(game, dungeon, console);

    // Wait for user input
    console.wait_for_keypress(true);

    // Respond to user input

    ActResult::None
}
